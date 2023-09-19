class Extended_PreInit_EventHandlers {
    class ADDON {
        init = QUOTE(call COMPILE_FILE(XEH_preInit));
    };
};

class Cfg3DEN {
	class EventHandlers {
		class ADDON {
			init = QUOTE(call COMPILE_FILE(3DEN_init));
		};
	};
};
