#include "script_component.hpp"

params [
    ["_name", "", [""]],
    ["_params", []]
];

private _display = call FUNC(get3DENDisplay);
if (isNull _display) exitWith {ERROR("Failed to get 3DEN display.")};

if (_display getVariable [QGVAR(currentInstance), 0] == 0) exitWith {};

private _blockMap = _display getVariable QGVAR(blockSendEventHandler_hashMap);
if (_blockMap getOrDefault [_name, false]) exitWith {
    _blockMap set [_name, nil];
};

private _internalName = QGVAR(sendEvent_) + _name;
[_internalName, [_name, _params]] call FUNC(callEventHandler);
