#include "script_component.hpp"

private _formatAttributePath = {
    private _parents = configHierarchy _this;
    private _parentName = configName (_parents select -3);
    format ["%1->%2", _parentName, configName _this];
};

private _getAttributeControlConfig = {
    private _controlClass = getText (_this >> "Control");
    configFile >> "Cfg3DEN" >> "Attributes" >> _controlClass;
};

private _duplicateCheck = {
    params ["_typeName", "_attributes", ["_knownMap", createHashMap]];

    {
        private _controlConfig = _x call _getAttributeControlConfig;
        private _skipCheckDuplicate = getNumber (_controlConfig >> QGVARMAIN(skipDuplicateCheck));

        private _name = _x call FUNC(getAttributeName);
        private _lowerName = toLower _name;

        if (_lowerName in _knownMap && _skipCheckDuplicate != 1) then {
            private _knownConfig = _knownMap get _lowerName;
            ERROR_4(
                "Attribute %1-%2 is used by multiple attributes! %3 & %4",
                _typeName,
                _name,
                _knownConfig call _formatAttributePath,
                _x call _formatAttributePath
                );
        } else {
            _knownMap set [_lowerName, _x];
        };
    } foreach _attributes;
};

private _cfgVehicleClasses = "true" configClasses (configFile >> "CfgVehicles");
private _logicClasses = _cfgVehicleClasses select {configName _x isKindOf "Logic"};
private _nonLogicClasses = _cfgVehicleClasses - _logicClasses;

private _waypointClasses = "true" configClasses (configFile >> "CfgWaypoints");
_waypointClasses = flatten (_waypointClasses apply {
    configProperties [_x, "isClass _x"];
});

private _cfg3DEN = configFile >> "Cfg3DEN";
{
    _x params ["_generalConfig", "_classes"];

    private _categories = configProperties [_generalConfig >> "AttributeCategories", "isClass _x"];
    private _generalAttributes = flatten (_categories apply {
        configProperties [_x >> "Attributes", "isClass _x"];
    });

    // General attributes
    [configName _generalConfig, _generalAttributes] call _duplicateCheck;

    // Specific attributes ignoring general attributes conflicts
    if (_classes isEqualTo []) then {continue};
    private _knownMap = createHashMapFromArray (_generalAttributes apply {
        private _name = _x call FUNC(getAttributeName);
        [toLower _name, _x]
    });

    private _isLogic = (_generalConfig == (_cfg3DEN >> "Logic"));
    {
        private _specificAttributes = configProperties [_x >> "Attributes", "isClass _x"];
        private _hasNoAttributes = (_specificAttributes isEqualTo []);
        if (!_isLogic && _hasNoAttributes) then {continue};

        if _isLogic then {
            _specificAttributes = if _hasNoAttributes then {
                // Some (older) modules have no attributes they use arguments instead
                configProperties [_x >> "Arguments", "isClass _x"];
            } else {
                // Modules which have both attributes and arguments tend to have controls mixed in with the attributes
                _specificAttributes select {isText (_x >> "property") || isText (_x >> "data")};
            };
        };
        [configName _x, _specificAttributes, +_knownMap] call _duplicateCheck;
    } foreach _classes;
} foreach [
    [_cfg3DEN >> "Object", _nonLogicClasses],
    _cfg3DEN >> "Group",
    [_cfg3DEN >> "Trigger", "true" configClasses (configFile >> "CfgNonAIVehicles")],
    [_cfg3DEN >> "Logic", _logicClasses],
    [_cfg3DEN >> "Waypoint", _waypointClasses],
    [_cfg3DEN >> "Marker", "true" configClasses (configFile >> "CfgMarkers")],
    _cfg3DEN >> "Layer",
    _cfg3DEN >> "Comment"
] + ("true" configClasses (_cfg3DEN >> "Mission"));
