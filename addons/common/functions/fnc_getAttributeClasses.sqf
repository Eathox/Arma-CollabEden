#include "script_component.hpp"

params [
    ["_entity", -1, [0, objNull, grpNull, "", []]],
    ["_category", "", [""]]
];

private _entityType = toUpper (_entity call FUNC(getEntityType));
if (isNil "_entityType") exitWith {[]};

private _includeAll = (_category == "");
private _includeSpecific = (_includeAll || _category == "#specific");
private _attributes = [];

if (_includeAll || !_includeSpecific) then {
    private _categoriesConfig = configFile >> "Cfg3DEN" >> _entityType >> "AttributeCategories";
    private _categories = if _includeAll then {
        configProperties [_categoriesConfig, "isClass _x"];
    } else {
        [_categoriesConfig >> _category];
    };

    private _generalAttributes = _categories apply {
        configProperties [_x >> "Attributes", "isClass _x"];
    };
    _attributes append flatten _generalAttributes;
};

private _hasSpecific = toUpper _entityType in ["OBJECT", "LOGIC", "TRIGGER", "WAYPOINT", "MARKER"];
if (_includeSpecific && _hasSpecific) then {
    private _entityConfig = switch _entityType do {
        case "WAYPOINT": {
            (_entity get3DENAttribute "ItemClass") params ["_class"];
            // Search through CfgWaypoints categories and find first match. TODO: prioritize vanilla categories?
            private _waypointCategories = configProperties [configFile >> "CfgWaypoints", "isClass _x"];
            private _findIndex = _waypointCategories findIf {isClass (_x >> _class)};
            _waypointCategories select _findIndex >> _class;
        };
        case "MARKER": {
            (_entity get3DENAttribute "markerType") params ["_markerType"];
            if (_markerType isNotEqualTo -1) exitWith {configNull}; // Area markers can't have specific attributes

            (_entity get3DENAttribute "ItemClass") params ["_class"];
            configFile >> "CfgMarkers" >> _class;
        };
        default {
            if (_entity isEqualType 0) then {
                _entity = get3DENEntity _entity;
            };
            configOf _entity
        };
    };

    private _specificAttributes = configProperties [_entityConfig >> "Attributes", "isClass _x"];
    if (_specificAttributes isEqualTo []) exitWith {};
    // Some specific attributes are controls and not actual attributes
    _specificAttributes = _specificAttributes select {isText (_x >> "property") || isText (_x >> "data")};
    _attributes append _specificAttributes;
};

_attributes
