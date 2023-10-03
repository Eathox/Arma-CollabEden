#include "script_component.hpp"

params [
    ["_section", "", [""]]
];

private _cache = uiNamespace getVariable QGVAR(missionAttributes);

private _names = _cache get toLower _section;
if (isNil "_names") exitWith {[]};

private _values = _names apply {
    (_section get3DENMissionAttribute _x) params ["_value"];
    if (isNil "_value") then {WARNING_1("Nil attribute: %1.", _x)}; // DEBUG
    _value
};

_names createHashMapFromArray _values
