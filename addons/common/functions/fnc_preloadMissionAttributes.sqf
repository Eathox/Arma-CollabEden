#include "script_component.hpp"

private _cache = uiNamespace getVariable QGVAR(missionAttributes);
if !(isNil "_cache") exitWith {
    INFO("3DEN mission attributes already preloaded.");
    false
};

private _timeStart = diag_tickTime;
_cache = createHashMap;

{
    private _sectionCache = createHashMap;
    private _categories = configProperties [_x >> "AttributeCategories", "isClass _x"];
    {
        private _attributes = configProperties [_x >> "Attributes", "isClass _x"];
        private _names = _attributes apply {_x call FUNC(getAttributeName)};
        _sectionCache set [toLower (configName _x), _names];
    } foreach _categories;

    _cache set [toLower (configName _x), _sectionCache];
} foreach ("true" configClasses (configFile >> "Cfg3DEN" >> "Mission"));

uiNamespace setVariable [QGVAR(missionAttributes), _cache];

private _duration = round ((diag_tickTime - _timeStart) * 1000);
INFO_1("Preloaded 3DEN mission attributes. Time: %1 ms", _duration);

true
