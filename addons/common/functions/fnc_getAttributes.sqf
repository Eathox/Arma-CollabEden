#include "script_component.hpp"

params [
    ["_entity", -1, [0, objNull, grpNull, "", []]],
    ["_category", "", [""]]
];

private _allAttributes = [_entity, _category] call FUNC(getAttributeClasses);
private _properties = _allAttributes apply {_x call FUNC(getAttributeProperty)};
private _values = _properties apply {
    (_entity get3DENAttribute _x) params ["_value"];
    if (isNil "_value") then {
        WARNING_1("Nil attribute: '%1'.", _x); // DEBUG
        "" // TODO: figure out good default value, attributes cant store nil
    } else {
        _value
    };
};

_properties createHashMapFromArray _values
