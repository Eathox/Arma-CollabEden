#include "script_component.hpp"
// Note: this file is not called when entering Eden from a mission preview.

params ["_display"];

_display setVariable [QGVAR(currentInstance), 0]; // 0 = None, 1 = Server, 2 = Client
_display setVariable [QGVAR(blockEventBroadcastHash), createHashMap];

_display displayAddEventHandler ["UnLoad", {
    params ["_display"];
    if (call FUNC(isNetworkInstanceActive)) then {
        LOG("Network interface still active; stopping it");
        call FUNC(stopNetworkInstance);
    };
}];

add3DENEventHandler ["OnMissionPreview", {
    private _display = call FUNC(get3DENDisplay);
    if (call FUNC(isNetworkInstanceActive)) then {
        LOG("Network interface still active; stopping it");
        call FUNC(stopNetworkInstance);
    };
}];

[QGVAR(serverStarted), {
    private _display = call FUNC(get3DENDisplay);
    _display setVariable [QGVAR(currentInstance), 1];
}] call FUNC(addEventHandler);

[QGVAR(clientConnected), {
    params ["_succeeded"];
    if _succeeded then {
        private _display = call FUNC(get3DENDisplay);
        _display setVariable [QGVAR(currentInstance), 2];
    };
}] call FUNC(addEventHandler);

private _setInstanceNone = {
    private _display = call FUNC(get3DENDisplay);
    _display setVariable [QGVAR(currentInstance), 0];
};
[QGVAR(serverStopped), _setInstanceNone] call FUNC(addEventHandler);
[QGVAR(clientDisconnected), _setInstanceNone] call FUNC(addEventHandler);

// -- Sync Events
[QEGVAR(common,objectCreated), {
    params ["_id"];

    private _object = get3DENEntity _id;
    _object addEventHandler ["Dragged3DEN", {
        params ["_object"];
        [QGVAR(objectDragged), _object] call FUNC(callSyncEventHandler);
    }];
    [QGVAR(objectCreated), [_id, _object]] call FUNC(callSyncEventHandler);
}] call FUNC(addEventHandler);

[QEGVAR(common,objectDeleted), {
    params ["_id"];

    [QGVAR(objectDeleted), _id] call FUNC(callSyncEventHandler);
}] call FUNC(addEventHandler);

[QGVAR(objectCreated), {
    params ["_eventName", "_params"];
    _params params ["_id", "_object"];

    [_eventName, [
        get3DENEntityID (group _object),
        _id,
        typeOf _object,
        (_object get3DENAttribute "position") # 0 // Returns [[x, y, z]], hence the # 0
    ]] call FUNC(broadcastEvent);
}, {
    params ["_eventName", "_params"];
    _params params ["_groupId", "_id", "_className", "_pos"];
    private _group = get3DENEntity _groupId;
    if (_group isEqualTo -1) then {_group = grpNull};

    _eventName call FUNC(blockEventBroadcast);
    _group create3DENEntity ["Object", _className, _pos];
}] call FUNC(addSyncEventHandler);

[QGVAR(objectDeleted), {
    params ["_eventName", "_id"];
    [_eventName, _id] call FUNC(broadcastEvent);
}, {
    params ["_eventName", "_id"];
    _eventName call FUNC(blockEventBroadcast);
    delete3DENEntities [_id];
}] call FUNC(addSyncEventHandler);

[QGVAR(objectDragged), {
    params ["_eventName", "_object"];
    [_eventName, [
        get3DENEntityID _object,
        (_object get3DENAttribute "position") # 0, // Returns [[x, y, z]], hence the # 0
        (_object get3DENAttribute "rotation") # 0 // Returns [[x, y, z]], hence the # 0
    ]] call FUNC(broadcastEvent);
}, {
    params ["_eventName", "_params"];
    _params params ["_id", "_pos", "_rot"];

    private _object = get3DENEntity _id;
    _object set3DENAttribute ["position", _pos];
    _object set3DENAttribute ["rotation", _rot];
}] call FUNC(addSyncEventHandler);

