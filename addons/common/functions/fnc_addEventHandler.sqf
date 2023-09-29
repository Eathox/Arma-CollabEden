#include "script_component.hpp"

params [
    ["_name", "", [""]],
    ["_func", nil, [{}]]
];

if (_name == "" || isNil "_func") exitWith {-1};

private _display = call FUNC(get3DENDisplay);
if (isNull _display) exitWith {ERROR("Failed to get 3DEN display")};

// Allow for single non-array param with BIS scriptedEventHandlers
_func = compile format ["(_this select 0) call %1", _func];

// Kinda slow, CBA would be nicer but we need the _display namespace
[_display, _name, _func] call BIS_fnc_addScriptedEventHandler;
