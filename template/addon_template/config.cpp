#include "script_component.hpp"

class CfgPatches {
    class ADDON {
        name = COMPONENT_NAME;
        VERSION_CONFIG;
        authors[] = {"Eathox"};
        requiredVersion = REQUIRED_VERSION;
        requiredAddons[] = {"coe_main"}; // Used for setting load order.
        // 0 = popup warning when missing any requiredAddons[], 1 = entire config is ignored when missing any requiredAddons[]. (Since Arma 3 2.14)
        skipWhenMissingDependencies = 0;
        // is3DENmod = 1; // Warns user when loading mission in editor that was saved with addon loaded. (for use with custom attributes, ect.)

        units[] = {};
        weapons[] = {};
    };
};

#include "CfgFunctions.hpp"
#include "CfgEventHandlers.hpp"
