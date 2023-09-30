#include "script_component.hpp"

with uiNamespace do {
    INFO("Validating attribute compatibility.");
    private _timeStart = diag_tickTime;

    call FUNC(checkAttributeDuplicates);
    call FUNC(checkArgumentMirrors);

    private _duration = round ((diag_tickTime - _timeStart) * 1000);
    INFO_1("Finished validating. Time: %1 ms", _duration);
};
