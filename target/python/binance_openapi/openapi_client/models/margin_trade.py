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

from pydantic import BaseModel, ConfigDict, Field, StrictBool, StrictInt, StrictStr
from typing import Any, ClassVar, Dict, List
from typing import Optional, Set
from typing_extensions import Self

class MarginTrade(BaseModel):
    """
    MarginTrade
    """ # noqa: E501
    commission: StrictStr
    commission_asset: StrictStr = Field(alias="commissionAsset")
    id: StrictInt
    is_best_match: StrictBool = Field(alias="isBestMatch")
    is_buyer: StrictBool = Field(alias="isBuyer")
    is_maker: StrictBool = Field(alias="isMaker")
    order_id: StrictInt = Field(alias="orderId")
    price: StrictStr
    qty: StrictStr
    symbol: StrictStr
    is_isolated: StrictBool = Field(alias="isIsolated")
    time: StrictInt
    __properties: ClassVar[List[str]] = ["commission", "commissionAsset", "id", "isBestMatch", "isBuyer", "isMaker", "orderId", "price", "qty", "symbol", "isIsolated", "time"]

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
        """Create an instance of MarginTrade from a JSON string"""
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
        """Create an instance of MarginTrade from a dict"""
        if obj is None:
            return None

        if not isinstance(obj, dict):
            return cls.model_validate(obj)

        _obj = cls.model_validate({
            "commission": obj.get("commission"),
            "commissionAsset": obj.get("commissionAsset"),
            "id": obj.get("id"),
            "isBestMatch": obj.get("isBestMatch"),
            "isBuyer": obj.get("isBuyer"),
            "isMaker": obj.get("isMaker"),
            "orderId": obj.get("orderId"),
            "price": obj.get("price"),
            "qty": obj.get("qty"),
            "symbol": obj.get("symbol"),
            "isIsolated": obj.get("isIsolated"),
            "time": obj.get("time")
        })
        return _obj

