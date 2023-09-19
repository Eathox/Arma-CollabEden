#define COMPONENT addon_template
#define COMPONENT_BEAUTIFIED addon_template
#include "..\main\script_mod.hpp"

// #define DEBUG_MODE_FULL
#define DISABLE_COMPILE_CACHE // DEBUG

#ifdef DEBUG_ENABLED_ADDON_TEMPLATE
    #define DEBUG_MODE_FULL
#endif
#ifdef DEBUG_SETTINGS_ADDON_TEMPLATE
    #define DEBUG_SETTINGS DEBUG_SETTINGS_ADDON_TEMPLATE
#endif

#include "..\main\script_macros.hpp"
