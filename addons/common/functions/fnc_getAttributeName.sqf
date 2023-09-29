#include "script_component.hpp"

params [
    ["_attribute", configNull, [configNull]]
];

if (isNull _attribute) exitWith {};

private _property = ["data", "property"] select isText (_this >> "property");
toLower (getText (_this >> _property))
