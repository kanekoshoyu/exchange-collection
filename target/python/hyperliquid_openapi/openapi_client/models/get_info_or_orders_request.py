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
from openapi_client.models.candle_snapshot_request import CandleSnapshotRequest
from openapi_client.models.clearinghouse_state_request import ClearinghouseStateRequest
from openapi_client.models.frontend_open_orders_request import FrontendOpenOrdersRequest
from openapi_client.models.funding_history_request import FundingHistoryRequest
from openapi_client.models.l2_book_request import L2BookRequest
from openapi_client.models.market_data_request import MarketDataRequest
from openapi_client.models.meta_and_asset_ctxs_request import MetaAndAssetCtxsRequest
from openapi_client.models.meta_request import MetaRequest
from openapi_client.models.open_orders_request import OpenOrdersRequest
from openapi_client.models.order_status_request import OrderStatusRequest
from openapi_client.models.spot_clearinghouse_state_request import SpotClearinghouseStateRequest
from openapi_client.models.spot_meta_and_asset_ctxs_request import SpotMetaAndAssetCtxsRequest
from openapi_client.models.spot_meta_request import SpotMetaRequest
from openapi_client.models.user_fills_by_time_request import UserFillsByTimeRequest
from openapi_client.models.user_fills_request import UserFillsRequest
from openapi_client.models.user_funding_or_ledger_updates_request import UserFundingOrLedgerUpdatesRequest
from pydantic import StrictStr, Field
from typing import Union, List, Set, Optional, Dict
from typing_extensions import Literal, Self

GETINFOORORDERSREQUEST_ONE_OF_SCHEMAS = ["CandleSnapshotRequest", "ClearinghouseStateRequest", "FrontendOpenOrdersRequest", "FundingHistoryRequest", "L2BookRequest", "MarketDataRequest", "MetaAndAssetCtxsRequest", "MetaRequest", "OpenOrdersRequest", "OrderStatusRequest", "SpotClearinghouseStateRequest", "SpotMetaAndAssetCtxsRequest", "SpotMetaRequest", "UserFillsByTimeRequest", "UserFillsRequest", "UserFundingOrLedgerUpdatesRequest"]

