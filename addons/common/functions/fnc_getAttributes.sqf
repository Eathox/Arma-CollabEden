#include "script_component.hpp"

params [
    ["_entity", -1, [0, objNull, grpNull, "", []]],
    ["_categoryFilter", "", ["", []]]
];

private _cache = uiNamespace getVariable QGVAR(entityAttributes);
private _entityType = _entity call FUNC(getEntityType);

private _allCategories = _cache get _entityType;
if (isNil "_allCategories") exitWith {[]};

if (_categoryFilter isEqualType "") then {
    _categoryFilter = if (_categoryFilter isEqualTo "") then {
        keys _allCategories;
    } else {
        [_categoryFilter];
    };
};
_categoryFilter = _categoryFilter apply {toLower _x};

private _specificCache = _allCategories get "#specific";
private _includeSpecific = "#specific" in _categoryFilter;
if _includeSpecific then {
    _categoryFilter = _categoryFilter - ["#specific"];
};

private _names = [];
{
    private _category = toLower _x;
    if (_category in _categoryFilter) then {
        _names append _y;
    };
} foreach _allCategories;

private _hasSpecific = _entityType in ["object", "logic", "trigger", "waypoint", "marker"];
if (_includeSpecific && _hasSpecific) then {
    (_entity get3DENAttribute "ItemClass") params ["_class"];

    private _specificAttributes = _specificCache get _class;
    if !(isNil "_specificAttributes") exitWith {
        _names append _specificAttributes;
    };

    private _entityConfig = switch _entityType do {
        case "waypoint": {
            private _waypointCategories = configProperties [configFile >> "CfgWaypoints", "isClass _x"];
            private _findIndex = _waypointCategories findIf {isClass (_x >> _class)};

            _waypointCategories select _findIndex >> _class;
        };
        case "marker": {
            (_entity get3DENAttribute "markerType") params ["_markerType"];
            if (_markerType isNotEqualTo -1) exitWith {configNull}; // Area markers can't have specific attributes

            configFile >> "CfgMarkers" >> _class;
        };
        case "trigger": {configFile >> "CfgNonAIVehicles" >> _class};
        default {configFile >> "CfgVehicles" >> _class};
    };

    _specificAttributes = configProperties [_entityConfig >> "Attributes", "isClass _x"];
    if (_entityType == "logic") then {
        _specificAttributes = if (_specificAttributes isEqualTo []) then {
            // Some (older) modules have no attributes they use arguments instead
            configProperties [_entityConfig >> "Arguments", "isClass _x"];
        } else {
            // Modules which have both attributes and arguments tend to have controls mixed in with the attributes
            _specificAttributes select {isText (_x >> "property") || isText (_x >> "data")};
        };
    };
    _specificAttributes = _specificAttributes apply {_x call FUNC(getAttributeName)};

    _specificCache set [_class, _specificAttributes];
    _names append _specificAttributes;
};

private _values = _names apply {
    (_entity get3DENAttribute _x) params ["_value"];
    if (isNil "_value") then {WARNING_1("Nil attribute: %1.", _x)}; // DEBUG
    _value
};

_names createHashMapFromArray _values
