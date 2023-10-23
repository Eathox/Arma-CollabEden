#include "script_component.hpp"

params [
    ["_entity", -1, [0, objNull, grpNull, "", []]]
];

private _cache = uiNamespace getVariable QGVAR(entityAttributes);
private _entityType = toLower (_entity call FUNC(getEntityType));

private _names = _cache get _entityType;
if (isNil "_names") exitWith {[]};

if (_entityType in ["object", "logic", "trigger", "waypoint", "marker"]) then {
    (_entity get3DENAttribute "ItemClass") params ["_className"];

    private _specificCache = _cache get _entityType + "#specific";
    private _specificAttributes = _specificCache get _className;
    if !(isNil "_specificAttributes") exitWith {
        _names = _names + _specificAttributes;
    };

    private _entityConfig = switch _entityType do {
        case "waypoint": {
            private _waypointCategories = configProperties [configFile >> "CfgWaypoints", toString {isClass _x}];
            private _findIndex = _waypointCategories findIf {isClass (_x >> _className)};

            _waypointCategories select _findIndex >> _className;
        };
        case "marker": {
            if (_className == "") exitWith {configNull}; // Area markers can't have specific attributes
            configFile >> "CfgMarkers" >> _className;
        };
        case "trigger": {configFile >> "CfgNonAIVehicles" >> _className};
        default {configFile >> "CfgVehicles" >> _className};
    };

    _specificAttributes = configProperties [_entityConfig >> "Attributes", toString {isClass _x}];
    if (_entityType == "logic") then {
        if (_specificAttributes isEqualTo []) then {
            // Some (older) modules have no attributes they use arguments instead
            _specificAttributes = configProperties [_entityConfig >> "Arguments", toString {isClass _x}];
        } else {
            // Modules which have both attributes and arguments tend to have controls mixed in with the attributes
            _specificAttributes = _specificAttributes select {isText (_x >> "property") || isText (_x >> "data")};
        };
    };
    _specificAttributes = _specificAttributes apply {_x call FUNC(getAttributeName)};

    _specificCache set [_className, _specificAttributes];
    _names = _names + _specificAttributes;
};

private _values = _names apply {
    (_entity get3DENAttribute _x) params ["_value"];
    if (isNil "_value") then {
        // Nil values aren't valid; cant be set or send
        WARNING_1("Nil attribute: %1, using """" instead.", _x);
        _value = "";
    };
    _value
};

_names createHashMapFromArray _values
