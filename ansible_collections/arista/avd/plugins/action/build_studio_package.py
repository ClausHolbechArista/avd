import json
from pathlib import Path

import yaml
from ansible.parsing.yaml.dumper import AnsibleDumper
from ansible.plugins.action import ActionBase

from ansible_collections.arista.avd.plugins.plugin_utils.schema.avdschema import AvdSchema
from ansible_collections.arista.avd.plugins.plugin_utils.studiobuilder.studiobuilder import AvdStudioBuilder
from ansible_collections.arista.avd.plugins.plugin_utils.utils import get

CONFIG_FILE = "config.yaml"
AVD_VERSION = "3.8.0-studios0"
AVD_BASE_PACKAGE_NAME = "package-avd-base"
AVD_BASE_PACKAGE_VERSION = "3.8.0"
DEFAULT_ACTION_SCRIPT_PATH = Path(__file__).parent.parent.joinpath("plugin_utils", "studiobuilder", "default_action_scripts")
POST_INSTALL_ACTION_FILE = DEFAULT_ACTION_SCRIPT_PATH.joinpath("action-post-install.py")
DEFAULT_STUDIO_PREBUILD_ACTION_FILE = DEFAULT_ACTION_SCRIPT_PATH.joinpath("action-studio-prebuild.py")
DEFAULT_WORKSPACE_PREBUILD_ACTION_FILE = DEFAULT_ACTION_SCRIPT_PATH.joinpath("action-workspace-prebuild.py")
DEFAULT_STUDIO_PRERENDER_ACTION_FILE = DEFAULT_ACTION_SCRIPT_PATH.joinpath("action-studio-prerender.py")


