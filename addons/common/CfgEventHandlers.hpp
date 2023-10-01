class Extended_PreStart_EventHandlers {
    class ADDON {
        Init = QUOTE(call COMPILE_FILE(XEH_preStart));
    };
};

class Cfg3DEN {
	class EventHandlers {
		class ADDON {
			Init = QUOTE(call COMPILE_FILE(init3DEN));
		};
	};
};
