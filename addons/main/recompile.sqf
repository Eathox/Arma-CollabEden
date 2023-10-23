#include "script_component.hpp"

// Arma doesn't compile functions with recompile = 1 when entering Eden
// This is a patch job solution to make sure that they are recompiled
INFO("Recompiling functions for use in 3DEN.");
{
    {
        private _name = format [QFUNC(%1), configName _x];
        _name call BIS_fnc_recompile;
    } foreach configProperties [_x, toString {isClass _x && getNumber (_x >> "recompile") == 1}];
} foreach ("true" configClasses (configFile >> "CfgFunctions" >> QUOTE(PREFIX)));
