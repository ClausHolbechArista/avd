<!--
  ~ Copyright (c) 2023 Arista Networks, Inc.
  ~ Use of this source code is governed by the Apache License 2.0
  ~ that can be found in the LICENSE file.
  -->
=== "Table"

    | Variable | Type | Required | Default | Value Restrictions | Description |
    | -------- | ---- | -------- | ------- | ------------------ | ----------- |
    | [<samp>fabric_ip_addressing</samp>](## "fabric_ip_addressing") | Dictionary |  |  |  | Configuration of the builtin IP addressing logic. These settings may not apply when using custom templates. |
    | [<samp>&nbsp;&nbsp;mlag</samp>](## "fabric_ip_addressing.mlag") | Dictionary |  |  |  |  |
    | [<samp>&nbsp;&nbsp;&nbsp;&nbsp;algorithm</samp>](## "fabric_ip_addressing.mlag.algorithm") | String |  | `first_id` | Valid Values:<br>- first_id<br>- odd_id<br>- same_subnet | This variable defines the Multi-chassis Link Aggregation (MLAG) algorithm used.<br>Each MLAG link will have a /31 subnet with each subnet allocated from the relevant MLAG pool via a calculated offset.<br>The offset is calculated using one of the following algorithms:<br>  - first_id: `(mlag_primary_id - 1) * 2` where `mlag_primary_id` is the ID of the first node defined under the node_group.<br>    This allocation method will skip every other /31 subnet making it less space efficient than `odd_id`.<br>  - odd_id: `(odd_id - 1) / 2`. Requires the node_group to have a node with an odd ID and a node with an even ID.<br>  - same_subnet: the offset will always be zero.<br>    This allocation method will cause every MLAG link to be addressed with the same /31 subnet. |
    | [<samp>&nbsp;&nbsp;inband_mgmt_ip</samp>](## "fabric_ip_addressing.inband_mgmt_ip") | Dictionary |  |  |  | Automatic assigment of `inband_mgmt_ip` unless it is statically configured under the node settings.<br><br>For L2 devices, the `inband_mgmt_ip` will be automatically assigned from either the configured `inband_mgmt_subnet` or<br>the subnet of the VLAN set with `inband_mgmt_vlan`. No IP will be assigned if no subnet is found.<br><br>L3 devices are not supported for automatic assignment of `inband_mgmt_ip`. |
    | [<samp>&nbsp;&nbsp;&nbsp;&nbsp;algorithm</samp>](## "fabric_ip_addressing.inband_mgmt_ip.algorithm") | String |  | `id` | Valid Values:<br>- id<br>- pool_manager | The IP address assignment is calculated using one of the following algorithms:<br>- id: Offset into the subnet with `node_id + <ips_reserved_for_gateways>`.<br>- pool_manager: Activate the pool manager.<br><br>  The pools are dynamically built and matched on the following data:<br>  - Subnet from either the configured `inband_mgmt_subnet` or the subnet of the VLAN set with `inband_mgmt_vlan`<br>  - `inband_mgmt_vrf`<br>  - The VRF under which the `inband_mgmt_vlan` is configured.<br><br>  Note: This means changing any of these fields may renumber the devices!<br><br>  Each pool will assign the first available IP starting from `1 + <ips_reserved_for_gateways>`.<br><br>  Stale entries will be reclaimed from each pool automatically after every run.<br>  A stale entry is an entry that was not accessed during the run. |
    | [<samp>&nbsp;&nbsp;&nbsp;&nbsp;ips_reserved_for_gateways</samp>](## "fabric_ip_addressing.inband_mgmt_ip.ips_reserved_for_gateways") | Integer |  | `3` |  |  |
    | [<samp>&nbsp;&nbsp;pools_file</samp>](## "fabric_ip_addressing.pools_file") | String |  |  |  | Path to file to use for storing IP pool data when using "pool_manager" as algorithm.<br>By default the path is "intended/data/<fabric_name>-ips.yml".<br><br>Note: Since the pool manager will remove stale entries after every run, each fabric should be using it's own file. |

=== "YAML"

    ```yaml
    fabric_ip_addressing:
      mlag:
        algorithm: <str>
      inband_mgmt_ip:
        algorithm: <str>
        ips_reserved_for_gateways: <int>
      pools_file: <str>
    ```
