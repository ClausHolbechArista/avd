# Copyright (c) 2024 Arista Networks, Inc.
# Use of this source code is governed by the Apache License 2.0
# that can be found in the LICENSE file.
from jinja2.runtime import Undefined


def default(primary_value, *default_values):
    """
    default will test value if defined and is not none.

    Arista.avd.default will test value if defined and is not none. If true
    return value else test default_value1.
    Test of default_value1 if defined and is not none. If true return
    default_value1 else test default_value2.
    If we run out of default values we return none.

    Example
    -------
    priority: {{ spanning_tree_priority | arista.avd.default("32768") }}

    Parameters
    ----------
    primary_value : any
        Ansible default value to look for

    Returns
    -------
    any
        Default value
    """
    if isinstance(primary_value, Undefined) or primary_value is None:
        # Invalid value - try defaults
        if len(default_values) >= 1:
            # Return the result of another loop
            return default(default_values[0], *default_values[1:])
        else:
            # Return None since no valid default values are found.
            return None
    else:
        # Return the valid value
        return primary_value
