#include "script_component.hpp"

params [
    ["_eventName", "", [""]],
    ["_params", []]
];

systemChat format ["Received Event: %1: %2", _eventName, _params]; // DEBUG

private _display = call FUNC(get3DENDisplay);
if (isNull _display) exitWith {ERROR("Failed to get 3DEN display.")};

if (_display getVariable [QGVAR(currentInstance), 0] == 0) exitWith {};

_display setVariable [QGVAR(blockSendEventHandler), true];

private _historyText = format ["Network Sync: %1", _eventName];
[_historyText, _historyText, "a3\3DEN\Data\Cfg3DEN\History\MultipleOperations_ca.paa"] collect3DENHistory {
    private _internalName = QGVAR(receivedEvent_) + _eventName;
    [_internalName, _params] call FUNC(callEventHandler);
};

_display setVariable [QGVAR(blockSendEventHandler), false];
