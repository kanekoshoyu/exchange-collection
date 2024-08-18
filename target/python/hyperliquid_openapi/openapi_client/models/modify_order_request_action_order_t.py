# coding: utf-8

"""
    Hyperliquid API

    Documentation for the Hyperliquid public API     ## **Rate limits** The following rate limits apply per IP address:   - All REST requests have a weight limit of 1200 per minute. All documented exchange API requests have a weight of 1. All documented info API requests have a weight of either 2 or 20; these limits can be found in the description for each info request in the Info endpoint section. All explorer API requests have a weight of 40.   - Maximum of 100 websocket connections   - Maximum of 1000 websocket subscriptions   - Maximum of 10 unique users across user-specific websocket subscriptions  - Maximum of 2000 inbound messages per minute across all websocket connections   - Use websockets for lowest latency realtime data. See the python SDK for a full-featured example.    ## **Address-based L1 Rate limits**    The L1 rate limiting logic will allow 1 requests per 1 USDC traded cumulatively since address inception.   Using an order value of 100 USDC, this only requires a fill rate of 1%.    Each address starts with an initial buffer of 10000 requests. When rate limited, an address will still be allowed one request every 10 seconds.  Cancels have cumulative limit min(limit + 100000, limit * 2) where limit is the default limit for other actions. This way, hitting the address-based rate limit will still allow open orders to be canceled.   Note that this rate limit only applies to L1 actions, not info requests.   ## **Batched Requests** A batched request with n orders (or cancels) is treated as one request for IP based rate limiting, but as n requests for address-based L1 rate limiting. 

    The version of the OpenAPI document: 1.0.1
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


from __future__ import annotations
import json
import pprint
from pydantic import BaseModel, ConfigDict, Field, StrictStr, ValidationError, field_validator
from typing import Any, List, Optional
from openapi_client.models.modify_order_request_action_order_t_one_of import ModifyOrderRequestActionOrderTOneOf
from openapi_client.models.modify_order_request_action_order_t_one_of1 import ModifyOrderRequestActionOrderTOneOf1
from pydantic import StrictStr, Field
from typing import Union, List, Set, Optional, Dict
from typing_extensions import Literal, Self

MODIFYORDERREQUESTACTIONORDERT_ONE_OF_SCHEMAS = ["ModifyOrderRequestActionOrderTOneOf", "ModifyOrderRequestActionOrderTOneOf1"]

class ModifyOrderRequestActionOrderT(BaseModel):
    """
    Order type details.
    """
    # data type: ModifyOrderRequestActionOrderTOneOf
    oneof_schema_1_validator: Optional[ModifyOrderRequestActionOrderTOneOf] = None
    # data type: ModifyOrderRequestActionOrderTOneOf1
    oneof_schema_2_validator: Optional[ModifyOrderRequestActionOrderTOneOf1] = None
    actual_instance: Optional[Union[ModifyOrderRequestActionOrderTOneOf, ModifyOrderRequestActionOrderTOneOf1]] = None
    one_of_schemas: Set[str] = { "ModifyOrderRequestActionOrderTOneOf", "ModifyOrderRequestActionOrderTOneOf1" }

    model_config = ConfigDict(
        validate_assignment=True,
        protected_namespaces=(),
    )


    def __init__(self, *args, **kwargs) -> None:
        if args:
            if len(args) > 1:
                raise ValueError("If a position argument is used, only 1 is allowed to set `actual_instance`")
            if kwargs:
                raise ValueError("If a position argument is used, keyword arguments cannot be used.")
            super().__init__(actual_instance=args[0])
        else:
            super().__init__(**kwargs)

    @field_validator('actual_instance')
    def actual_instance_must_validate_oneof(cls, v):
        instance = ModifyOrderRequestActionOrderT.model_construct()
        error_messages = []
        match = 0
        # validate data type: ModifyOrderRequestActionOrderTOneOf
        if not isinstance(v, ModifyOrderRequestActionOrderTOneOf):
            error_messages.append(f"Error! Input type `{type(v)}` is not `ModifyOrderRequestActionOrderTOneOf`")
        else:
            match += 1
        # validate data type: ModifyOrderRequestActionOrderTOneOf1
        if not isinstance(v, ModifyOrderRequestActionOrderTOneOf1):
            error_messages.append(f"Error! Input type `{type(v)}` is not `ModifyOrderRequestActionOrderTOneOf1`")
        else:
            match += 1
        if match > 1:
            # more than 1 match
            raise ValueError("Multiple matches found when setting `actual_instance` in ModifyOrderRequestActionOrderT with oneOf schemas: ModifyOrderRequestActionOrderTOneOf, ModifyOrderRequestActionOrderTOneOf1. Details: " + ", ".join(error_messages))
        elif match == 0:
            # no match
            raise ValueError("No match found when setting `actual_instance` in ModifyOrderRequestActionOrderT with oneOf schemas: ModifyOrderRequestActionOrderTOneOf, ModifyOrderRequestActionOrderTOneOf1. Details: " + ", ".join(error_messages))
        else:
            return v

    @classmethod
    def from_dict(cls, obj: Union[str, Dict[str, Any]]) -> Self:
        return cls.from_json(json.dumps(obj))

    @classmethod
    def from_json(cls, json_str: str) -> Self:
        """Returns the object represented by the json string"""
        instance = cls.model_construct()
        error_messages = []
        match = 0

        # deserialize data into ModifyOrderRequestActionOrderTOneOf
        try:
            instance.actual_instance = ModifyOrderRequestActionOrderTOneOf.from_json(json_str)
            match += 1
        except (ValidationError, ValueError) as e:
            error_messages.append(str(e))
        # deserialize data into ModifyOrderRequestActionOrderTOneOf1
        try:
            instance.actual_instance = ModifyOrderRequestActionOrderTOneOf1.from_json(json_str)
            match += 1
        except (ValidationError, ValueError) as e:
            error_messages.append(str(e))

        if match > 1:
            # more than 1 match
            raise ValueError("Multiple matches found when deserializing the JSON string into ModifyOrderRequestActionOrderT with oneOf schemas: ModifyOrderRequestActionOrderTOneOf, ModifyOrderRequestActionOrderTOneOf1. Details: " + ", ".join(error_messages))
        elif match == 0:
            # no match
            raise ValueError("No match found when deserializing the JSON string into ModifyOrderRequestActionOrderT with oneOf schemas: ModifyOrderRequestActionOrderTOneOf, ModifyOrderRequestActionOrderTOneOf1. Details: " + ", ".join(error_messages))
        else:
            return instance

    def to_json(self) -> str:
        """Returns the JSON representation of the actual instance"""
        if self.actual_instance is None:
            return "null"

        if hasattr(self.actual_instance, "to_json") and callable(self.actual_instance.to_json):
            return self.actual_instance.to_json()
        else:
            return json.dumps(self.actual_instance)

    def to_dict(self) -> Optional[Union[Dict[str, Any], ModifyOrderRequestActionOrderTOneOf, ModifyOrderRequestActionOrderTOneOf1]]:
        """Returns the dict representation of the actual instance"""
        if self.actual_instance is None:
            return None

        if hasattr(self.actual_instance, "to_dict") and callable(self.actual_instance.to_dict):
            return self.actual_instance.to_dict()
        else:
            # primitive type
            return self.actual_instance

    def to_str(self) -> str:
        """Returns the string representation of the actual instance"""
        return pprint.pformat(self.model_dump())


