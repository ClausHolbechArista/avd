# Copyright (c) 2024 Arista Networks, Inc.
# Use of this source code is governed by the Apache License 2.0
# that can be found in the LICENSE file.
from __future__ import annotations

from abc import ABC, abstractmethod
from dataclasses import dataclass

# Dummy placeholder for future types.
DeviceHostname = str
DeviceInputs = object
FeatureFacts = object
IntrastructureLink = object
StructuredConfig = object


class FeatureFactsFactory(ABC):
    """
    Abstract Class used for generating facts for one feature for one device.

    A subclass instance is created for every device.

    The facts are used for sharing information in a between devices, but since the instances are created in different processes
    in parallel, they have to both handle the situation where we know the peers up front, and the ones were we learn in a later
    build step.

    ### Example for a leaf-spine topology

    In AVD the uplinks of a leaf switch is defined under the leaf.
    This means the spine will need information coming from the leaf to be able to generate the downlink towards the leaf.
    The leaf will also need a little bit of information from the spine like a BGP AS number.

    The leaf holds the information about which spines it is uplinked to, so it can generate specific facts for each spine.
    The list of spines is retrieved using the `get_peer_devices` method on the leaf instance. The facts for each spine is then
    retrieved using the `get_facts_for_peer_device` method on the leaf instance.
    These facts are later inserted into the `FeatureFactsCollection` used during initialization of the
    `FeatureStructuredConfigFactory` for the spine.

    Since the spine does not know up front which leaf switches it might be connected to, it has to provide some common facts that
    the leaf switches can consume. These facts are retrieved with the `get_facts` method on the spine instance.
    These spine facts are later inserted into the `FeatureFactsCollection` used during initialization of the
    `FeatureStructuredConfigFactory` for the leaf.

    """

    device: DeviceHostname
    device_inputs: DeviceInputs

    def __init__(self, device: DeviceHostname, device_inputs: DeviceInputs) -> None:
        self.device = device
        self.device_inputs = device_inputs

    @abstractmethod
    def get_facts(self) -> FeatureFacts:
        """
        The method is called once to get common facts to be shared with any other device peering to this device.

        The Facts here are used by other devices, but we do not know which up front.
        """

    @abstractmethod
    def get_peer_devices(self) -> list[DeviceHostname]:
        """
        The method is called once to get a list of devices which would need specific facts from this device.

        Returns:
            List of device hostnames this device is peering with and has specific facts for.
        """

    @abstractmethod
    def get_facts_for_peer_device(self, peer_device: DeviceHostname) -> FeatureFacts:
        """
        The method is called once for every 'peer_device' returned from 'get_peer_devices' to get the specific facts for that peer.

        Args:
            peer_device: Hostname of the peer device.

        Returns:
            Instance of subclass of FeatureFacts holding the facts for this peer.
        """


@dataclass
class FeatureFactsCollection(ABC):
    """
    Collection of FeatureFacts for use by one device
    """

    facts_from_known_peers: dict[DeviceHostname, FeatureFacts]
    """
    FeatureFacts returned from `get_facts` on each peer's instance of FeatureFactsFactory

    ### Example for a leaf-spine topology

    For the leaf switch this would hold FeatureFacts from every spine it is uplinked to.
    """
    facts_from_unkown_peers: dict[DeviceHostname, FeatureFacts]
    """
    FeatureFacts returned from `get_facts_for_peer_device(this_device)` on other instances of FeatureFactsFactory giving us as peer

    ### Example for a leaf-spine topology

    For the spine switch this would hold FeatureFacts from every leaf uplinked to it.
    """


class FeatureStructuredConfigFactory(ABC):
    device: DeviceHostname
    device_inputs: DeviceInputs
    facts: FeatureFactsCollection

    def __init__(self, device: DeviceHostname, device_inputs: DeviceInputs, facts: FeatureFactsCollection) -> None:
        self.device = device
        self.device_inputs = device_inputs
        self.facts = facts

    def get_structured_config(self) -> StructuredConfig | None:
        """
        Called once to get structured config for this feature for this device.

        returns:
            Structured config or `None` if this no config is required for this feature.
        """
        return None

    def get_structured_config_for_infrastructure_link(self, infrastructure_link: IntrastructureLink) -> StructuredConfig | None:
        """
        Called once per infrastructure link (link between network devices) to get structured config for this feature for this device.

        args:
            link: The internal IntrastructureLink data model generated by `IntrastructureLinkFactory`.

        returns:
            Structured config or `None` if this no config is required for this feature.
        """
        return None

    def get_structured_config_for_network_services_vrf(self, tenant: DeviceInputs.Tenant, vrf: DeviceInputs.Tenant.Vrf) -> StructuredConfig | None:
        """
        Called once per Network Services VRF to get structured config for this feature for this device.

        args:
            tenant: The Tenant data model fully resolved for inheritance and profiles.
            vrf: The VRF data model fully resolved for inheritance and profiles.

        returns:
            Structured config or `None` if this no config is required for this feature.
        """
        return None

    def get_structured_config_for_network_services_svi(
        self, tenant: DeviceInputs.Tenant, vrf: DeviceInputs.Tenant.Vrf, svi: DeviceInputs.Tenant.Vrf.Svi
    ) -> StructuredConfig | None:
        """
        Called once per Network Services SVI to get structured config for this feature for this device.

        args:
            tenant: The Tenant data model fully resolved for inheritance and profiles.
            vrf: The VRF data model fully resolved for inheritance and profiles.
            svi: The SVI data model fully resolved for inheritance and profiles.

        returns:
            Structured config or `None` if this no config is required for this feature.
        """
        return None

    def get_structured_config_for_network_services_l2vlan(self, tenant: DeviceInputs.Tenant, l2vlan: DeviceInputs.Tenant.L2vlan) -> StructuredConfig | None:
        """
        Called once per Network Services L2VLAN to get structured config for this feature for this device.

        args:
            tenant: The Tenant data model fully resolved for inheritance and profiles.
            l2vlan: The L2VLAN data model fully resolved for inheritance and profiles.

        returns:
            Structured config or `None` if this no config is required for this feature.
        """
        return None

    def get_structured_config_for_connected_endpoint(self, connected_endpoint: DeviceInputs.ConnectedEndpoint) -> StructuredConfig | None:
        """
        Called once per Connected Endpoint to get structured config for this feature for this device.

        args:
            connected_endpoint: The ConnectedEndpoint data model fully resolved for inheritance and profiles.

        returns:
            Structured config or `None` if this no config is required for this feature.
        """
        return None
