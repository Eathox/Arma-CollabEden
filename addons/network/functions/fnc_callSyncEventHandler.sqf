#include "script_component.hpp"

params [
    ["_name", "", [""]],
    ["_params", [], []]
];

if !(call FUNC(isNetworkInstanceActive)) exitWith {};

private _localName = format [QGVAR(localEvent_%1), _name];
[_localName, [_name, _params]] call FUNC(callEventHandler);
