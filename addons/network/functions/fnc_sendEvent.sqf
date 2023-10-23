#include "script_component.hpp"

params [
    ["_name", "", [""]],
    ["_params", [], []]
];

if (_name == "") exitWith {false};

systemChat format ["Sending Event: %1: %2", _name, _params]; // DEBUG
private _result = ["send_event", [_name, _params]] call FUNC(callExtension);
!isNil "_result";

// WIP: add batching system that groups events together (all events in the same frame?) and sends them all at once
// Required so that we dont hog the extension callback limit (which is 100 per frame)
