from __future__ import annotations
from typing import Any, List, Dict
from . import AnonymousSchema15
from . import AnonymousSchema32
class AnonymousSchema12: 
  def __init__(self, input: Dict):
    self._id: str = input['id']
    self._status: int = input['status']
    if 'result' in input:
      self._result: AnonymousSchema15.AnonymousSchema15 = AnonymousSchema15.AnonymousSchema15(input['result'])
    if 'rate_limits' in input:
      self._rate_limits: List[AnonymousSchema32.AnonymousSchema32] = input['rate_limits']
    if 'additional_properties' in input:
      self._additional_properties: dict[str, Any] = input['additional_properties']

  @property
  def id(self) -> str:
    return self._id
  @id.setter
  def id(self, id: str):
    self._id = id

  @property
  def status(self) -> int:
    return self._status
  @status.setter
  def status(self, status: int):
    self._status = status

  @property
  def result(self) -> AnonymousSchema15.AnonymousSchema15:
    return self._result
  @result.setter
  def result(self, result: AnonymousSchema15.AnonymousSchema15):
    self._result = result

  @property
  def rate_limits(self) -> List[AnonymousSchema32.AnonymousSchema32]:
    return self._rate_limits
  @rate_limits.setter
  def rate_limits(self, rate_limits: List[AnonymousSchema32.AnonymousSchema32]):
    self._rate_limits = rate_limits

  @property
  def additional_properties(self) -> dict[str, Any]:
    return self._additional_properties
  @additional_properties.setter
  def additional_properties(self, additional_properties: dict[str, Any]):
    self._additional_properties = additional_properties
