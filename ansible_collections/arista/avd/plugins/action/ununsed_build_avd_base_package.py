from pathlib import Path

import yaml
from ansible.parsing.yaml.dumper import AnsibleDumper
from ansible.plugins.action import ActionBase

CONFIG_FILE = "config.yaml"
AVD_VERSION = "3.8.0-studios0"
AVD_BASE_PACKAGE_VERSION = "3.8.0"
DEFAULT_ACTION_SCRIPT_PATH = Path(__file__).parent.parent.joinpath("plugin_utils", "studiobuilder", "default_action_scripts")
ACTIONS = [
    # (id, file, description)
    (
        "action-workspace-prebuild-avd-base",
        DEFAULT_ACTION_SCRIPT_PATH.joinpath("action-workspace-prebuild-avd-base.py"),
        "Workspace Pre-build action for AVD Studios",
    ),
]


class ActionModule(ActionBase):
    """
    Build AVD Base Package containing default actions consumed by AVD Studios.
    """

    argspec = {
        "package_dir": {
            "description": "Directory where base package should be built. Must exist and be writable",
            "type": "str",
            "required": True,
        },
    }

    def run(self, tmp=None, task_vars=None):
        if task_vars is None:
            task_vars = {}

        result = super().run(tmp, task_vars)
        del tmp  # tmp no longer has any effect

        _res, args = self.validate_argument_spec(self.argspec)

        package_dir_path = Path(args["package_dir"])

        self.action_ids = []

        self.create_package_path(package_dir_path)
        self.create_actions()
        self.create_package()

        result["package_id"] = self.package_id
        result["package_version"] = AVD_BASE_PACKAGE_VERSION
        return result

    @property
    def package_id(self) -> str:
        return "package-avd-base"

    @property
    def package_name(self) -> str:
        return "AVD Base Package"

    def create_package_path(self, package_dir_path: Path) -> None:
        self.package_path = package_dir_path.joinpath(self.package_id)
        self.package_path.mkdir(mode=0o775, exist_ok=True)

    def create_package(self) -> None:
        description = "AVD Base Package containing default actions consumed by AVD Studios"
        package_version = AVD_BASE_PACKAGE_VERSION
        package_config = {
            "id": self.package_id,
            "type": "PACKAGE",
            "name": self.package_name,
            "description": description,
            "version": package_version,
            "actions": self.action_ids,
        }
        self.package_path.joinpath(CONFIG_FILE).write_text(
            yaml.dump(package_config, indent=2, sort_keys=False, Dumper=AnsibleDumper),
            encoding="UTF-8",
        )

    def create_actions(self) -> None:
        for action in ACTIONS:
            action_id, action_script_file, description = action
            self.create_action(action_script_file, description, action_id)

    def create_action(self, action_script_file: str, description: str, action_id: str):
        action_script_path = Path(action_script_file)
        if not action_script_path.is_file():
            raise FileNotFoundError(action_script_file)

        action_name = action_script_path.name.replace(".py", "")
        action_config = {
            "type": "ACTION",
            "name": action_name,
            "description": description,
            "language": "PYTHON_3",
            "action-type": "STUDIO_AUTOFILL",
            "file": "script.py",
            "static-params": [
                {
                    "name": "AVDVersion",
                    "description": "",
                    "required": False,
                    "hidden": False,
                    "default": AVD_VERSION,
                }
            ],
            "dynamic-params": [
                {
                    "name": "StudioID",
                    "description": "",
                    "required": True,
                    "hidden": False,
                    "default": "",
                },
                {
                    "name": "InputPath",
                    "description": "",
                    "required": True,
                    "hidden": False,
                    "default": "",
                },
                {
                    "name": "WorkspaceID",
                    "description": "",
                    "required": True,
                    "hidden": False,
                    "default": "",
                },
            ],
            "args": {},
        }
        action_path = self.package_path.joinpath(action_id)
        action_path.mkdir(mode=0o775, exist_ok=True)
        action_path.joinpath(CONFIG_FILE).write_text(
            yaml.dump(action_config, indent=2, sort_keys=False, Dumper=AnsibleDumper),
            encoding="UTF-8",
        )
        action_path.joinpath("script.py").write_text(
            action_script_path.read_text(encoding="UTF-8"),
            encoding="UTF-8",
        )
        self.action_ids.append(action_id)
