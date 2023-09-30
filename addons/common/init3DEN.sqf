#include "script_component.hpp"
// Note: this file is not called when entering Eden from a mission preview.

params ["_display"];

uiNameSpace setVariable [QGVAR(3DENDisplay), _display];
_display displayAddEventHandler ["UnLoad", {
    uiNameSpace setVariable [QGVAR(3DENDisplay), nil];
}];

_display setVariable [QGVAR(detectEntityEvents_prevEntities), []];

call FUNC(preloadAttributes);
call FUNC(preloadMissionAttributes);

add3DENEventHandler ["OnUndo", FUNC(detectEntityEvents)];
add3DENEventHandler ["OnRedo", FUNC(detectEntityEvents)];
add3DENEventHandler ["OnHistoryChange", FUNC(detectEntityEvents)];
// Required as these don't trigger OnHistoryChange, tracking issue: https://feedback.bistudio.com/T175680
add3DENEventHandler ["OnPaste", FUNC(detectEntityEvents)];
add3DENEventHandler ["OnPasteUnitOrig", FUNC(detectEntityEvents)];

// -- WIP, might want to move these latter to a interface component or something
[QEGVAR(network,serverStarted), {
    "Hosting server" call BIS_fnc_3DENNotification;
}] call FUNC(addEventHandler);

[QEGVAR(network,serverStopped), {
    "Shutdown server" call BIS_fnc_3DENNotification;
}] call FUNC(addEventHandler);

[QEGVAR(network,serverClientConnected), {
    params ["_clientId"];
    format ["Client (%1) joined server", _clientId] call BIS_fnc_3DENNotification;
}] call FUNC(addEventHandler);

[QEGVAR(network,serverClientDisconnected), {
    params ["_clientId"];
    format ["Client (%1) left server", _clientId] call BIS_fnc_3DENNotification;
}] call FUNC(addEventHandler);

[QEGVAR(network,clientConnected), {
    params ["_succeeded"];
    private _params = [["Failed to connect to server", 1], "Connected to server"] select _succeeded;
    _params call BIS_fnc_3DENNotification;
}] call FUNC(addEventHandler);

[QEGVAR(network,clientDisconnected), {
    params ["_lostConnection"];
    private _params = ["Disconnected from server", ["Lost connection to server", 1]] select _lostConnection;
    _params call BIS_fnc_3DENNotification;
}] call FUNC(addEventHandler);
