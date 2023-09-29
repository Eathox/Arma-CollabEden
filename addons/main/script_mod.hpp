#define MAINPREFIX z
#define PREFIX coe

// #define IS_RELEASE_BUILD // Only defined in release builds (automatically set by HEMTT)
#define EXT "collab_eden"

#include "script_version.hpp"

#define VERSION MAJOR.MINOR
#define VERSION_AR MAJOR, MINOR, PATCH, BUILD

#define REQUIRED_VERSION 2.14

#ifdef COMPONENT_BEAUTIFIED
    #define COMPONENT_NAME QUOTE(COE - COMPONENT_BEAUTIFIED)
#else
    #define COMPONENT_NAME QUOTE(COE - COMPONENT)
#endif