class GetInfoOrOrdersRequest(BaseModel):
    """
    GetInfoOrOrdersRequest
    """
    # data type: MarketDataRequest
    oneof_schema_1_validator: Optional[MarketDataRequest] = None
    # data type: OpenOrdersRequest
    oneof_schema_2_validator: Optional[OpenOrdersRequest] = None
    # data type: FrontendOpenOrdersRequest
    oneof_schema_3_validator: Optional[FrontendOpenOrdersRequest] = None
    # data type: UserFillsRequest
    oneof_schema_4_validator: Optional[UserFillsRequest] = None
    # data type: UserFillsByTimeRequest
    oneof_schema_5_validator: Optional[UserFillsByTimeRequest] = None
    # data type: OrderStatusRequest
    oneof_schema_6_validator: Optional[OrderStatusRequest] = None
    # data type: L2BookRequest
    oneof_schema_7_validator: Optional[L2BookRequest] = None
    # data type: CandleSnapshotRequest
    oneof_schema_8_validator: Optional[CandleSnapshotRequest] = None
    # data type: MetaRequest
    oneof_schema_9_validator: Optional[MetaRequest] = None
    # data type: MetaAndAssetCtxsRequest
    oneof_schema_10_validator: Optional[MetaAndAssetCtxsRequest] = None
    # data type: ClearinghouseStateRequest
    oneof_schema_11_validator: Optional[ClearinghouseStateRequest] = None
    # data type: UserFundingOrLedgerUpdatesRequest
    oneof_schema_12_validator: Optional[UserFundingOrLedgerUpdatesRequest] = None
    # data type: FundingHistoryRequest
    oneof_schema_13_validator: Optional[FundingHistoryRequest] = None
    # data type: SpotMetaRequest
    oneof_schema_14_validator: Optional[SpotMetaRequest] = None
    # data type: SpotMetaAndAssetCtxsRequest
    oneof_schema_15_validator: Optional[SpotMetaAndAssetCtxsRequest] = None
    # data type: SpotClearinghouseStateRequest
    oneof_schema_16_validator: Optional[SpotClearinghouseStateRequest] = None
    actual_instance: Optional[Union[CandleSnapshotRequest, ClearinghouseStateRequest, FrontendOpenOrdersRequest, FundingHistoryRequest, L2BookRequest, MarketDataRequest, MetaAndAssetCtxsRequest, MetaRequest, OpenOrdersRequest, OrderStatusRequest, SpotClearinghouseStateRequest, SpotMetaAndAssetCtxsRequest, SpotMetaRequest, UserFillsByTimeRequest, UserFillsRequest, UserFundingOrLedgerUpdatesRequest]] = None
    one_of_schemas: Set[str] = { "CandleSnapshotRequest", "ClearinghouseStateRequest", "FrontendOpenOrdersRequest", "FundingHistoryRequest", "L2BookRequest", "MarketDataRequest", "MetaAndAssetCtxsRequest", "MetaRequest", "OpenOrdersRequest", "OrderStatusRequest", "SpotClearinghouseStateRequest", "SpotMetaAndAssetCtxsRequest", "SpotMetaRequest", "UserFillsByTimeRequest", "UserFillsRequest", "UserFundingOrLedgerUpdatesRequest" }

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
        instance = GetInfoOrOrdersRequest.model_construct()
        error_messages = []
        match = 0
        # validate data type: MarketDataRequest
        if not isinstance(v, MarketDataRequest):
            error_messages.append(f"Error! Input type `{type(v)}` is not `MarketDataRequest`")
        else:
            match += 1
        # validate data type: OpenOrdersRequest
        if not isinstance(v, OpenOrdersRequest):
            error_messages.append(f"Error! Input type `{type(v)}` is not `OpenOrdersRequest`")
        else:
            match += 1
        # validate data type: FrontendOpenOrdersRequest
        if not isinstance(v, FrontendOpenOrdersRequest):
            error_messages.append(f"Error! Input type `{type(v)}` is not `FrontendOpenOrdersRequest`")
        else:
            match += 1
        # validate data type: UserFillsRequest
        if not isinstance(v, UserFillsRequest):
            error_messages.append(f"Error! Input type `{type(v)}` is not `UserFillsRequest`")
        else:
            match += 1
        # validate data type: UserFillsByTimeRequest
        if not isinstance(v, UserFillsByTimeRequest):
            error_messages.append(f"Error! Input type `{type(v)}` is not `UserFillsByTimeRequest`")
        else:
            match += 1
        # validate data type: OrderStatusRequest
        if not isinstance(v, OrderStatusRequest):
            error_messages.append(f"Error! Input type `{type(v)}` is not `OrderStatusRequest`")
        else:
            match += 1
        # validate data type: L2BookRequest
        if not isinstance(v, L2BookRequest):
            error_messages.append(f"Error! Input type `{type(v)}` is not `L2BookRequest`")
        else:
            match += 1
        # validate data type: CandleSnapshotRequest
        if not isinstance(v, CandleSnapshotRequest):
            error_messages.append(f"Error! Input type `{type(v)}` is not `CandleSnapshotRequest`")
        else:
            match += 1
        # validate data type: MetaRequest
        if not isinstance(v, MetaRequest):
            error_messages.append(f"Error! Input type `{type(v)}` is not `MetaRequest`")
        else:
            match += 1
        # validate data type: MetaAndAssetCtxsRequest
        if not isinstance(v, MetaAndAssetCtxsRequest):
            error_messages.append(f"Error! Input type `{type(v)}` is not `MetaAndAssetCtxsRequest`")
        else:
            match += 1
        # validate data type: ClearinghouseStateRequest
        if not isinstance(v, ClearinghouseStateRequest):
            error_messages.append(f"Error! Input type `{type(v)}` is not `ClearinghouseStateRequest`")
        else:
            match += 1
        # validate data type: UserFundingOrLedgerUpdatesRequest
        if not isinstance(v, UserFundingOrLedgerUpdatesRequest):
            error_messages.append(f"Error! Input type `{type(v)}` is not `UserFundingOrLedgerUpdatesRequest`")
        else:
            match += 1
        # validate data type: FundingHistoryRequest
        if not isinstance(v, FundingHistoryRequest):
            error_messages.append(f"Error! Input type `{type(v)}` is not `FundingHistoryRequest`")
        else:
            match += 1
        # validate data type: SpotMetaRequest
        if not isinstance(v, SpotMetaRequest):
            error_messages.append(f"Error! Input type `{type(v)}` is not `SpotMetaRequest`")
        else:
            match += 1
        # validate data type: SpotMetaAndAssetCtxsRequest
        if not isinstance(v, SpotMetaAndAssetCtxsRequest):
            error_messages.append(f"Error! Input type `{type(v)}` is not `SpotMetaAndAssetCtxsRequest`")
        else:
            match += 1
        # validate data type: SpotClearinghouseStateRequest
        if not isinstance(v, SpotClearinghouseStateRequest):
            error_messages.append(f"Error! Input type `{type(v)}` is not `SpotClearinghouseStateRequest`")
        else:
            match += 1
        if match > 1:
            # more than 1 match
            raise ValueError("Multiple matches found when setting `actual_instance` in GetInfoOrOrdersRequest with oneOf schemas: CandleSnapshotRequest, ClearinghouseStateRequest, FrontendOpenOrdersRequest, FundingHistoryRequest, L2BookRequest, MarketDataRequest, MetaAndAssetCtxsRequest, MetaRequest, OpenOrdersRequest, OrderStatusRequest, SpotClearinghouseStateRequest, SpotMetaAndAssetCtxsRequest, SpotMetaRequest, UserFillsByTimeRequest, UserFillsRequest, UserFundingOrLedgerUpdatesRequest. Details: " + ", ".join(error_messages))
        elif match == 0:
            # no match
            raise ValueError("No match found when setting `actual_instance` in GetInfoOrOrdersRequest with oneOf schemas: CandleSnapshotRequest, ClearinghouseStateRequest, FrontendOpenOrdersRequest, FundingHistoryRequest, L2BookRequest, MarketDataRequest, MetaAndAssetCtxsRequest, MetaRequest, OpenOrdersRequest, OrderStatusRequest, SpotClearinghouseStateRequest, SpotMetaAndAssetCtxsRequest, SpotMetaRequest, UserFillsByTimeRequest, UserFillsRequest, UserFundingOrLedgerUpdatesRequest. Details: " + ", ".join(error_messages))
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

        # deserialize data into MarketDataRequest
        try:
            instance.actual_instance = MarketDataRequest.from_json(json_str)
            match += 1
        except (ValidationError, ValueError) as e:
            error_messages.append(str(e))
        # deserialize data into OpenOrdersRequest
        try:
            instance.actual_instance = OpenOrdersRequest.from_json(json_str)
            match += 1
        except (ValidationError, ValueError) as e:
            error_messages.append(str(e))
        # deserialize data into FrontendOpenOrdersRequest
        try:
            instance.actual_instance = FrontendOpenOrdersRequest.from_json(json_str)
            match += 1
        except (ValidationError, ValueError) as e:
            error_messages.append(str(e))
        # deserialize data into UserFillsRequest
        try:
            instance.actual_instance = UserFillsRequest.from_json(json_str)
            match += 1
        except (ValidationError, ValueError) as e:
            error_messages.append(str(e))
        # deserialize data into UserFillsByTimeRequest
        try:
            instance.actual_instance = UserFillsByTimeRequest.from_json(json_str)
            match += 1
        except (ValidationError, ValueError) as e:
            error_messages.append(str(e))
        # deserialize data into OrderStatusRequest
        try:
            instance.actual_instance = OrderStatusRequest.from_json(json_str)
            match += 1
        except (ValidationError, ValueError) as e:
            error_messages.append(str(e))
        # deserialize data into L2BookRequest
        try:
            instance.actual_instance = L2BookRequest.from_json(json_str)
            match += 1
        except (ValidationError, ValueError) as e:
            error_messages.append(str(e))
        # deserialize data into CandleSnapshotRequest
        try:
            instance.actual_instance = CandleSnapshotRequest.from_json(json_str)
            match += 1
        except (ValidationError, ValueError) as e:
            error_messages.append(str(e))
        # deserialize data into MetaRequest
        try:
            instance.actual_instance = MetaRequest.from_json(json_str)
            match += 1
        except (ValidationError, ValueError) as e:
            error_messages.append(str(e))
        # deserialize data into MetaAndAssetCtxsRequest
        try:
            instance.actual_instance = MetaAndAssetCtxsRequest.from_json(json_str)
            match += 1
        except (ValidationError, ValueError) as e:
            error_messages.append(str(e))
        # deserialize data into ClearinghouseStateRequest
        try:
            instance.actual_instance = ClearinghouseStateRequest.from_json(json_str)
            match += 1
        except (ValidationError, ValueError) as e:
            error_messages.append(str(e))
        # deserialize data into UserFundingOrLedgerUpdatesRequest
        try:
            instance.actual_instance = UserFundingOrLedgerUpdatesRequest.from_json(json_str)
            match += 1
        except (ValidationError, ValueError) as e:
            error_messages.append(str(e))
        # deserialize data into FundingHistoryRequest
        try:
            instance.actual_instance = FundingHistoryRequest.from_json(json_str)
            match += 1
        except (ValidationError, ValueError) as e:
            error_messages.append(str(e))
        # deserialize data into SpotMetaRequest
        try:
            instance.actual_instance = SpotMetaRequest.from_json(json_str)
            match += 1
        except (ValidationError, ValueError) as e:
            error_messages.append(str(e))
        # deserialize data into SpotMetaAndAssetCtxsRequest
        try:
            instance.actual_instance = SpotMetaAndAssetCtxsRequest.from_json(json_str)
            match += 1
        except (ValidationError, ValueError) as e:
            error_messages.append(str(e))
        # deserialize data into SpotClearinghouseStateRequest
        try:
            instance.actual_instance = SpotClearinghouseStateRequest.from_json(json_str)
            match += 1
        except (ValidationError, ValueError) as e:
            error_messages.append(str(e))

        if match > 1:
            # more than 1 match
            raise ValueError("Multiple matches found when deserializing the JSON string into GetInfoOrOrdersRequest with oneOf schemas: CandleSnapshotRequest, ClearinghouseStateRequest, FrontendOpenOrdersRequest, FundingHistoryRequest, L2BookRequest, MarketDataRequest, MetaAndAssetCtxsRequest, MetaRequest, OpenOrdersRequest, OrderStatusRequest, SpotClearinghouseStateRequest, SpotMetaAndAssetCtxsRequest, SpotMetaRequest, UserFillsByTimeRequest, UserFillsRequest, UserFundingOrLedgerUpdatesRequest. Details: " + ", ".join(error_messages))
        elif match == 0:
            # no match
            raise ValueError("No match found when deserializing the JSON string into GetInfoOrOrdersRequest with oneOf schemas: CandleSnapshotRequest, ClearinghouseStateRequest, FrontendOpenOrdersRequest, FundingHistoryRequest, L2BookRequest, MarketDataRequest, MetaAndAssetCtxsRequest, MetaRequest, OpenOrdersRequest, OrderStatusRequest, SpotClearinghouseStateRequest, SpotMetaAndAssetCtxsRequest, SpotMetaRequest, UserFillsByTimeRequest, UserFillsRequest, UserFundingOrLedgerUpdatesRequest. Details: " + ", ".join(error_messages))
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

    def to_dict(self) -> Optional[Union[Dict[str, Any], CandleSnapshotRequest, ClearinghouseStateRequest, FrontendOpenOrdersRequest, FundingHistoryRequest, L2BookRequest, MarketDataRequest, MetaAndAssetCtxsRequest, MetaRequest, OpenOrdersRequest, OrderStatusRequest, SpotClearinghouseStateRequest, SpotMetaAndAssetCtxsRequest, SpotMetaRequest, UserFillsByTimeRequest, UserFillsRequest, UserFundingOrLedgerUpdatesRequest]]:
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

