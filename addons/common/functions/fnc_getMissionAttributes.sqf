#include "script_component.hpp"

params [
    ["_section", "", [""]],
    ["_categoryFilter", "", ["", []]]
];

private _cache = uiNamespace getVariable QGVAR(missionAttributes);

private _allCategories = _cache get toLower _section;
if (isNil "_allCategories") exitWith {[]};

if (_categoryFilter isEqualType "") then {
    _categoryFilter = if (_categoryFilter isEqualTo "") then {
        keys _allCategories;
    } else {
        [_categoryFilter];
    };
};
_categoryFilter = _categoryFilter apply {toLower _x};

private _names = [];
{
    private _category = toLower _x;
    if (_category in _categoryFilter) then {
        _names append _y;
    };
} foreach _allCategories;

private _values = _names apply {
    (_section get3DENMissionAttribute _x) params ["_value"];
    if (isNil "_value") then {WARNING_1("Nil attribute: %1.", _x)}; // DEBUG
    _value
};

_names createHashMapFromArray _values
