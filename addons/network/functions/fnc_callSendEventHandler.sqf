#include "script_component.hpp"

params [
    ["_eventName", "", [""]],
    ["_params", []]
];

private _display = call FUNC(get3DENDisplay);
if (isNull _display) exitWith {ERROR("Failed to get 3DEN display.")};

if (_display getVariable [QGVAR(currentInstance), 0] == 0) exitWith {};

private _blockSend = _display getVariable QGVAR(blockSendEventHandler);
if _blockSend exitWith {};

private _internalName = QGVAR(sendEvent_) + _eventName;
[_internalName, _params] call FUNC(callEventHandler);
