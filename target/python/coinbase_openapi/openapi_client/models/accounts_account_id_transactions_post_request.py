# coding: utf-8

"""
    Coinbase API

    The Coinbase v2 API

    The version of the OpenAPI document: 2.0.0
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


from __future__ import annotations
import pprint
import re  # noqa: F401
import json

from pydantic import BaseModel, ConfigDict, Field, StrictBool, StrictStr
from typing import Any, ClassVar, Dict, List, Optional
from typing import Optional, Set
from typing_extensions import Self

class AccountsAccountIdTransactionsPostRequest(BaseModel):
    """
    AccountsAccountIdTransactionsPostRequest
    """ # noqa: E501
    type: Optional[StrictStr] = Field(default=None, description="Type should be \"send\" for sending money, or \"request\" for requesting money.")
    to: Optional[StrictStr] = Field(default=None, description="A bitcoin address (send only) or an email of the recipient (send or request)")
    amount: Optional[StrictStr] = Field(default=None, description="Amount to be sent/requested.")
    currency: Optional[StrictStr] = Field(default=None, description="Currency for the amount")
    description: Optional[StrictStr] = Field(default=None, description="Notes to be included in the email that the recipient receives")
    skip_notifications: Optional[StrictBool] = Field(default=None, description="(Send only) Don’t send notification emails for small amounts (e.g. tips)")
    fee: Optional[StrictStr] = Field(default=None, description="(Send only) Transaction fee in BTC if you would like to pay it. Coinbase pays transaction fees on payments greater than or equal to 0.0001 BTC. But for smaller amounts you may want to add your own amount. Fees can be added as a string, such as 0.0005")
    idem: Optional[StrictStr] = Field(default=None, description="(Send only) A token to ensure idempotence. If a previous transaction with the same idem parameter already exists for this sender, that previous transaction will be returned and a new one will not be created. Max length 100 characters")
    __properties: ClassVar[List[str]] = ["type", "to", "amount", "currency", "description", "skip_notifications", "fee", "idem"]

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
        """Create an instance of AccountsAccountIdTransactionsPostRequest from a JSON string"""
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
        """Create an instance of AccountsAccountIdTransactionsPostRequest from a dict"""
        if obj is None:
            return None

        if not isinstance(obj, dict):
            return cls.model_validate(obj)

        _obj = cls.model_validate({
            "type": obj.get("type"),
            "to": obj.get("to"),
            "amount": obj.get("amount"),
            "currency": obj.get("currency"),
            "description": obj.get("description"),
            "skip_notifications": obj.get("skip_notifications"),
            "fee": obj.get("fee"),
            "idem": obj.get("idem")
        })
        return _obj


