#include "script_component.hpp"

params [
    ["_function", "", [""]],
    ["_params", nil, []]
];

if (_function isEqualTo "") exitWith {ERROR("No function specified")};

if (isNil "_params") then {
    LOG_1("Calling extension '%1'", _function);
    EXT callExtension [_function, []];
} else {
    if !(_params isEqualType []) then {_params = [_params]};
    LOG_2("Calling extension '%1' with data '%2'", _function, _params);
    EXT callExtension [_function, _params];
} params ["_result", "_code", "_error"];

if (_error != 0) exitWith {nil}; // Arma logs the error already
if (_code != 0) exitWith {
    _code = str _code;
    private _message = switch (_code select [0, 1]) do {
        case "1": {"function not found"};
        case "2": {format ["invalid number of arguments %1", _code select [1]]};
        case "3": {format ["invalid argument type at %1", _code select [1]]};
        case "4": {"result to large for buffer"};
        case "9": {_result};
        default {format ["unknown error %1", _code]};
    };
    ERROR_2("callExtension '%1': %2", _function, _message);
    nil
};
_result
