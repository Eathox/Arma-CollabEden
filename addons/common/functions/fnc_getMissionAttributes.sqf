#include "script_component.hpp"

params [
    ["_section", "", [""]],
    ["_category", "", [""]]
];

private _allAttributes = [_section, _category] call FUNC(getMissionAttributeClasses);
private _names = _allAttributes apply {_x call FUNC(getAttributeName)};
private _values = _names apply {
    (_section get3DENMissionAttribute _x) params ["_value"];
    if (isNil "_value") then {
        WARNING_1("nil attribute: '%1'", _x); // DEBUG
        "" // TODO: figure out good default value, attributes cant store nil
    } else {
        _value
    };
};

_names createHashMapFromArray _values
