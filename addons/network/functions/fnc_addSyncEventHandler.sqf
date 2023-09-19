#include "script_component.hpp"

params [
    ["_name", "", [""]],
    ["_localFunc", nil, [{}]],
    ["_remoteFunc", nil, [{}]]
];

if (_name isEqualTo "" || isNil "_localFunc" || isNil "_remoteFunc") exitWith {-1};

private _localName = format [QGVAR(localEvent_%1), _name];
private _remoteName = format [QGVAR(remoteEvent_%1), _name];
[_localName, _localFunc] call FUNC(addEventHandler);
[_remoteName, _remoteFunc] call FUNC(addEventHandler);
