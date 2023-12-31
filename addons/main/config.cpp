#include "script_component.hpp"

class CfgPatches {
    class ADDON {
        name = COMPONENT_NAME;
        VERSION_CONFIG;
        authors[] = {"Eathox"};
        requiredVersion = REQUIRED_VERSION;
        requiredAddons[] = {"3DEN", "cba_main"};
        // is3DENmod = 1; // Warns user when loading mission in editor that was saved with addon loaded. (for use with custom attributes, ect.)

        units[] = {};
        weapons[] = {};
    };
};

// // Unsure if even required when using mod.cpp
// class CfgMods {
//     class PREFIX {
//         name = "Collaborative Eden";
//         picture = "title_co.paa"; // Picture displayed from the expansions menu. Optimal size is 2048x1024
//         author = "Eathox";

//         // hideName = "false";    // Hide the extension name in main menu and extension menu
//         // hidePicture = "false"; // Hide the extension picture in the extension menu

//         // action = "https://www";                            // Website URL, that can accessed from the expansions menu
//         // actionName = "today?";                             // label of button/tooltip in extension menu
//         // description = "It's unclear where this will show"; // Probably in context with action

//         // Color used for DLC stripes and backgrounds (RGBA)
//         dlcColor[] = {0.92, 0.86, 0.16, 1};
//     };
// };

#include "CfgEventHandlers.hpp"
