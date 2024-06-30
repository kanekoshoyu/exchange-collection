from __future__ import annotations
from typing import Any, Dict

class RateLimit: 
  def __init__(self, input: Dict):
    if 'rate_limit_type' in input:
      self._rate_limit_type: str = input['rate_limit_type']
    if 'interval' in input:
      self._interval: str = input['interval']
    if 'interval_num' in input:
      self._interval_num: int = input['interval_num']
    if 'limit' in input:
      self._limit: int = input['limit']
    if 'count' in input:
      self._count: int = input['count']
    if 'additional_properties' in input:
      self._additional_properties: dict[str, Any] = input['additional_properties']

  @property
  def rate_limit_type(self) -> str:
    return self._rate_limit_type
  @rate_limit_type.setter
  def rate_limit_type(self, rate_limit_type: str):
    self._rate_limit_type = rate_limit_type

  @property
  def interval(self) -> str:
    return self._interval
  @interval.setter
  def interval(self, interval: str):
    self._interval = interval

  @property
  def interval_num(self) -> int:
    return self._interval_num
  @interval_num.setter
  def interval_num(self, interval_num: int):
    self._interval_num = interval_num

  @property
  def limit(self) -> int:
    return self._limit
  @limit.setter
  def limit(self, limit: int):
    self._limit = limit

  @property
  def count(self) -> int:
    return self._count
  @count.setter
  def count(self, count: int):
    self._count = count

  @property
  def additional_properties(self) -> dict[str, Any]:
    return self._additional_properties
  @additional_properties.setter
  def additional_properties(self, additional_properties: dict[str, Any]):
    self._additional_properties = additional_properties
