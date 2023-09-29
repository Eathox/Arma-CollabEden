#include "script_component.hpp"

params [
    ["_entity", -1, [0, objNull, grpNull, "", []]],
    ["_category", "", [""]]
];

private _allAttributes = [_entity, _category] call FUNC(getAttributeClasses);
private _names = _allAttributes apply {_x call FUNC(getAttributeName)};
private _values = _names apply {
    (_entity get3DENAttribute _x) params ["_value"];
    if (isNil "_value") then {
        WARNING_1("nil attribute: '%1'", _x); // DEBUG
        "" // TODO: figure out good default value, attributes cant store nil
    } else {
        _value
    };
};

_names createHashMapFromArray _values
