from __future__ import annotations
from typing import Any, Dict
from . import Side
from . import OrderType
class SendRequestParam: 
  def __init__(self, input: Dict):
    if 'symbol' in input:
      self._symbol: str = input['symbol']
    if 'side' in input:
      self._side: Side.Side = Side.Side(input['side'])
    if 'type' in input:
      self._type: OrderType.OrderType = OrderType.OrderType(input['type'])
    if 'price' in input:
      self._price: str = input['price']
    if 'quantity' in input:
      self._quantity: str = input['quantity']
    if 'time_in_force' in input:
      self._time_in_force: str = input['time_in_force']
    if 'timestamp' in input:
      self._timestamp: int = input['timestamp']
    if 'api_key' in input:
      self._api_key: str = input['api_key']
    if 'signature' in input:
      self._signature: str = input['signature']
    if 'additional_properties' in input:
      self._additional_properties: dict[str, Any] = input['additional_properties']

  @property
  def symbol(self) -> str:
    return self._symbol
  @symbol.setter
  def symbol(self, symbol: str):
    self._symbol = symbol

  @property
  def side(self) -> Side.Side:
    return self._side
  @side.setter
  def side(self, side: Side.Side):
    self._side = side

  @property
  def type(self) -> OrderType.OrderType:
    return self._type
  @type.setter
  def type(self, type: OrderType.OrderType):
    self._type = type

  @property
  def price(self) -> str:
    return self._price
  @price.setter
  def price(self, price: str):
    self._price = price

  @property
  def quantity(self) -> str:
    return self._quantity
  @quantity.setter
  def quantity(self, quantity: str):
    self._quantity = quantity

  @property
  def time_in_force(self) -> str:
    return self._time_in_force
  @time_in_force.setter
  def time_in_force(self, time_in_force: str):
    self._time_in_force = time_in_force

  @property
  def timestamp(self) -> int:
    return self._timestamp
  @timestamp.setter
  def timestamp(self, timestamp: int):
    self._timestamp = timestamp

  @property
  def api_key(self) -> str:
    return self._api_key
  @api_key.setter
  def api_key(self, api_key: str):
    self._api_key = api_key

  @property
  def signature(self) -> str:
    return self._signature
  @signature.setter
  def signature(self, signature: str):
    self._signature = signature

  @property
  def additional_properties(self) -> dict[str, Any]:
    return self._additional_properties
  @additional_properties.setter
  def additional_properties(self, additional_properties: dict[str, Any]):
    self._additional_properties = additional_properties
