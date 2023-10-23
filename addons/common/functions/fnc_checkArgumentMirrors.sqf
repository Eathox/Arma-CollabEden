#include "script_component.hpp"

{
    private _arguments = configProperties [_x >> "Arguments", toString {isClass _x}];
    if (_arguments isEqualTo []) then {continue};

    private _attributes = configProperties [_x >> "Attributes", toString {isClass _x}];
    if (_attributes isEqualTo []) then {continue};

    // These modules tend to have controls mixed in with attributes, dunno why
    _attributes = _attributes select {isText (_x >> "property") || isText (_x >> "data")};

    private _moduleName = configName _x;
    {
        private _argumentClass = configName _x;
        private _argumentName = _x call FUNC(getAttributeName);
        private _attributeMirrorIndex = _attributes findIf {
            _argumentClass == configName _x || {_argumentName == _x call FUNC(getAttributeName)}
        };

        if (_attributeMirrorIndex == -1) then {
            WARNING_2("Module %1 uses attributes but is missing mirror for: %2 argument!", _moduleName, _argumentClass);
        };
    } foreach _arguments;
} foreach (toString {configName _x isKindOf "Logic"} configClasses (configFile >> "CfgVehicles"));
