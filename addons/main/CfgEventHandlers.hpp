// Remove recompile patch, recompiling is automatically disabled in release builds
#ifndef IS_RELEASE_BUILD
class Extended_PreInit_EventHandlers {
    class ADDON {
        init = QUOTE(call COMPILE_FILE(recompile));
    };
};

class Cfg3DEN {
	class EventHandlers {
		class ADDON {
			init = QUOTE(call COMPILE_FILE(recompile));
		};
	};
};
#endif
