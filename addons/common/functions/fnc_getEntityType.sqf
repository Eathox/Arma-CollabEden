#include "script_component.hpp"

// returns: "OBJECT", "GROUP", "TRIGGER", "LOGIC", "WAYPOINT", "MARKER", "LAYER", "COMMENT" or nil

// Need to [_this] to make sure waypoints, i.e: [group, index] don't get split
[_this] params [
    ["_entity", -1, [0, objNull, grpNull, "", []]]
];

if (_entity isEqualType 0) then {
    _entity = get3DENEntity _entity;
};

if (_entity isEqualTo -1 || {_entity isEqualTo []}) exitWith {};
if (_entity isEqualType objNull) exitWith {
    if (_entity isKindOf "EmptyDetector") exitWith {"TRIGGER"};
    ["OBJECT", "LOGIC"] select (_entity isKindOf "logic")
};
if (_entity isEqualType grpNull) exitWith {"GROUP"};
if (_entity isEqualType []) exitWith {"WAYPOINT"};
if (_entity isEqualType "") exitWith {"MARKER"};
if (_entity isEqualType 0) exitWith {
    (_entity get3DENAttribute "position") params ["_pos"];
    ["COMMENT", "LAYER"] select isNil "_pos"
};

