<!--
  ~ Copyright (c) 2026 Arista Networks, Inc.
  ~ Use of this source code is governed by the Apache License 2.0
  ~ that can be found in the LICENSE file.
  -->
=== "Table"

    | Variable | Type | Required | Default | Value Restrictions | Description |
    | -------- | ---- | -------- | ------- | ------------------ | ----------- |
    | [<samp>dns_settings_profile</samp>](## "dns_settings_profile") | String |  |  |  | Reference to a defined DNS settings profile. It allows reusing DNS settings across multiple devices. |

=== "YAML"

    ```yaml
    # Reference to a defined DNS settings profile. It allows reusing DNS settings across multiple devices.
    dns_settings_profile: <str>
    ```
