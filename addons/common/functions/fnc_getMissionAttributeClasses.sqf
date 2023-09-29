#include "script_component.hpp"

params [
    ["_section", "", [""]],
    ["_category", "", [""]]
];

private _sectionsConfig = configFile >> "Cfg3DEN" >> "Mission" >> _section;
if (isNull _sectionsConfig) exitWith {[]};

private _categoriesConfig = _sectionsConfig >> "AttributeCategories";
private _categories = if (_category == "") then {
    configProperties [_categoriesConfig, "isClass _x"];
} else {
    [_categoriesConfig >> _category]
};

private _attributes = _categories apply {
    configProperties [_x >> "Attributes", "isClass _x"];
};
flatten _attributes
