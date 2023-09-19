#include "script_component.hpp"

private _display = call FUNC(get3DENDisplay);
if (isNull _display) exitWith {ERROR("Failed to get 3DEN display")};

_display getVariable [QGVAR(currentInstance), 0] > 0
