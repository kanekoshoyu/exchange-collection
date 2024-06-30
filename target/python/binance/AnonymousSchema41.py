from __future__ import annotations
from typing import Any, Dict

class AnonymousSchema41: 
  def __init__(self, input: Dict):
    if 'code' in input:
      self._code: int = input['code']
    if 'msg' in input:
      self._msg: str = input['msg']
    if 'additional_properties' in input:
      self._additional_properties: dict[str, Any] = input['additional_properties']

  @property
  def code(self) -> int:
    return self._code
  @code.setter
  def code(self, code: int):
    self._code = code

  @property
  def msg(self) -> str:
    return self._msg
  @msg.setter
  def msg(self, msg: str):
    self._msg = msg

  @property
  def additional_properties(self) -> dict[str, Any]:
    return self._additional_properties
  @additional_properties.setter
  def additional_properties(self, additional_properties: dict[str, Any]):
    self._additional_properties = additional_properties
