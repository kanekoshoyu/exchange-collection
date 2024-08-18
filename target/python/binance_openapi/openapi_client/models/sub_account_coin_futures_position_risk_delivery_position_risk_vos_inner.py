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

from pydantic import BaseModel, ConfigDict, Field, StrictStr
from typing import Any, ClassVar, Dict, List
from typing import Optional, Set
from typing_extensions import Self

class SubAccountCOINFuturesPositionRiskDeliveryPositionRiskVosInner(BaseModel):
    """
    SubAccountCOINFuturesPositionRiskDeliveryPositionRiskVosInner
    """ # noqa: E501
    entry_price: StrictStr = Field(alias="entryPrice")
    mark_price: StrictStr = Field(alias="markPrice")
    leverage: StrictStr
    isolated: StrictStr
    isolated_wallet: StrictStr = Field(alias="isolatedWallet")
    isolated_margin: StrictStr = Field(alias="isolatedMargin")
    is_auto_add_margin: StrictStr = Field(alias="isAutoAddMargin")
    position_side: StrictStr = Field(alias="positionSide")
    position_amount: StrictStr = Field(alias="positionAmount")
    symbol: StrictStr
    unrealized_profit: StrictStr = Field(alias="unrealizedProfit")
    __properties: ClassVar[List[str]] = ["entryPrice", "markPrice", "leverage", "isolated", "isolatedWallet", "isolatedMargin", "isAutoAddMargin", "positionSide", "positionAmount", "symbol", "unrealizedProfit"]

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
        """Create an instance of SubAccountCOINFuturesPositionRiskDeliveryPositionRiskVosInner from a JSON string"""
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
        """Create an instance of SubAccountCOINFuturesPositionRiskDeliveryPositionRiskVosInner from a dict"""
        if obj is None:
            return None

        if not isinstance(obj, dict):
            return cls.model_validate(obj)

        _obj = cls.model_validate({
            "entryPrice": obj.get("entryPrice"),
            "markPrice": obj.get("markPrice"),
            "leverage": obj.get("leverage"),
            "isolated": obj.get("isolated"),
            "isolatedWallet": obj.get("isolatedWallet"),
            "isolatedMargin": obj.get("isolatedMargin"),
            "isAutoAddMargin": obj.get("isAutoAddMargin"),
            "positionSide": obj.get("positionSide"),
            "positionAmount": obj.get("positionAmount"),
            "symbol": obj.get("symbol"),
            "unrealizedProfit": obj.get("unrealizedProfit")
        })
        return _obj

