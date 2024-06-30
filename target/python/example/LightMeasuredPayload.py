from __future__ import annotations
from typing import Any, Dict

class LightMeasuredPayload: 
  def __init__(self, input: Dict):
    if 'lumens' in input:
      self._lumens: int = input['lumens']
    if 'sent_at' in input:
      self._sent_at: str = input['sent_at']
    if 'additional_properties' in input:
      self._additional_properties: dict[str, Any] = input['additional_properties']

  @property
  def lumens(self) -> int:
    return self._lumens
  @lumens.setter
  def lumens(self, lumens: int):
    self._lumens = lumens

  @property
  def sent_at(self) -> str:
    return self._sent_at
  @sent_at.setter
  def sent_at(self, sent_at: str):
    self._sent_at = sent_at

  @property
  def additional_properties(self) -> dict[str, Any]:
    return self._additional_properties
  @additional_properties.setter
  def additional_properties(self, additional_properties: dict[str, Any]):
    self._additional_properties = additional_properties
