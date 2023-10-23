#include "script_component.hpp"

if !(is3DEN) exitWith {};

// TODO: get https://github.com/CBATeam/CBA_A3/pull/1616 merged and remove this
addMissionEventHandler ["EachFrame", {call CBA_common_fnc_onFrame}];