class ActionModule(ActionBase):
    """
    Build Studio package including associated actions.
    """

    argspec = {
        "schema_id": {
            "description": "ID of AVD schema",
            "type": "str",
            "required": True,
            "valid_valued": ["eos_cli_config_gen", "eos_designs"],
        },
        "studio_design": {
            "description": "Studio Design",
            "type": "dict",
            "required": True,
        },
        "package_dir": {
            "description": "Directory where studio package should be built. Must exist and be writable",
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

        schema_id = args["schema_id"]
        studio_design = args["studio_design"]
        package_dir_path = Path(args["package_dir"])

        avdschema = AvdSchema(schema_id=schema_id)
        builder = AvdStudioBuilder(avdschema)
        self.depends_on_avd_package = True
        self.action_associations = {
            "StudioPreBuildActionIDs": [],
            "WorkspacePreBuildActionIDs": [],
            "StudioPreRenderActionIDs": [],
        }

        # build return the complete studio object. This is broken down and written into separate files
        self.studio, datamappings, tagmappings = builder.build(studio_design)

        self.update_template_tagmappings(tagmappings)
        self.create_package_path(package_dir_path)
        self.create_studio()
        self.create_actions(studio_design, datamappings, tagmappings)
        self.create_package(studio_design)

        result.update(
            {
                "package_id": self.package_id,
                "package_version": self.package_version,
            }
        )
        return result

    @property
    def studio_id(self) -> str:
        return get(self.studio, "key.studio_id", required=True)

    @property
    def studio_name(self) -> str:
        return get(self.studio, "display_name", required=True)

    @property
    def package_id(self) -> str:
        return f"package-{self.studio_id}"

    @property
    def package_name(self) -> str:
        return f"Studio Package for {self.studio_name}"

    def create_package_path(self, package_dir_path: Path) -> None:
        self.package_path = package_dir_path.joinpath(self.package_id)
        self.package_path.mkdir(mode=0o775, exist_ok=True)

    def create_package(self, studio_design) -> None:
        description = f"Package including Studio '{self.studio_name}' and associated actions"
        self.package_version = studio_design.get("package_version", "0.0.0")
        package_config = {
            "id": self.package_id,
            "type": "PACKAGE",
            "name": self.package_name,
            "description": description,
            "version": self.package_version,
            "install-hooks": {
                f"STUDIO:{self.studio_id}": {
                    "post-install": self.post_install_action_id,
                }
            },
        }
        # Not supported yet
        # if self.depends_on_avd_package:
        #     package_config.update(
        #         {
        #             "depends-on": {
        #                 AVD_BASE_PACKAGE_NAME: AVD_BASE_PACKAGE_VERSION,
        #             }
        #         }
        #     )

        self.package_path.joinpath(CONFIG_FILE).write_text(
            yaml.dump(package_config, indent=2, sort_keys=False, Dumper=AnsibleDumper),
            encoding="UTF-8",
        )

    def create_studio(self) -> None:
        studio_description = get(self.studio, "description", default="")
        studio_schema = get(self.studio, "input_schema.fields.values", required=True)
        # The API format has layout as a string containing json, so we have to load it.
        studio_layout = json.loads(get(self.studio, "input_schema.layout.value", required=True))
        studio_template = get(self.studio, "template.body", required=True)
        studio_config = {
            "type": "STUDIO",
            "name": self.studio_name,
            "description": studio_description,
            "schema": {
                "file": "schema.yaml",
            },
            "layout": {
                "file": "layout.yaml",
            },
            "template": {
                "language": "MAKO",
                "file": "template.mako",
            },
        }
        studio_path = self.package_path.joinpath(self.studio_id)
        studio_path.mkdir(mode=0o775, exist_ok=True)
        studio_path.joinpath(CONFIG_FILE).write_text(
            yaml.dump(studio_config, indent=2, sort_keys=False, Dumper=AnsibleDumper),
            encoding="UTF-8",
        )
        studio_path.joinpath("schema.yaml").write_text(
            yaml.dump(studio_schema, indent=2, sort_keys=False, Dumper=AnsibleDumper),
            encoding="UTF-8",
        )
        studio_path.joinpath("layout.yaml").write_text(
            yaml.dump(studio_layout, indent=2, sort_keys=False, Dumper=AnsibleDumper),
            encoding="UTF-8",
        )
        studio_path.joinpath("template.mako").write_text(
            studio_template,
            encoding="UTF-8",
        )

    def create_actions(self, studio_design: dict, datamappings: list, tagmappings: list) -> None:
        studio_prebuild_action_file = get(studio_design, "build_pipeline.studio_prebuild_action_file", default=DEFAULT_STUDIO_PREBUILD_ACTION_FILE)
        description = f"Studio Pre-build action for Studio {self.studio_name}"
        action_id = f"action-studio-prebuild-{self.studio_id}"
        self.create_action(studio_prebuild_action_file, description, action_id, datamappings=datamappings)
        self.action_associations["StudioPreBuildActionIDs"].append(action_id)

        workspace_prebuild_action_file = get(studio_design, "build_pipeline.workspace_prebuild_action_file", default=DEFAULT_WORKSPACE_PREBUILD_ACTION_FILE)
        self.depends_on_avd_package = False
        description = f"Workspace Pre-build action for Studio {self.studio_name}"
        action_id = f"action-workspace-prebuild-{self.studio_id}"
        self.create_action(workspace_prebuild_action_file, description, action_id, tagmappings=tagmappings)
        self.action_associations["WorkspacePreBuildActionIDs"].append(action_id)

        studio_prerender_action_file = get(studio_design, "build_pipeline.studio_prerender_action_file", default=DEFAULT_STUDIO_PRERENDER_ACTION_FILE)
        description = f"Studio Pre-render action for Studio {self.studio_name}"
        action_id = f"action-studio-prerender-{self.studio_id}"
        self.create_action(studio_prerender_action_file, description, action_id)
        self.action_associations["StudioPreRenderActionIDs"].append(action_id)

        post_install_action_file = DEFAULT_ACTION_SCRIPT_PATH.joinpath("action-post-install.py")
        description = f"Post-install action for Studio {self.studio_name}"
        # Storing in class since we need this when creating the package.
        self.post_install_action_id = f"action-post-install-{self.studio_id}"
        self.create_action(post_install_action_file, description, self.post_install_action_id, action_type="PACKAGING_INSTALL_HOOK")

    def create_action(
        self,
        action_script_file: str,
        description: str,
        action_id: str,
        action_type: str = "STUDIO_BUILD_HOOK",
        datamappings: list | None = None,
        tagmappings: list | None = None,
    ):
        action_script_path = Path(action_script_file)
        if not action_script_path.is_file():
            raise FileNotFoundError(action_script_file)

        action_name = action_script_path.name.replace(".py", "")
        action_config = {
            "type": "ACTION",
            "name": action_name,
            "description": description,
            "language": "PYTHON_3",
            "action-type": action_type,
            "file": "script.py",
            "args": {},
            "static-params": [],
            "dynamic-params": [
                {
                    "name": "StudioID",
                    "description": "",
                    "required": False,
                    "hidden": False,
                    "default": "",
                },
                {
                    "name": "WorkspaceID",
                    "description": "",
                    "required": False,
                    "hidden": False,
                    "default": "",
                },
            ],
        }
        if action_type == "STUDIO_BUILD_HOOK":
            action_config["static-params"].append(
                {
                    "name": "StudioIDs",
                    "description": "",
                    "required": False,
                    "hidden": False,
                    "default": "",
                },
                {
                    "name": "AVDVersion",
                    "description": "",
                    "required": False,
                    "hidden": False,
                    "default": AVD_VERSION,
                },
            )
        elif action_type == "PACKAGING_INSTALL_HOOK":
            action_config["static-params"].extend(
                [
                    {
                        "name": "StudioPreBuildActionIDs",
                        "description": "",
                        "required": False,
                        "hidden": True,
                        "default": ",".join(self.action_associations["StudioPreBuildActionIDs"]),
                    },
                    {
                        "name": "WorkspacePreBuildActionIDs",
                        "description": "",
                        "required": False,
                        "hidden": True,
                        "default": ",".join(self.action_associations["WorkspacePreBuildActionIDs"]),
                    },
                    {
                        "name": "StudioPreRenderActionIDs",
                        "description": "",
                        "required": False,
                        "hidden": True,
                        "default": ",".join(self.action_associations["StudioPreRenderActionIDs"]),
                    },
                ]
            )

        action_path = self.package_path.joinpath(action_id)
        action_path.mkdir(mode=0o775, exist_ok=True)
        action_path.joinpath(CONFIG_FILE).write_text(
            yaml.dump(action_config, indent=2, sort_keys=False, Dumper=AnsibleDumper),
            encoding="UTF-8",
        )

        script = action_script_path.read_text(encoding="UTF-8")
        if datamappings is not None:
            # Replace INPUTMAPPINGS = [] with INPUTMAPPINGS = <the list of datamappings passed to this function> in the script
            script = script.replace("INPUTMAPPINGS = []", f"INPUTMAPPINGS = {json.dumps(datamappings)}")

        if tagmappings is not None:
            # Replace TAGMAPPINGS = [] with INPUTMAPPINGS = <the list of tagmappings passed to this function> in the script
            script = script.replace("TAGMAPPINGS = []", f"TAGMAPPINGS = {json.dumps(tagmappings)}")

        # Write script
        action_path.joinpath("script.py").write_text(
            script,
            encoding="UTF-8",
        )

    def update_template_tagmappings(self, tagmappings):
        # Replace TAGMAPPINGS = [] with TAGMAPPINGS = <the list of tagmappings passed to this function> in the script
        template_body: str = get(self.studio, "template.body", required=True)
        self.studio["template"]["body"] = template_body.replace("TAGMAPPINGS = []", f"TAGMAPPINGS = {json.dumps(tagmappings)}")
