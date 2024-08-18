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

class SapiV1AlgoFuturesSubOrdersGet200ResponseSubOrdersInner(BaseModel):
    """
    SapiV1AlgoFuturesSubOrdersGet200ResponseSubOrdersInner
    """ # noqa: E501
    algo_id: StrictInt = Field(alias="algoId")
    order_id: StrictInt = Field(alias="orderId")
    order_status: StrictStr = Field(alias="orderStatus")
    executed_qty: Optional[StrictStr] = Field(default=None, alias="executedQty")
    executed_amt: StrictStr = Field(alias="executedAmt")
    fee_amt: StrictStr = Field(alias="feeAmt")
    fee_asset: StrictStr = Field(alias="feeAsset")
    book_time: StrictInt = Field(alias="bookTime")
    avg_price: StrictStr = Field(alias="avgPrice")
    side: StrictStr
    symbol: StrictStr
    sub_id: StrictInt = Field(alias="subId")
    time_in_force: StrictStr = Field(alias="timeInForce")
    orig_qty: StrictStr = Field(alias="origQty")
    __properties: ClassVar[List[str]] = ["algoId", "orderId", "orderStatus", "executedQty", "executedAmt", "feeAmt", "feeAsset", "bookTime", "avgPrice", "side", "symbol", "subId", "timeInForce", "origQty"]

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
        """Create an instance of SapiV1AlgoFuturesSubOrdersGet200ResponseSubOrdersInner from a JSON string"""
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
        """Create an instance of SapiV1AlgoFuturesSubOrdersGet200ResponseSubOrdersInner from a dict"""
        if obj is None:
            return None

        if not isinstance(obj, dict):
            return cls.model_validate(obj)

        _obj = cls.model_validate({
            "algoId": obj.get("algoId"),
            "orderId": obj.get("orderId"),
            "orderStatus": obj.get("orderStatus"),
            "executedQty": obj.get("executedQty"),
            "executedAmt": obj.get("executedAmt"),
            "feeAmt": obj.get("feeAmt"),
            "feeAsset": obj.get("feeAsset"),
            "bookTime": obj.get("bookTime"),
            "avgPrice": obj.get("avgPrice"),
            "side": obj.get("side"),
            "symbol": obj.get("symbol"),
            "subId": obj.get("subId"),
            "timeInForce": obj.get("timeInForce"),
            "origQty": obj.get("origQty")
        })
        return _obj

