from __future__ import absolute_import, division, print_function

__metaclass__ = type

import logging

from ansible import constants as C
from ansible.errors import AnsibleAssertionError
from ansible.executor.task_result import TaskResult
from ansible.inventory.host import Host

try:
    from ansible.utils.display import color_to_log_level
except ImportError:

    def color_to_log_level(color):
        return logging.ERROR if color == C.COLOR_ERROR else logging.INFO


import rich.console
import rich.progress
import rich.theme
from ansible.playbook.task import Task
from ansible.plugins.callback import CallbackBase
from ansible.utils.display import Display, logger

DOCUMENTATION = """
  callback: arista.avd.progressbars
  callback_type: aggregate
  requirements:
    - Set as stdout in config
    - Rich Python library
  short_description: rich Ansible screen output
  description:
    - Displays ansible screen out using Rich
  extends_documentation_fragment:
    - default_callback
"""


class Console(rich.console.Console):
    _printed = False

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

    def print(self, *args, **kwargs):
        super().print(*args, **kwargs)
        self._printed = True

    def banner(self, *args, **kwargs):
        if self._printed:
            self.line(1)
        self.print(*args, **kwargs)


class RichDisplay(Display):
    def __init__(self, verbosity=0):
        super().__init__(verbosity=verbosity)
        theme = rich.theme.Theme(
            {
                "banner": "bold",
                "banner.prefix": "bold underline on dark_blue",
                "banner.time": "dim",
                "banner.timedelta": "dim",
                "banner.title": "bold",
                "ok": self.color_to_style(C.COLOR_OK),
                "changed": self.color_to_style(C.COLOR_CHANGED),
                "unreachable": self.color_to_style(C.COLOR_UNREACHABLE),
                "failed": self.color_to_style(C.COLOR_ERROR),
                "skipped": self.color_to_style(C.COLOR_SKIP),
                "rescued": self.color_to_style(C.COLOR_OK),
                "ignored": self.color_to_style(C.COLOR_WARN),
                "table.header": "default",
                "table.footer": "default",
            }
        )
        self.console = Console(
            emoji=False,
            highlight=False,
            markup=False,  # Don't interpret [markup]foo[/markup] by default
            theme=theme,
        )
        self.error_console = Console(
            emoji=False,
            highlight=False,
            markup=False,  # Don't interpret [markup]foo[/markup] by default
            theme=theme,
            stderr=True,
        )

    @staticmethod
    def color_to_style(color):
        """Convert an Ansible color spec to a rich style."""
        if color is None:
            return color
        return color.replace("bright", "bold")

    def display(
        self,
        msg,
        color=None,
        stderr=False,
        screen_only=False,
        log_only=False,
        newline=True,
        **kwargs,
    ):
        style = self.color_to_style(color)
        console = self.error_console if stderr else self.console
        end = "\n" if newline and not msg.endswith("\n") else ""

        if not log_only:
            if style:
                msg_escaped = msg.replace("[", r"\[")
                msg_markup = f"[{style}]{msg_escaped}[/{style}]"
                console.print(
                    msg_markup,
                    markup=True,
                    end=end,
                    crop=False,
                    soft_wrap=True,
                    **kwargs,
                )
            else:
                console.print(msg, end=end, crop=False, soft_wrap=True, **kwargs)

        if logger and not screen_only:
            log_level = logging.INFO
            if color:
                try:
                    log_level = color_to_log_level[color]
                except KeyError as e:
                    # this should not happen, but JIC
                    raise AnsibleAssertionError(f"Invalid color supplied to display: {color}") from e
            logger.log(log_level, msg)

    def banner(self, msg, color=None):
        style = "banner" if color is None else self.color_to_style(color)
        self.console.banner(msg, style=style)


class CallbackModule(CallbackBase):
    CALLBACK_NAME = "arista.avd.progressbars"
    CALLBACK_TYPE = "aggregate"
    CALLBACK_VERSION = 2.0

    def __init__(self):
        super().__init__()
        self._display = RichDisplay()
        self._progress = rich.progress.Progress(
            rich.progress.BarColumn(),
            "{task.completed}/{task.total}",
            rich.progress.TimeElapsedColumn(),
            "[{task.description}]",
            console=self._display.console,
            auto_refresh=False,
        )
        self._progress_tasks = {}
        self._total_progress_task = self._progress.add_task("Total", total=0)
        # Adding one extra "fake" step to total task, to avoid stopping the time between regular tasks.
        self._task_totals = {"total": 1}
        self._task_names = {}

    def _get_progress_task_id(self, host: Host, task: Task) -> rich.progress.TaskID:
        description = self._task_names.get(task._uuid, task.name)
        if task._uuid not in self._progress_tasks:
            self._progress_tasks[task._uuid] = self._progress.add_task(description, total=0)
            self._task_totals[task._uuid] = 0

        return self._progress_tasks[task._uuid]

    def _start_task(self, host: Host, task: Task):
        progress_task_id = self._get_progress_task_id(host, task)

        # Increase total with one for this task
        self._task_totals[task._uuid] = self._task_totals[task._uuid] + 1
        self._progress.update(progress_task_id, total=self._task_totals[task._uuid], refresh=True)

        # Increase total with one for total task
        self._task_totals["total"] = self._task_totals["total"] + 1
        self._progress.update(self._total_progress_task, total=self._task_totals["total"], refresh=True)

    def _advance_task(self, host: Host, task: Task):
        progress_task_id = self._get_progress_task_id(host, task)

        # Increase completed with one for this task
        self._progress.update(progress_task_id, advance=1, refresh=True)

        # Increase completed for total task
        self._progress.update(self._total_progress_task, advance=1, refresh=True)

    def _complete_all_tasks(self):
        # Complete the total task with the extra "fake" step we inserted.
        self._progress.update(self._total_progress_task, advance=1, refresh=True)
        self._progress.stop()

    def v2_playbook_on_start(self, playbook):
        self._progress.start()

    def v2_playbook_on_task_start(self, task: Task, is_conditional=False):
        # Store name of task, since this is the only place where the task name is templated for us.
        self._task_names[task._uuid] = task.get_name(True)

    def v2_runner_on_start(self, host: Host, task: Task):
        self._start_task(host, task)

    def v2_runner_on_ok(self, result: TaskResult):
        self._advance_task(result._host, result._task)

    def v2_runner_on_failed(self, result, ignore_errors=False):
        self._advance_task(result._host, result._task)

    def v2_runner_on_skipped(self, result):
        self._advance_task(result._host, result._task)

    def v2_playbook_on_stats(self, stats):
        self._complete_all_tasks()
