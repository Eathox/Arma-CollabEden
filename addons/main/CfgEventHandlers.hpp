// Remove recompile patch, recompiling is automatically disabled in release builds
#ifndef IS_RELEASE_BUILD
class Cfg3DEN {
    class EventHandlers {
        class ADDON {
            Init = QUOTE(call COMPILE_FILE(recompile));
            OnMissionNew = QUOTE(call COMPILE_FILE(recompile));
            OnMissionLoad = QUOTE(call COMPILE_FILE(recompile));
            OnMissionPreviewEnd = QUOTE(call COMPILE_FILE(recompile));
        };
    };
};
#endif
