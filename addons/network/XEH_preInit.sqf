#include "script_component.hpp"

// WIP: Use 3DEN preferences instead of CBA settings?
[
    QGVAR(defaultPort),
    "EDITBOX",
    ["Default Port", "Default port when hosting or connecting to a server."],
    ["Collaborative Eden", QUOTE(COMPONENT_BEAUTIFIED)],
    "2300",
    0, // 2 // disable overwriting clients
    nil,
    true
] call CBA_fnc_addSetting;

if !(is3DEN || is3DENPreview) exitWith {};

addMissionEventHandler ["ExtensionCallback", {
    params ["_name", "_function", "_data"];

    if (toLower _name isNotEqualTo EXT) exitWith {};
    switch _function do {
        case "log": {
            parseSimpleArray _data params ["_target", "_level", "_message"];
            LOG_SYS(_level,FORMAT_2("<%1> %2",_target,_message));
        };
        default {
            private _eventName = QUOTE(ADDON) + "_" + _function;
            [_eventName, parseSimpleArray _data] call FUNC(callEventHandler);
        };
    };
}];
