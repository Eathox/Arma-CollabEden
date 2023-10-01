#define DEBUG_SYNCHRONOUS
#include "\x\cba\addons\main\script_macros_common.hpp"

// Disable recompiling for release builds
#ifdef IS_RELEASE_BUILD
#undef DISABLE_COMPILE_CACHE
#endif

// Using CfgFunctions instead of PREP
#undef PREP

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

// Not using PREP means all functions lose their component prefix
#undef FUNC
#define FUNC(var1) TRIPLES(PREFIX,fnc,var1)

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
