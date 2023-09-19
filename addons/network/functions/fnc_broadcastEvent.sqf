#include "script_component.hpp"

params [
    ["_name", "", [""]],
    ["_params", [], []]
];

if (_name isEqualTo "") exitWith {};

private _display = call FUNC(get3DENDisplay);
if (isNull _display) exitWith {ERROR("Failed to get 3DEN display")};

private _blockEventBroadcastHash = _display getVariable QGVAR(blockEventBroadcastHash);
if (_blockEventBroadcastHash getOrDefault [_name, false]) exitWith {
    _blockEventBroadcastHash set [_name, nil];
};

systemChat format ["Sending Event: %1: %2", _name, _params]; // DEBUG
["send_event", [_name, _params]] call FUNC(callExtension);
