#include "script_component.hpp"

params [
    ["_name", "", [""]],
    ["_func", nil, [{}]]
];

private _internalName = QGVAR(sendEvent_) + _name;
[_internalName, _func] call FUNC(addEventHandler);
