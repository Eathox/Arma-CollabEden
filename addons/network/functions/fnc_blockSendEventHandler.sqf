#include "script_component.hpp"

params [
    ["_name", "", [""]]
];

// WIP: This func requires further integration testing to make sure it always blocks the correct event

if (_name == "") exitWith {};

private _display = call FUNC(get3DENDisplay);
if (isNull _display) exitWith {ERROR("Failed to get 3DEN display.")};

private _blockMap = _display getVariable QGVAR(blockSendEventHandler_hashMap);
_blockMap set [_name, true]; // WIP: add timeout?
