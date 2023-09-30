#include "script_component.hpp"

params [
    ["_attribute", configNull, [configNull]]
];

if (isNull _attribute) exitWith {};

private _parents = configHierarchy _attribute;
private _parentName = configName (_parents select -2);

// Some modules use arguments as attributes
if (_parentName == "arguments") then {
    private _className = configName (_parents select -3);
    _className + "_" + configName _attribute;
} else {
    // Engine attributes use data
    private _property = ["property", "data"] select isText (_attribute >> "data");
    getText (_attribute >> _property);
};

