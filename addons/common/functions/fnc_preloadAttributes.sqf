#include "script_component.hpp"

private _entityCache = uiNamespace getVariable QGVAR(entityAttributes);
private _missionCache = uiNamespace getVariable QGVAR(missionAttributes);
if (!isNil "_entityCache" && !isNil "_missionCache") exitWith {
    INFO("3DEN attributes already preloaded.");
    false
};

private _cfg3DEN = configFile >> "Cfg3DEN";

if (isNil "_entityCache") then {
    _entityCache = createHashMap;

    {
        private _categories = configProperties [_x >> "AttributeCategories", "isClass _x"];
        private _allNames = flatten (_categories apply {
            private _attributes = configProperties [_x >> "Attributes", "isClass _x"];
            _attributes apply {_x call FUNC(getAttributeName)};
        });

        private _typeName = toLower (configName _x);
        _entityCache set [_typeName, _allNames];

        // Setup but don't preload specific attributes, these are cached after first use
        if (_typeName in ["object", "logic", "trigger", "waypoint", "marker"]) then {
            _entityCache set [_typeName+"#specific", createHashMap];
        };
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
};

if (isNil "_missionCache") then {
    _missionCache = createHashMap;

    {
        private _categories = configProperties [_x >> "AttributeCategories", "isClass _x"];
        private _allNames = flatten (_categories apply {
            private _attributes = configProperties [_x >> "Attributes", "isClass _x"];
            _attributes apply {_x call FUNC(getAttributeName)};
        });

        private _sectionName = toLower (configName _x);
        _missionCache set [_sectionName, _allNames];
    } foreach ("true" configClasses (_cfg3DEN >> "Mission"));
};

uiNamespace setVariable [QGVAR(entityAttributes), _entityCache];
uiNamespace setVariable [QGVAR(missionAttributes), _missionCache];

true
