#include "script_component.hpp"

params [
    ["_name", "", [""]],
    ["_params", [], []]
];

if (_name isEqualTo "") exitWith {ERROR("No event name specified")};

private _display = call FUNC(get3DENDisplay);
if (isNull _display) exitWith {ERROR("Failed to get 3DEN display")};

// Allow for single no array param with BIS scriptedEventHandlers
_params = [_params];

// Kinda slow, CBA would be nicer but we need the _display namespace
[_display, _name, _params] call BIS_fnc_callScriptedEventHandler;

nil
