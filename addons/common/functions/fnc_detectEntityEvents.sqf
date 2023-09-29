#include "script_component.hpp"
// There is no EH for entity created.
// OnDeleteUnits doesn't include the deleted entities, and it fires after the entities are deleted.
// So instead we have to detect and fire entity created/deleted ourself.

private _display = call FUNC(get3DENDisplay);
if (isNull _display) exitWith {ERROR("Failed to get 3DEN display")};

(_display getVariable QGVAR(detectEntityEvents_prevEntities)) params [
    ["_prevObjects", []],
    ["_prevGroups", []],
    ["_prevTriggers", []],
    ["_prevLogics", []],
    ["_prevWaypoints", []],
    ["_prevMarkers", []],
    ["_prevLayers", []],
    ["_prevComments", [-999]] // Dunno why it starts with -999
];

// Has to be ID's so we can still send the correct ID even if the entity is deleted.
private _curEntities = all3DENEntities apply {_x apply {get3DENEntityID _x}};
_display setVariable [QGVAR(detectEntityEvents_prevEntities), _curEntities];
_curEntities params [
    "_curObjects",
    "_curGroups",
    "_curTriggers",
    "_curLogics",
    "_curWaypoints",
    "_curMarkers",
    "_curLayers",
    "_curComments"
];

private ["_deleted", "_created"];
private _fireEvents = {
    params ["_cur", "_prev", "_typeName"];
    private _shared = _cur arrayIntersect _prev;
    _deleted = _prev - _shared;
    _created = _cur - _shared;

    private _deletedEHName = format [QGVAR(%1Deleted), _typeName];
    private _createdEHName = format [QGVAR(%1Created), _typeName];
    {[_deletedEHName, _x] call FUNC(callEventHandler)} foreach _deleted;
    {[_createdEHName, _x] call FUNC(callEventHandler)} foreach _created;
};

// Still has issues with detecting layer creation, tracking issue: https://feedback.bistudio.com/T175680
[_curLayers, _prevLayers, "layer"] call _fireEvents;

[_curMarkers, _prevMarkers, "marker"] call _fireEvents;
[_curLogics, _prevLogics, "logic"] call _fireEvents;
[_curTriggers, _prevTriggers, "trigger"] call _fireEvents;
[_curGroups, _prevGroups, "group"] call _fireEvents;
[_curObjects, _prevObjects, "object"] call _fireEvents;
[_curWaypoints, _prevWaypoints, "waypoint"] call _fireEvents;
[_curComments, _prevComments, "comment"] call _fireEvents;
