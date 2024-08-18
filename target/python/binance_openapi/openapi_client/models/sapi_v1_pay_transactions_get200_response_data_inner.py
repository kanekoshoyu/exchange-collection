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
from typing import Any, ClassVar, Dict, List
from openapi_client.models.sapi_v1_pay_transactions_get200_response_data_inner_funds_detail_inner import SapiV1PayTransactionsGet200ResponseDataInnerFundsDetailInner
from openapi_client.models.sapi_v1_pay_transactions_get200_response_data_inner_payer_info import SapiV1PayTransactionsGet200ResponseDataInnerPayerInfo
from openapi_client.models.sapi_v1_pay_transactions_get200_response_data_inner_receiver_info import SapiV1PayTransactionsGet200ResponseDataInnerReceiverInfo
from typing import Optional, Set
from typing_extensions import Self

class SapiV1PayTransactionsGet200ResponseDataInner(BaseModel):
    """
    SapiV1PayTransactionsGet200ResponseDataInner
    """ # noqa: E501
    order_type: StrictStr = Field(description="Enum：PAY(C2B Merchant Acquiring Payment), PAY_REFUND(C2B Merchant Acquiring Payment,refund), C2C(C2C Transfer Payment),CRYPTO_BOX(Crypto box), CRYPTO_BOX_RF(Crypto Box, refund), C2C_HOLDING(Transfer to new Binance user), C2C_HOLDING_RF(Transfer to new Binance user,refund), PAYOUT(B2C Disbursement Payment)", alias="orderType")
    transaction_id: StrictStr = Field(alias="transactionId")
    transaction_time: StrictInt = Field(alias="transactionTime")
    amount: StrictStr = Field(description="order amount(up to 8 decimal places), positive is income, negative is expenditure")
    currency: StrictStr
    wallet_type: StrictInt = Field(alias="walletType")
    wallet_types: List[StrictInt] = Field(alias="walletTypes")
    funds_detail: List[SapiV1PayTransactionsGet200ResponseDataInnerFundsDetailInner] = Field(alias="fundsDetail")
    payer_info: SapiV1PayTransactionsGet200ResponseDataInnerPayerInfo = Field(alias="payerInfo")
    receiver_info: SapiV1PayTransactionsGet200ResponseDataInnerReceiverInfo = Field(alias="receiverInfo")
    __properties: ClassVar[List[str]] = ["orderType", "transactionId", "transactionTime", "amount", "currency", "walletType", "walletTypes", "fundsDetail", "payerInfo", "receiverInfo"]

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
        """Create an instance of SapiV1PayTransactionsGet200ResponseDataInner from a JSON string"""
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
        # override the default output from pydantic by calling `to_dict()` of each item in funds_detail (list)
        _items = []
        if self.funds_detail:
            for _item in self.funds_detail:
                if _item:
                    _items.append(_item.to_dict())
            _dict['fundsDetail'] = _items
        # override the default output from pydantic by calling `to_dict()` of payer_info
        if self.payer_info:
            _dict['payerInfo'] = self.payer_info.to_dict()
        # override the default output from pydantic by calling `to_dict()` of receiver_info
        if self.receiver_info:
            _dict['receiverInfo'] = self.receiver_info.to_dict()
        return _dict

    @classmethod
    def from_dict(cls, obj: Optional[Dict[str, Any]]) -> Optional[Self]:
        """Create an instance of SapiV1PayTransactionsGet200ResponseDataInner from a dict"""
        if obj is None:
            return None

        if not isinstance(obj, dict):
            return cls.model_validate(obj)

        _obj = cls.model_validate({
            "orderType": obj.get("orderType"),
            "transactionId": obj.get("transactionId"),
            "transactionTime": obj.get("transactionTime"),
            "amount": obj.get("amount"),
            "currency": obj.get("currency"),
            "walletType": obj.get("walletType"),
            "walletTypes": obj.get("walletTypes"),
            "fundsDetail": [SapiV1PayTransactionsGet200ResponseDataInnerFundsDetailInner.from_dict(_item) for _item in obj["fundsDetail"]] if obj.get("fundsDetail") is not None else None,
            "payerInfo": SapiV1PayTransactionsGet200ResponseDataInnerPayerInfo.from_dict(obj["payerInfo"]) if obj.get("payerInfo") is not None else None,
            "receiverInfo": SapiV1PayTransactionsGet200ResponseDataInnerReceiverInfo.from_dict(obj["receiverInfo"]) if obj.get("receiverInfo") is not None else None
        })
        return _obj

