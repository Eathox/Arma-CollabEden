#include "script_component.hpp"

// Arma doesn't compile functions with recompile = 1 when entering Eden
// This is a patch job solution to make sure that they are recompiled
private _cfgFunctions = (configfile >> "CfgFunctions" >> QUOTE(PREFIX));
if (isClass _cfgFunctions) then {
	private _recompileTargets = ("true" configClasses _cfgFunctions) apply {
        "getNumber (_x >> 'recompile') == 1" configClasses _x;
    };

	{
		format ["%1_fnc_%2", QUOTE(PREFIX), configName _x] call BIS_fnc_recompile;
	} foreach flatten _recompileTargets;
};
