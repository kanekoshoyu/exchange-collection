from __future__ import annotations
from typing import Any, Dict
from . import AnonymousSchema4
class AnonymousSchema1: 
  def __init__(self, input: Dict):
    self._id: str = input['id']
    self._method: str = input['method']
    if 'params' in input:
      self._params: AnonymousSchema4.AnonymousSchema4 = AnonymousSchema4.AnonymousSchema4(input['params'])
    if 'additional_properties' in input:
      self._additional_properties: dict[str, Any] = input['additional_properties']

  @property
  def id(self) -> str:
    return self._id
  @id.setter
  def id(self, id: str):
    self._id = id

  @property
  def method(self) -> str:
    return self._method
  @method.setter
  def method(self, method: str):
    self._method = method

  @property
  def params(self) -> AnonymousSchema4.AnonymousSchema4:
    return self._params
  @params.setter
  def params(self, params: AnonymousSchema4.AnonymousSchema4):
    self._params = params

  @property
  def additional_properties(self) -> dict[str, Any]:
    return self._additional_properties
  @additional_properties.setter
  def additional_properties(self, additional_properties: dict[str, Any]):
    self._additional_properties = additional_properties
