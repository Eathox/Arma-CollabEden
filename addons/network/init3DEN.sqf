#include "script_component.hpp"
#include "\a3\3den\UI\resincl.inc"
// Note: this file is not called when entering Eden from a mission preview.

params ["_display"];

// TODO: support phase selection, till then disable it
(_display displayCtrl IDC_DISPLAY3DEN_TOOLBAR_WORKSPACE) ctrlEnable false;

_display setVariable [QGVAR(currentInstance), 0]; // 0 = None, 1 = Server, 2 = Client
_display setVariable [QGVAR(blockSendEventHandler_hashMap), createHashMap];

_display displayAddEventHandler ["UnLoad", FUNC(stopNetworkInstance)];
add3DENEventHandler ["OnMissionPreview", FUNC(stopNetworkInstance)];

[QGVAR(receivedEvent), {
    params ["_name", "_params"];

    systemChat format ["Received Event: %1: %2", _name, _params]; // DEBUG
    private _internalName = QGVAR(receivedEvent_) + _name;
    [_internalName, [_name, _params]] call FUNC(callEventHandler);
}] call FUNC(addEventHandler);

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

// -- Trigger Send Events
[QEGVAR(common,objectCreated), {
    params ["_id"];

    private _object = get3DENEntity _id;
    _object addEventHandler ["Dragged3DEN", {
        params ["_object"];
        [QGVAR(objectDragged), _object] call FUNC(callSendEventHandler);
    }];
    [QGVAR(objectCreated), [_id, _object]] call FUNC(callSendEventHandler);
}] call FUNC(addEventHandler);

[QEGVAR(common,objectDeleted), {
    params ["_id"];

    [QGVAR(objectDeleted), _id] call FUNC(callSendEventHandler);
}] call FUNC(addEventHandler);

// -- Send And Received Handlers
[QGVAR(objectCreated), {
    params ["_eventName", "_params"];
    _params params ["_id", "_object"];

    [_eventName, [
        get3DENEntityID (group _object),
        _id,
        typeOf _object,
        (_object get3DENAttribute "position") select 0 // Returns [[x, y, z]], hence the select
    ]] call FUNC(sendEvent);
}] call FUNC(addSendEventHandler);
[QGVAR(objectCreated), {
    params ["_eventName", "_params"];
    _params params ["_groupId", "_id", "_className", "_pos"];

    private _group = get3DENEntity _groupId;
    if (_group isEqualTo -1) then {_group = grpNull};
    _eventName call FUNC(blockSendEventHandler);
    _group create3DENEntity ["Object", _className, _pos];
}] call FUNC(addReceivedEventHandler);

[QGVAR(objectDeleted), FUNC(sendEvent)] call FUNC(addSendEventHandler);
[QGVAR(objectDeleted), {
    params ["_eventName", "_id"];

    _eventName call FUNC(blockSendEventHandler);
    delete3DENEntities [_id];
}] call FUNC(addReceivedEventHandler);

// TODO: think about what data needs to be sent, i.e: dont need to send an attribute that wasn't changed
[QGVAR(objectDragged), {
    params ["_eventName", "_object"];

    [_eventName, [
        get3DENEntityID _object,
        (_object get3DENAttribute "position") select 0, // Returns [[x, y, z]], hence the select
        (_object get3DENAttribute "rotation") select 0 // Returns [[x, y, z]], hence the select
    ]] call FUNC(sendEvent);
}] call FUNC(addSendEventHandler);
[QGVAR(objectDragged), {
    params ["_eventName", "_params"];
    _params params ["_id", "_pos", "_rot"];

    private _object = get3DENEntity _id;
    _object set3DENAttribute ["position", _pos];
    _object set3DENAttribute ["rotation", _rot];
}] call FUNC(addReceivedEventHandler);

