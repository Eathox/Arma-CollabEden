#include "script_component.hpp"

INFO("Validating attribute compatibility.");

private _timeStart = diag_tickTime;
private _cfg3DEN = configFile >> "Cfg3DEN";

private _formatAttributePath = {
    private _parents = configHierarchy _this;
    private _parentName = configName (_parents select -3);
    format ["%1->%2", _parentName, configName _this];
};

private _getAttributeControlConfig = {
    private _controlClass = getText (_this >> "Control");
    _cfg3DEN >> "Attributes" >> _controlClass;
};

// Duplicate attribute properties
private _duplicateCheck = {
    params ["_typeName", "_attributes", ["_knownMap", createHashMap]];

    {
        private _controlConfig = _x call _getAttributeControlConfig;
        private _checkDuplicate = getNumber (_controlConfig >> QGVARMAIN(skipDuplicateCheck));
        if (_checkDuplicate == 1) then {continue};

        private _property = _x call FUNC(getAttributeProperty);
        private _lowerProperty = toLower _property;

        if !(_lowerProperty in _knownMap) then {
            _knownMap set [_lowerProperty, _x];
        } else {
            private _knownCfg = _knownMap get _lowerProperty;
            ERROR_4(
                "Property %1-%2 is used by multiple attributes! %3 & %4",
                _typeName,
                _property,
                _knownCfg call _formatAttributePath,
                _x call _formatAttributePath
            );
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

{
    _x params ["_generalConfig", "_classes"];

    private _categories = configProperties [_generalConfig >> "AttributeCategories", "isClass _x"];
    private _generalAttributes = _categories apply {
        configProperties [_x >> "Attributes", "isClass _x"];
    };
    _generalAttributes = flatten _generalAttributes;

    // General attributes
    [configName _generalConfig, _generalAttributes] call _duplicateCheck;

    // Specific attributes ignoring general attributes conflicts
    if (_classes isEqualTo []) then {continue};
    private _knownMap = createHashMapFromArray (_generalAttributes apply {
        private _property = _x call FUNC(getAttributeProperty);
        [toLower _property, _x]
    });

    private _isLogic = (_generalConfig == (_cfg3DEN >> "Logic"));
    {
        private _specificAttributes = configProperties [_x >> "Attributes", "isClass _x"];
        private _hasNoAttributes = (_specificAttributes isEqualTo []);
        if (!_isLogic && _hasNoAttributes) then {continue};

        if _isLogic then {
            if _hasNoAttributes then {
                // Some (older) modules have no attributes they use arguments instead
                _specificAttributes = configProperties [_x >> "Arguments", "isClass _x"];
            } else {
                // Some module attributes are controls and not actual attributes
                _specificAttributes = _specificAttributes select {isText (_x >> "property") || isText (_x >> "data")};
            };
        };
        [configName _x, _specificAttributes, +_knownMap] call _duplicateCheck;
    } foreach _classes;
} foreach [
    _cfg3DEN >> "Group",
    _cfg3DEN >> "Layer",
    _cfg3DEN >> "Comment",
    [_cfg3DEN >> "Object", _nonLogicClasses],
    [_cfg3DEN >> "Logic", _logicClasses],
    [_cfg3DEN >> "Waypoint", _waypointClasses],
    [_cfg3DEN >> "Trigger", "true" configClasses (configFile >> "CfgNonAIVehicles")],
    [_cfg3DEN >> "Marker", "true" configClasses (configFile >> "CfgMarkers")]
] + ("true" configClasses (_cfg3DEN >> "Mission"));

// Missing argument mirroring
{
    private _arguments = configProperties [_x >> "Arguments", "isClass _x"];
    if (_arguments isEqualTo []) then {continue};

    private _attributes = configProperties [_x >> "Attributes", "isClass _x"];
    if (_attributes isEqualTo []) then {continue};

    // These modules tend to have controls mixed in with attributes, dunno why
    _attributes = _attributes select {isText (_x >> "property") || isText (_x >> "data")};

    private _moduleName = configName _x;
    {
        private _argumentClass = configName _x;
        private _argumentProperty = _x call FUNC(getAttributeProperty);
        private _attributeMirrorIndex = _attributes findIf {
            _argumentClass == configName _x || {_argumentProperty == _x call FUNC(getAttributeProperty)}
        };

        if (_attributeMirrorIndex == -1) then {
            WARNING_2("Module %1 uses attributes but is missing mirror for: %2 argument!", _moduleName, _argumentClass);
        };
    } foreach _arguments;
} foreach _logicClasses;

private _duration = round ((diag_tickTime - _timeStart) * 1000);
INFO_1("Finished validating. Time: %1 ms", _duration);
