# arista.avd.batch_template

Render Jinja2 template on multiple items and write result to individual files\.

## Synopsis

Render Jinja2 template on multiple items and write result to individual files\.

Destination file mode is hardcoded to 0o664\.

## Parameters

| Argument | Type | Required | Default | Value Restrictions | Description |
| -------- | ---- | -------- | ------- | ------------------ | ----------- |
| template | str | True | None |  | Path to Jinja2 Template file |
| dest_format_string | str | True | None |  | Format string used to specify target file for each item\. \'item\' is the current item from \'items\'\. Like \"mypath/\{item\}\.md\" |
| items | list | True | None |  | List of strings\. Each list item is passed to \'dest\_format\_string\' as \'item\' and passed to templater as \'item\' |

## Examples

```yaml
- name: Output eos_cli_config_gen Documentation
  tags: [eos_cli_config_gen]
  delegate_to: localhost
  run_once: true
  arista.avd.batch_template:
    template: avd_schema_documentation.j2
    dest_format_str: "{{ role_documentation_dir }}/{item}.md"
    items: "{{ documentation_schema | list }}"
  vars:
    documentation_schema: "{{ role_name | arista.avd.convert_schema(type='documentation') }}"
```

## Status

## Authors

- Arista Ansible Team (@aristanetworks)