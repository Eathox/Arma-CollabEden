#include "script_component.hpp"

params [
    ["_name", "", [""]]
];

if (_name isEqualTo "") exitWith {};

private _display = call FUNC(get3DENDisplay);
if (isNull _display) exitWith {ERROR("Failed to get 3DEN display")};

private _blockSyncHash = _display getVariable QGVAR(blockEventBroadcastHash);
_blockSyncHash set [_name, true]; // WIP: add timeout?

