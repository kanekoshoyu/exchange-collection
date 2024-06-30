from __future__ import annotations
from typing import Any, Dict
from . import Side
from . import OrderType
class SuccessResponseResult: 
  def __init__(self, input: Dict):
    if 'symbol' in input:
      self._symbol: str = input['symbol']
    if 'order_id' in input:
      self._order_id: int = input['order_id']
    if 'order_list_id' in input:
      self._order_list_id: int = input['order_list_id']
    if 'client_order_id' in input:
      self._client_order_id: str = input['client_order_id']
    if 'transact_time' in input:
      self._transact_time: int = input['transact_time']
    if 'price' in input:
      self._price: str = input['price']
    if 'orig_qty' in input:
      self._orig_qty: str = input['orig_qty']
    if 'executed_qty' in input:
      self._executed_qty: str = input['executed_qty']
    if 'cummulative_quote_qty' in input:
      self._cummulative_quote_qty: str = input['cummulative_quote_qty']
    if 'status' in input:
      self._status: str = input['status']
    if 'time_in_force' in input:
      self._time_in_force: str = input['time_in_force']
    if 'side' in input:
      self._side: Side.Side = Side.Side(input['side'])
    if 'type' in input:
      self._type: OrderType.OrderType = OrderType.OrderType(input['type'])
    if 'working_time' in input:
      self._working_time: int = input['working_time']
    if 'self_trade_prevention_mode' in input:
      self._self_trade_prevention_mode: str = input['self_trade_prevention_mode']
    if 'additional_properties' in input:
      self._additional_properties: dict[str, Any] = input['additional_properties']

  @property
  def symbol(self) -> str:
    return self._symbol
  @symbol.setter
  def symbol(self, symbol: str):
    self._symbol = symbol

  @property
  def order_id(self) -> int:
    return self._order_id
  @order_id.setter
  def order_id(self, order_id: int):
    self._order_id = order_id

  @property
  def order_list_id(self) -> int:
    return self._order_list_id
  @order_list_id.setter
  def order_list_id(self, order_list_id: int):
    self._order_list_id = order_list_id

  @property
  def client_order_id(self) -> str:
    return self._client_order_id
  @client_order_id.setter
  def client_order_id(self, client_order_id: str):
    self._client_order_id = client_order_id

  @property
  def transact_time(self) -> int:
    return self._transact_time
  @transact_time.setter
  def transact_time(self, transact_time: int):
    self._transact_time = transact_time

  @property
  def price(self) -> str:
    return self._price
  @price.setter
  def price(self, price: str):
    self._price = price

  @property
  def orig_qty(self) -> str:
    return self._orig_qty
  @orig_qty.setter
  def orig_qty(self, orig_qty: str):
    self._orig_qty = orig_qty

  @property
  def executed_qty(self) -> str:
    return self._executed_qty
  @executed_qty.setter
  def executed_qty(self, executed_qty: str):
    self._executed_qty = executed_qty

  @property
  def cummulative_quote_qty(self) -> str:
    return self._cummulative_quote_qty
  @cummulative_quote_qty.setter
  def cummulative_quote_qty(self, cummulative_quote_qty: str):
    self._cummulative_quote_qty = cummulative_quote_qty

  @property
  def status(self) -> str:
    return self._status
  @status.setter
  def status(self, status: str):
    self._status = status

  @property
  def time_in_force(self) -> str:
    return self._time_in_force
  @time_in_force.setter
  def time_in_force(self, time_in_force: str):
    self._time_in_force = time_in_force

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
  def working_time(self) -> int:
    return self._working_time
  @working_time.setter
  def working_time(self, working_time: int):
    self._working_time = working_time

  @property
  def self_trade_prevention_mode(self) -> str:
    return self._self_trade_prevention_mode
  @self_trade_prevention_mode.setter
  def self_trade_prevention_mode(self, self_trade_prevention_mode: str):
    self._self_trade_prevention_mode = self_trade_prevention_mode

  @property
  def additional_properties(self) -> dict[str, Any]:
    return self._additional_properties
  @additional_properties.setter
  def additional_properties(self, additional_properties: dict[str, Any]):
    self._additional_properties = additional_properties
