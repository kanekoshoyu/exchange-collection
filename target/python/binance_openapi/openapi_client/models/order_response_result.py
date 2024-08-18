# coding: utf-8

"""
    Binance Public Spot API

    OpenAPI Specifications for the Binance Public Spot API  API documents:   - [https://github.com/binance/binance-spot-api-docs](https://github.com/binance/binance-spot-api-docs)   - [https://binance-docs.github.io/apidocs/spot/en](https://binance-docs.github.io/apidocs/spot/en)

    The version of the OpenAPI document: 1.0
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


from __future__ import annotations
import pprint
import re  # noqa: F401
import json

from pydantic import BaseModel, ConfigDict, Field, StrictInt, StrictStr
from typing import Any, ClassVar, Dict, List, Optional
from typing import Optional, Set
from typing_extensions import Self

class OrderResponseResult(BaseModel):
    """
    OrderResponseResult
    """ # noqa: E501
    symbol: StrictStr
    order_id: StrictInt = Field(alias="orderId")
    order_list_id: StrictInt = Field(alias="orderListId")
    client_order_id: StrictStr = Field(alias="clientOrderId")
    transact_time: StrictInt = Field(alias="transactTime")
    price: StrictStr
    orig_qty: StrictStr = Field(alias="origQty")
    executed_qty: StrictStr = Field(alias="executedQty")
    cummulative_quote_qty: StrictStr = Field(alias="cummulativeQuoteQty")
    status: StrictStr
    time_in_force: StrictStr = Field(alias="timeInForce")
    type: StrictStr
    side: StrictStr
    strategy_id: Optional[StrictInt] = Field(default=None, alias="strategyId")
    strategy_type: Optional[StrictInt] = Field(default=None, alias="strategyType")
    working_time: StrictInt = Field(alias="workingTime")
    self_trade_prevention_mode: StrictStr = Field(alias="selfTradePreventionMode")
    __properties: ClassVar[List[str]] = ["symbol", "orderId", "orderListId", "clientOrderId", "transactTime", "price", "origQty", "executedQty", "cummulativeQuoteQty", "status", "timeInForce", "type", "side", "strategyId", "strategyType", "workingTime", "selfTradePreventionMode"]

    model_config = ConfigDict(
        populate_by_name=True,
        validate_assignment=True,
        protected_namespaces=(),
    )


    def to_str(self) -> str:
        """Returns the string representation of the model using alias"""
        return pprint.pformat(self.model_dump(by_alias=True))

    def to_json(self) -> str:
        """Returns the JSON representation of the model using alias"""
        # TODO: pydantic v2: use .model_dump_json(by_alias=True, exclude_unset=True) instead
        return json.dumps(self.to_dict())

    @classmethod
    def from_json(cls, json_str: str) -> Optional[Self]:
        """Create an instance of OrderResponseResult from a JSON string"""
        return cls.from_dict(json.loads(json_str))

    def to_dict(self) -> Dict[str, Any]:
        """Return the dictionary representation of the model using alias.

        This has the following differences from calling pydantic's
        `self.model_dump(by_alias=True)`:

        * `None` is only added to the output dict for nullable fields that
          were set at model initialization. Other fields with value `None`
          are ignored.
        """
        excluded_fields: Set[str] = set([
        ])

        _dict = self.model_dump(
            by_alias=True,
            exclude=excluded_fields,
            exclude_none=True,
        )
        return _dict

    @classmethod
    def from_dict(cls, obj: Optional[Dict[str, Any]]) -> Optional[Self]:
        """Create an instance of OrderResponseResult from a dict"""
        if obj is None:
            return None

        if not isinstance(obj, dict):
            return cls.model_validate(obj)

        _obj = cls.model_validate({
            "symbol": obj.get("symbol"),
            "orderId": obj.get("orderId"),
            "orderListId": obj.get("orderListId"),
            "clientOrderId": obj.get("clientOrderId"),
            "transactTime": obj.get("transactTime"),
            "price": obj.get("price"),
            "origQty": obj.get("origQty"),
            "executedQty": obj.get("executedQty"),
            "cummulativeQuoteQty": obj.get("cummulativeQuoteQty"),
            "status": obj.get("status"),
            "timeInForce": obj.get("timeInForce"),
            "type": obj.get("type"),
            "side": obj.get("side"),
            "strategyId": obj.get("strategyId"),
            "strategyType": obj.get("strategyType"),
            "workingTime": obj.get("workingTime"),
            "selfTradePreventionMode": obj.get("selfTradePreventionMode")
        })
        return _obj

