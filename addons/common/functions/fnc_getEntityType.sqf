#include "script_component.hpp"

// returns: "object", "group", "trigger", "logic", "waypoint", "marker", "layer", "comment" or nil

// Need to [_this] to make sure waypoints, i.e: [group, index] don't get split
[_this] params [
    ["_entity", -1, [0, objNull, grpNull, "", []]]
];

if (_entity isEqualType 0) then {
    _entity = get3DENEntity _entity;
};

if (_entity isEqualTo -1 || {_entity isEqualTo []}) exitWith {};
if (_entity isEqualType objNull) exitWith {
    if (_entity isKindOf "EmptyDetector") exitWith {"trigger"};
    ["object", "logic"] select (_entity isKindOf "logic")
};
if (_entity isEqualType grpNull) exitWith {"group"};
if (_entity isEqualType []) exitWith {"waypoint"};
if (_entity isEqualType "") exitWith {"marker"};
if (_entity isEqualType 0) exitWith {
    (_entity get3DENAttribute "position") params ["_pos"];
    ["comment", "layer"] select isNil "_pos"
};

