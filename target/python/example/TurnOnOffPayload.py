from __future__ import annotations
from typing import Any, Dict
from . import AnonymousSchema6
class TurnOnOffPayload: 
  def __init__(self, input: Dict):
    if 'command' in input:
      self._command: AnonymousSchema6.AnonymousSchema6 = AnonymousSchema6.AnonymousSchema6(input['command'])
    if 'sent_at' in input:
      self._sent_at: str = input['sent_at']
    if 'additional_properties' in input:
      self._additional_properties: dict[str, Any] = input['additional_properties']

  @property
  def command(self) -> AnonymousSchema6.AnonymousSchema6:
    return self._command
  @command.setter
  def command(self, command: AnonymousSchema6.AnonymousSchema6):
    self._command = command

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
