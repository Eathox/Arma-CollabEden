#include "script_component.hpp"

params [
    ["_ip", "", [""]],
    ["_port", GVAR(defaultPort), [""]]
];

if !is3DEN exitWith {false};
if (_ip isEqualTo "") exitWith {false};

private _result = ["join", [_ip, _port]] call FUNC(callExtension);
!isNil "_result"
