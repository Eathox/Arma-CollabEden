#include "script_component.hpp"
// There is no EH for entity created.
// OnDeleteUnits doesn't include the deleted entities, and it fires after the entities are deleted.
// So instead we have to detect and fire entity created/deleted ourself.

private _display = call FUNC(get3DENDisplay);
if (isNull _display) exitWith {ERROR("Failed to get 3DEN display")};

// Get all entities ID's. Has to be ID's so we can still send the correct ID even if the entity is deleted.
private _curEntities = all3DENEntities apply {_x apply {get3DENEntityID _x}};
_curEntities params ["_curObjects", "_curGroups", "_curTriggers", "_curSystems", "_curWaypoints", "_curMarkers", "_curLayers", "_curComments"];
(_display getVariable QGVAR(detectEntityEventsPrev)) params [
    ["_prevObjects", []],
    ["_prevGroups", []],
    ["_prevTriggers", []],
    ["_prevSystems", []],
    ["_prevWaypoints", []],
    ["_prevMarkers", []],
    ["_prevLayers", []],
    ["_prevComments", [-999]] // Dunno why it starts with -999
];

private ["_deleted", "_added"];
private _detectChange = {
    params ["_cur", "_prev", "_typeName"];
    _deleted = _prev - _cur;
    _added = _cur - _prev;

    private _deletedEHName = format [QGVAR(%1Deleted), _typeName];
    private _createdEHName = format [QGVAR(%1Created), _typeName];
    {[_deletedEHName, _x] call FUNC(callEventHandler)} foreach _deleted;
    {[_createdEHName, _x] call FUNC(callEventHandler)} foreach _added;
};

[_curMarkers, _prevMarkers, "marker"] call _detectChange;
[_curSystems, _prevSystems, "system"] call _detectChange;
[_curTriggers, _prevTriggers, "trigger"] call _detectChange;
[_curGroups, _prevGroups, "group"] call _detectChange;
[_curObjects, _prevObjects, "object"] call _detectChange;
[_curWaypoints, _prevWaypoints, "waypoint"] call _detectChange;
[_curComments, _prevComments, "comment"] call _detectChange;

// Still has issues with detecting layer creation, tracking issue: https://feedback.bistudio.com/T175680
[_curLayers, _prevLayers, "layer"] call _detectChange;

_display setVariable [QGVAR(detectEntityEventsPrev), _curEntities];
