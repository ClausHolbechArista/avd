# Copyright (c) 2024 Arista Networks, Inc.
# Use of this source code is governed by the Apache License 2.0
# that can be found in the LICENSE file.
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# sources: arista/subscriptions/subscriptions.proto
# plugin: python-aristaproto
# This file has been @generated

from dataclasses import dataclass

import aristaproto


class Operation(aristaproto.Enum):
    UNSPECIFIED = 0
    INITIAL = 10
    """
    INITIAL indicates the associated notification is that of the
     current state and a fully-specified Resource is provided.
    """

    INITIAL_SYNC_COMPLETE = 11
    """
    INITIAL_SYNC_COMPLETE indicates all existing-state has been
     streamed to the client. This status will be sent in an
     otherwise-empty message and no subsequent INITIAL messages
     should be expected.
    """

    UPDATED = 20
    """
    UPDATED indicates the associated notification carries
     modification to the last-streamed state. This indicates
     the contained Resource may be a partial diff, though, it
     may contain a fully-specified Resource.
    """

    DELETED = 30
    """
    DETLETED indicates the associated notification carries
     a deletion. The Resource's key will always be set in this case,
     but no other fields should be expected.
    """
