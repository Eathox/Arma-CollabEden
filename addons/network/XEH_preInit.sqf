#include "script_component.hpp"

[
    QGVAR(defaultPort),
    "EDITBOX",
    ["Default Port", "Default port when hosting or connecting to a server."],
    ["Collaborative Eden", QUOTE(COMPONENT_BEAUTIFIED)],
    "2300",
    2,
    nil,
    true
] call CBA_fnc_addSetting;

if !(is3DEN || is3DENPreview) exitWith {};

addMissionEventHandler ["ExtensionCallback", {
    params ["_name", "_function", "_data"];

    if (toLower _name == EXT + "_log") exitWith {
        parseSimpleArray _data params ["_component", "_message"];
        LOG_SYS(_function,FORMAT_2("<%1> %2",_component,_message));
    };

    if (toLower _name != EXT) exitWith {};
    if (_function == "remoteEvent") then {
        parseSimpleArray _data params ["_name", "_params"];
        private _remoteName = format [QGVAR(remoteEvent_%1), _name];
        systemChat format ["Received Event: %1: %2", _name, _params]; // DEBUG
        [_remoteName, [_name, _params]] call FUNC(callEventHandler);
    } else {
        _name = format [QGVAR(%1), _function];
        [_name, parseSimpleArray _data] call FUNC(callEventHandler);
    }
}];
