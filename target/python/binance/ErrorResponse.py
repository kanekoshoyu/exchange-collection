from __future__ import annotations
from typing import Any, List, Dict
from . import Error
from . import RateLimit
class ErrorResponse: 
  def __init__(self, input: Dict):
    self._id: str = input['id']
    self._status: int = input['status']
    self._error: Error.Error = Error.Error(input['error'])
    if 'rate_limits' in input:
      self._rate_limits: List[RateLimit.RateLimit] = input['rate_limits']
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
  def error(self) -> Error.Error:
    return self._error
  @error.setter
  def error(self, error: Error.Error):
    self._error = error

  @property
  def rate_limits(self) -> List[RateLimit.RateLimit]:
    return self._rate_limits
  @rate_limits.setter
  def rate_limits(self, rate_limits: List[RateLimit.RateLimit]):
    self._rate_limits = rate_limits

  @property
  def additional_properties(self) -> dict[str, Any]:
    return self._additional_properties
  @additional_properties.setter
  def additional_properties(self, additional_properties: dict[str, Any]):
    self._additional_properties = additional_properties
