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
from openapi_client.models.batch_modify_order_request import BatchModifyOrderRequest
from openapi_client.models.cancel_order_by_cloid_request import CancelOrderByCloidRequest
from openapi_client.models.cancel_order_request import CancelOrderRequest
from openapi_client.models.initiate_withdrawal_request import InitiateWithdrawalRequest
from openapi_client.models.l1_spot_transfer_request import L1SpotTransferRequest
from openapi_client.models.modify_order_request import ModifyOrderRequest
from openapi_client.models.place_order_request import PlaceOrderRequest
from openapi_client.models.schedule_cancel_request import ScheduleCancelRequest
from openapi_client.models.spot_to_perp_transfer_request import SpotToPerpTransferRequest
from openapi_client.models.update_isolated_margin_request import UpdateIsolatedMarginRequest
from openapi_client.models.update_leverage_request import UpdateLeverageRequest
from openapi_client.models.vault_transfer_request import VaultTransferRequest
from pydantic import StrictStr, Field
from typing import Union, List, Set, Optional, Dict
from typing_extensions import Literal, Self

EXCHANGEPOSTREQUEST_ONE_OF_SCHEMAS = ["BatchModifyOrderRequest", "CancelOrderByCloidRequest", "CancelOrderRequest", "InitiateWithdrawalRequest", "L1SpotTransferRequest", "ModifyOrderRequest", "PlaceOrderRequest", "ScheduleCancelRequest", "SpotToPerpTransferRequest", "UpdateIsolatedMarginRequest", "UpdateLeverageRequest", "VaultTransferRequest"]

