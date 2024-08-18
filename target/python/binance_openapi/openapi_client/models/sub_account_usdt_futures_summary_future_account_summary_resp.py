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
from openapi_client.models.sub_account_usdt_futures_summary_future_account_summary_resp_sub_account_list_inner import SubAccountUSDTFuturesSummaryFutureAccountSummaryRespSubAccountListInner
from typing import Optional, Set
from typing_extensions import Self

class SubAccountUSDTFuturesSummaryFutureAccountSummaryResp(BaseModel):
    """
    SubAccountUSDTFuturesSummaryFutureAccountSummaryResp
    """ # noqa: E501
    total_initial_margin: StrictStr = Field(alias="totalInitialMargin")
    total_maintenance_margin: StrictStr = Field(alias="totalMaintenanceMargin")
    total_margin_balance: StrictStr = Field(alias="totalMarginBalance")
    total_open_order_initial_margin: StrictStr = Field(alias="totalOpenOrderInitialMargin")
    total_position_initial_margin: StrictStr = Field(alias="totalPositionInitialMargin")
    total_unrealized_profit: StrictStr = Field(alias="totalUnrealizedProfit")
    total_wallet_balance: StrictStr = Field(alias="totalWalletBalance")
    asset: StrictStr = Field(description="The sum of BUSD and USDT")
    sub_account_list: List[SubAccountUSDTFuturesSummaryFutureAccountSummaryRespSubAccountListInner] = Field(alias="subAccountList")
    __properties: ClassVar[List[str]] = ["totalInitialMargin", "totalMaintenanceMargin", "totalMarginBalance", "totalOpenOrderInitialMargin", "totalPositionInitialMargin", "totalUnrealizedProfit", "totalWalletBalance", "asset", "subAccountList"]

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
        """Create an instance of SubAccountUSDTFuturesSummaryFutureAccountSummaryResp from a JSON string"""
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
        # override the default output from pydantic by calling `to_dict()` of each item in sub_account_list (list)
        _items = []
        if self.sub_account_list:
            for _item in self.sub_account_list:
                if _item:
                    _items.append(_item.to_dict())
            _dict['subAccountList'] = _items
        return _dict

    @classmethod
    def from_dict(cls, obj: Optional[Dict[str, Any]]) -> Optional[Self]:
        """Create an instance of SubAccountUSDTFuturesSummaryFutureAccountSummaryResp from a dict"""
        if obj is None:
            return None

        if not isinstance(obj, dict):
            return cls.model_validate(obj)

        _obj = cls.model_validate({
            "totalInitialMargin": obj.get("totalInitialMargin"),
            "totalMaintenanceMargin": obj.get("totalMaintenanceMargin"),
            "totalMarginBalance": obj.get("totalMarginBalance"),
            "totalOpenOrderInitialMargin": obj.get("totalOpenOrderInitialMargin"),
            "totalPositionInitialMargin": obj.get("totalPositionInitialMargin"),
            "totalUnrealizedProfit": obj.get("totalUnrealizedProfit"),
            "totalWalletBalance": obj.get("totalWalletBalance"),
            "asset": obj.get("asset"),
            "subAccountList": [SubAccountUSDTFuturesSummaryFutureAccountSummaryRespSubAccountListInner.from_dict(_item) for _item in obj["subAccountList"]] if obj.get("subAccountList") is not None else None
        })
        return _obj

