#include "script_component.hpp"

// Call send handlers

[QGVARMAIN(entityCreated), {
    params ["_entityType", "_id"];

    private _entity = get3DENEntity _id;
    [QGVAR(entityCreated), [_entityType, _id, _entity]] call FUNC(callSendEventHandler);

    if (_entityType in ["object", "trigger", "logic"]) exitWith {
        _entity addEventHandler ["AttributesChanged3DEN", {
            params ["_entity"];

            private _id = get3DENEntityID _entity;
            private _entityType = _entity call FUNC(getEntityType);
            [QGVAR(entityAttributesChanged), [_entityType, _id, _entity]] call FUNC(callSendEventHandler);
        }];
        _entity addEventHandler ["Dragged3DEN", {
            params ["_entity"];

            private _id = get3DENEntityID _entity;
            private _entityType = _entity call FUNC(getEntityType);
            [QGVAR(entityDragged), [_entityType, _id, _entity]] call FUNC(callSendEventHandler);
        }];
    };
}] call FUNC(addEventHandler);

[QGVARMAIN(entityDeleted), {
    params ["_entityType", "_id"];

    [QGVAR(entityDeleted), [_entityType, _id]] call FUNC(callSendEventHandler);
}] call FUNC(addEventHandler);

// Sync Handlers

[QGVAR(entityCreated), {
    params ["_entityType", "_id", "_entity"];

    switch toLower _entityType do {
        // WIP: impl empty cases once more EH support arives
        case "layer": {}; // WIP: could construct a tree of layers using the edit control (IDC_DISPLAY3DEN_PANELLEFT_EDIT)
        case "comment": {};
        case "marker": {};
        case "trigger": {};
        case "logic": {};
        case "group": {};
        case "object": {
            // TODO: Add vehicle with and without crew support
            private _params = [
                get3DENEntityID (group _entity),
                typeOf _entity,
                (_entity get3DENAttribute "position") select 0 // Returns [[x, y, z]], hence the select
            ];
            [_eventName, [_entityType, _id, _params]] call FUNC(sendEvent);
        };
        case "waypoint": {};
        default {ERROR_1("Unknown entity type in send created: %1", _entityType)};
    };


}] call FUNC(addSendEventHandler);

[QGVAR(entityCreated), {
    params ["_entityType", "_id", "_params"];

    switch toLower _entityType do {
        // WIP: impl empty cases once more EH support arives
        case "layer": {};
        case "comment": {};
        case "marker": {}; // TODO: figure out how to handle name conflicts
        case "trigger": {};
        case "logic": {};
        case "group": {};
        case "object": {
            _params params ["_groupId", "_className", "_pos"];

            _group = get3DENEntity _groupId;
            if (_group isEqualTo -1) then {
                create3DENEntity ["object", _className, _pos];
            } else {
                _group create3DENEntity ["object", _className, _pos];
            };
        };
        case "waypoint": {};
        default {ERROR_1("Unknown entity type in received created: %1", _entityType)};
    };
}] call FUNC(addReceivedEventHandler);


[QGVAR(entityDeleted), {
    params ["_entityType", "_id"];

    [_eventName, _id] call FUNC(sendEvent)
}] call FUNC(addSendEventHandler);
[QGVAR(entityDeleted), {
    params ["_id"];

    delete3DENEntities [_id];
}] call FUNC(addReceivedEventHandler);


// Currently only works for objects, triggers and logics
[QGVAR(entityAttributesChanged), {
    params ["_entityType", "_id", "_entity"];

    private _attributes = _entity call FUNC(getAttributes);
    [_eventName, [_id, _attributes]] call FUNC(sendEvent);
}] call FUNC(addSendEventHandler);

[QGVAR(entityAttributesChanged), {
    params ["_id", "_attributes"];

    set3DENAttributes (_attributes apply {
		[[_id], _x select 0, _x select 1]
	});
}] call FUNC(addReceivedEventHandler);


// Currently only works for objects, triggers and logics
[QGVAR(entityDragged), {
    params ["_entityType", "_id", "_entity"];

    private _currentTime = diag_tickTime;
    private _lastUpdate = _entity getVariable [QGVAR(lastDragUpdate), 0];
    if (_currentTime - _lastUpdate < 0.025) exitWith {}; // 40 updates per second

    _entity setVariable [QGVAR(lastDragUpdate), _currentTime];
    [_eventName, [
        _id,
        (_entity get3DENAttribute "position") select 0, // Returns [[x, y, z]], hence the select
        (_entity get3DENAttribute "rotation") select 0 // Returns [[x, y, z]], hence the select
    ]] call FUNC(sendEvent);
}] call FUNC(addSendEventHandler);

[QGVAR(entityDragged), {
    params ["_id", "_pos", "_rot"];

    set3DENAttributes [[[_id], "position", _pos], [[_id], "rotation", _rot]];
}] call FUNC(addReceivedEventHandler);