class ExchangePostRequest(BaseModel):
    """
    ExchangePostRequest
    """
    # data type: PlaceOrderRequest
    oneof_schema_1_validator: Optional[PlaceOrderRequest] = None
    # data type: CancelOrderRequest
    oneof_schema_2_validator: Optional[CancelOrderRequest] = None
    # data type: CancelOrderByCloidRequest
    oneof_schema_3_validator: Optional[CancelOrderByCloidRequest] = None
    # data type: ScheduleCancelRequest
    oneof_schema_4_validator: Optional[ScheduleCancelRequest] = None
    # data type: ModifyOrderRequest
    oneof_schema_5_validator: Optional[ModifyOrderRequest] = None
    # data type: BatchModifyOrderRequest
    oneof_schema_6_validator: Optional[BatchModifyOrderRequest] = None
    # data type: UpdateLeverageRequest
    oneof_schema_7_validator: Optional[UpdateLeverageRequest] = None
    # data type: UpdateIsolatedMarginRequest
    oneof_schema_8_validator: Optional[UpdateIsolatedMarginRequest] = None
    # data type: L1SpotTransferRequest
    oneof_schema_9_validator: Optional[L1SpotTransferRequest] = None
    # data type: InitiateWithdrawalRequest
    oneof_schema_10_validator: Optional[InitiateWithdrawalRequest] = None
    # data type: SpotToPerpTransferRequest
    oneof_schema_11_validator: Optional[SpotToPerpTransferRequest] = None
    # data type: VaultTransferRequest
    oneof_schema_12_validator: Optional[VaultTransferRequest] = None
    actual_instance: Optional[Union[BatchModifyOrderRequest, CancelOrderByCloidRequest, CancelOrderRequest, InitiateWithdrawalRequest, L1SpotTransferRequest, ModifyOrderRequest, PlaceOrderRequest, ScheduleCancelRequest, SpotToPerpTransferRequest, UpdateIsolatedMarginRequest, UpdateLeverageRequest, VaultTransferRequest]] = None
    one_of_schemas: Set[str] = { "BatchModifyOrderRequest", "CancelOrderByCloidRequest", "CancelOrderRequest", "InitiateWithdrawalRequest", "L1SpotTransferRequest", "ModifyOrderRequest", "PlaceOrderRequest", "ScheduleCancelRequest", "SpotToPerpTransferRequest", "UpdateIsolatedMarginRequest", "UpdateLeverageRequest", "VaultTransferRequest" }

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
        instance = ExchangePostRequest.model_construct()
        error_messages = []
        match = 0
        # validate data type: PlaceOrderRequest
        if not isinstance(v, PlaceOrderRequest):
            error_messages.append(f"Error! Input type `{type(v)}` is not `PlaceOrderRequest`")
        else:
            match += 1
        # validate data type: CancelOrderRequest
        if not isinstance(v, CancelOrderRequest):
            error_messages.append(f"Error! Input type `{type(v)}` is not `CancelOrderRequest`")
        else:
            match += 1
        # validate data type: CancelOrderByCloidRequest
        if not isinstance(v, CancelOrderByCloidRequest):
            error_messages.append(f"Error! Input type `{type(v)}` is not `CancelOrderByCloidRequest`")
        else:
            match += 1
        # validate data type: ScheduleCancelRequest
        if not isinstance(v, ScheduleCancelRequest):
            error_messages.append(f"Error! Input type `{type(v)}` is not `ScheduleCancelRequest`")
        else:
            match += 1
        # validate data type: ModifyOrderRequest
        if not isinstance(v, ModifyOrderRequest):
            error_messages.append(f"Error! Input type `{type(v)}` is not `ModifyOrderRequest`")
        else:
            match += 1
        # validate data type: BatchModifyOrderRequest
        if not isinstance(v, BatchModifyOrderRequest):
            error_messages.append(f"Error! Input type `{type(v)}` is not `BatchModifyOrderRequest`")
        else:
            match += 1
        # validate data type: UpdateLeverageRequest
        if not isinstance(v, UpdateLeverageRequest):
            error_messages.append(f"Error! Input type `{type(v)}` is not `UpdateLeverageRequest`")
        else:
            match += 1
        # validate data type: UpdateIsolatedMarginRequest
        if not isinstance(v, UpdateIsolatedMarginRequest):
            error_messages.append(f"Error! Input type `{type(v)}` is not `UpdateIsolatedMarginRequest`")
        else:
            match += 1
        # validate data type: L1SpotTransferRequest
        if not isinstance(v, L1SpotTransferRequest):
            error_messages.append(f"Error! Input type `{type(v)}` is not `L1SpotTransferRequest`")
        else:
            match += 1
        # validate data type: InitiateWithdrawalRequest
        if not isinstance(v, InitiateWithdrawalRequest):
            error_messages.append(f"Error! Input type `{type(v)}` is not `InitiateWithdrawalRequest`")
        else:
            match += 1
        # validate data type: SpotToPerpTransferRequest
        if not isinstance(v, SpotToPerpTransferRequest):
            error_messages.append(f"Error! Input type `{type(v)}` is not `SpotToPerpTransferRequest`")
        else:
            match += 1
        # validate data type: VaultTransferRequest
        if not isinstance(v, VaultTransferRequest):
            error_messages.append(f"Error! Input type `{type(v)}` is not `VaultTransferRequest`")
        else:
            match += 1
        if match > 1:
            # more than 1 match
            raise ValueError("Multiple matches found when setting `actual_instance` in ExchangePostRequest with oneOf schemas: BatchModifyOrderRequest, CancelOrderByCloidRequest, CancelOrderRequest, InitiateWithdrawalRequest, L1SpotTransferRequest, ModifyOrderRequest, PlaceOrderRequest, ScheduleCancelRequest, SpotToPerpTransferRequest, UpdateIsolatedMarginRequest, UpdateLeverageRequest, VaultTransferRequest. Details: " + ", ".join(error_messages))
        elif match == 0:
            # no match
            raise ValueError("No match found when setting `actual_instance` in ExchangePostRequest with oneOf schemas: BatchModifyOrderRequest, CancelOrderByCloidRequest, CancelOrderRequest, InitiateWithdrawalRequest, L1SpotTransferRequest, ModifyOrderRequest, PlaceOrderRequest, ScheduleCancelRequest, SpotToPerpTransferRequest, UpdateIsolatedMarginRequest, UpdateLeverageRequest, VaultTransferRequest. Details: " + ", ".join(error_messages))
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

        # deserialize data into PlaceOrderRequest
        try:
            instance.actual_instance = PlaceOrderRequest.from_json(json_str)
            match += 1
        except (ValidationError, ValueError) as e:
            error_messages.append(str(e))
        # deserialize data into CancelOrderRequest
        try:
            instance.actual_instance = CancelOrderRequest.from_json(json_str)
            match += 1
        except (ValidationError, ValueError) as e:
            error_messages.append(str(e))
        # deserialize data into CancelOrderByCloidRequest
        try:
            instance.actual_instance = CancelOrderByCloidRequest.from_json(json_str)
            match += 1
        except (ValidationError, ValueError) as e:
            error_messages.append(str(e))
        # deserialize data into ScheduleCancelRequest
        try:
            instance.actual_instance = ScheduleCancelRequest.from_json(json_str)
            match += 1
        except (ValidationError, ValueError) as e:
            error_messages.append(str(e))
        # deserialize data into ModifyOrderRequest
        try:
            instance.actual_instance = ModifyOrderRequest.from_json(json_str)
            match += 1
        except (ValidationError, ValueError) as e:
            error_messages.append(str(e))
        # deserialize data into BatchModifyOrderRequest
        try:
            instance.actual_instance = BatchModifyOrderRequest.from_json(json_str)
            match += 1
        except (ValidationError, ValueError) as e:
            error_messages.append(str(e))
        # deserialize data into UpdateLeverageRequest
        try:
            instance.actual_instance = UpdateLeverageRequest.from_json(json_str)
            match += 1
        except (ValidationError, ValueError) as e:
            error_messages.append(str(e))
        # deserialize data into UpdateIsolatedMarginRequest
        try:
            instance.actual_instance = UpdateIsolatedMarginRequest.from_json(json_str)
            match += 1
        except (ValidationError, ValueError) as e:
            error_messages.append(str(e))
        # deserialize data into L1SpotTransferRequest
        try:
            instance.actual_instance = L1SpotTransferRequest.from_json(json_str)
            match += 1
        except (ValidationError, ValueError) as e:
            error_messages.append(str(e))
        # deserialize data into InitiateWithdrawalRequest
        try:
            instance.actual_instance = InitiateWithdrawalRequest.from_json(json_str)
            match += 1
        except (ValidationError, ValueError) as e:
            error_messages.append(str(e))
        # deserialize data into SpotToPerpTransferRequest
        try:
            instance.actual_instance = SpotToPerpTransferRequest.from_json(json_str)
            match += 1
        except (ValidationError, ValueError) as e:
            error_messages.append(str(e))
        # deserialize data into VaultTransferRequest
        try:
            instance.actual_instance = VaultTransferRequest.from_json(json_str)
            match += 1
        except (ValidationError, ValueError) as e:
            error_messages.append(str(e))

        if match > 1:
            # more than 1 match
            raise ValueError("Multiple matches found when deserializing the JSON string into ExchangePostRequest with oneOf schemas: BatchModifyOrderRequest, CancelOrderByCloidRequest, CancelOrderRequest, InitiateWithdrawalRequest, L1SpotTransferRequest, ModifyOrderRequest, PlaceOrderRequest, ScheduleCancelRequest, SpotToPerpTransferRequest, UpdateIsolatedMarginRequest, UpdateLeverageRequest, VaultTransferRequest. Details: " + ", ".join(error_messages))
        elif match == 0:
            # no match
            raise ValueError("No match found when deserializing the JSON string into ExchangePostRequest with oneOf schemas: BatchModifyOrderRequest, CancelOrderByCloidRequest, CancelOrderRequest, InitiateWithdrawalRequest, L1SpotTransferRequest, ModifyOrderRequest, PlaceOrderRequest, ScheduleCancelRequest, SpotToPerpTransferRequest, UpdateIsolatedMarginRequest, UpdateLeverageRequest, VaultTransferRequest. Details: " + ", ".join(error_messages))
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

    def to_dict(self) -> Optional[Union[Dict[str, Any], BatchModifyOrderRequest, CancelOrderByCloidRequest, CancelOrderRequest, InitiateWithdrawalRequest, L1SpotTransferRequest, ModifyOrderRequest, PlaceOrderRequest, ScheduleCancelRequest, SpotToPerpTransferRequest, UpdateIsolatedMarginRequest, UpdateLeverageRequest, VaultTransferRequest]]:
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

