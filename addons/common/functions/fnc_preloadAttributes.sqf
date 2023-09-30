#include "script_component.hpp"

private _cache = uiNamespace getVariable QGVAR(entityAttributes);
if !(isNil "_cache") exitWith {
    INFO("3DEN entity attributes already preloaded.");
    false
};

private _timeStart = diag_tickTime;
_cache = createHashMap;

private _cfg3DEN = configFile >> "Cfg3DEN";
{
    private _typeCache = createHashMap;
    private _categories = configProperties [_x >> "AttributeCategories", "isClass _x"];
    {
        private _attributes = configProperties [_x >> "Attributes", "isClass _x"];
        private _names = _attributes apply {_x call FUNC(getAttributeName)};
        _typeCache set [toLower (configName _x), _names];
    } foreach _categories;

    private _typeName = toLower (configName _x);

    // Setup but don't preload specific attributes, these are cached after first use
    if (_typeName in ["object", "logic", "trigger", "waypoint", "marker"]) then {
        _typeCache set ["#specific", createHashMap];
    };
    _cache set [_typeName, _typeCache];
} foreach [
    _cfg3DEN >> "Object",
    _cfg3DEN >> "Group",
    _cfg3DEN >> "Trigger",
    _cfg3DEN >> "Logic",
    _cfg3DEN >> "Waypoint",
    _cfg3DEN >> "Marker",
    _cfg3DEN >> "Layer",
    _cfg3DEN >> "Comment"
];

uiNamespace setVariable [QGVAR(entityAttributes), _cache];

private _duration = round ((diag_tickTime - _timeStart) * 1000);
INFO_1("Preloaded 3DEN entity attributes. Time: %1 ms", _duration);

true
