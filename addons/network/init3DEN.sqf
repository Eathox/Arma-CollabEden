#include "script_component.hpp"
#include "\a3\3den\UI\resincl.inc"
// Note: this file is not called when entering Eden from a mission preview.

params ["_display"];

// TODO: support phase selection, till then disable it
(_display displayCtrl IDC_DISPLAY3DEN_TOOLBAR_WORKSPACE) ctrlEnable false;

_display setVariable [QGVAR(currentInstance), 0]; // 0 = None, 1 = Server, 2 = Client
_display setVariable [QGVAR(blockSendEventHandler), false];

_display displayAddEventHandler ["UnLoad", FUNC(stopNetworkInstance)];
add3DENEventHandler ["OnMissionPreview", FUNC(stopNetworkInstance)];

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

call FUNC(initSyncHandlers);
