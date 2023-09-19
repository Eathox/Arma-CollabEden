#define DEBUG_SYNCHRONOUS
#include "\x\cba\addons\main\script_macros_common.hpp"

// Using CfgFunctions instead of PREP
#ifdef DISABLE_COMPILE_CACHE
    #undef RECOMPILE
    #define RECOMPILE recompile = 1
#endif

#undef PATHTO_FNC
#define PATHTO_FNC(fncName) \
    class fncName { \
        file = QPATHTOF(functions\DOUBLES(fnc,fncName).sqf); \
        CFGFUNCTION_HEADER; \
        RECOMPILE; \
    }

#undef PREP
// #ifdef DISABLE_COMPILE_CACHE
// #undef PREP
// #define PREP(fncName) FUNC(fncName) = compile preprocessFileLineNumbers QPATHTOF(functions\DOUBLES(fnc,fncName).sqf)
// #else
// #undef PREP
// #define PREP(fncName) [ QPATHTOF(functions\DOUBLES(fnc,fncName).sqf), QFUNC(fncName) ] call CBA_fnc_compileFunction
// #endif

// Not using PREP means all functions lose their component prefix
#undef FUNC
#define FUNC(var1) TRIPLES(PREFIX,fnc,var1)
// #define UFUNC(var1) (uiNameSpace getVariable QFUNC(var1))

// QFUNC and QQFUNC are still available
#undef FUNCMAIN
#undef FUNC_INNER
#undef EFUNC
#undef QFUNCMAIN
#undef QFUNC_INNER
#undef QEFUNC
#undef QQFUNCMAIN
#undef QQFUNC_INNER
#undef QQEFUNC

// RemoteExec
// #define REMOTE_GLOBAL  0
// #define REMOTE_SERVER  ([2,([{call CBA_fnc_currentUnit},2] select isDedicated)] select isServer)
// #define REMOTE_CLIENTS ([0,-2] select isDedicated)
