#include "script_component.hpp"

params [
    ["_section", "", [""]],
    ["_category", "", [""]]
];

private _allAttributes = [_section, _category] call FUNC(getMissionAttributeClasses);
private _properties = _allAttributes apply {_x call FUNC(getAttributeProperty)};
private _values = _properties apply {
    (_section get3DENMissionAttribute _x) params ["_value"];
    if (isNil "_value") then {
        WARNING_1("Nil attribute: '%1'.", _x); // DEBUG
        "" // TODO: figure out good default value, attributes cant store nil
    } else {
        _value
    };
};

_properties createHashMapFromArray _values
