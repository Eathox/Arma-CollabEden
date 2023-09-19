let releases = HEMTT_RFS.join("releases");

let src = releases.join(HEMTT.project().prefix() + "-latest.zip");
let dst = releases.join(HEMTT.project().name().to_lower() + "_v" + HEMTT.project().version().to_string_short() + ".zip");

if src.is_file() { // support --no-archive
    print("moving zip to " + dst);
    if !src.move(dst) {
        fatal("failed to move " + src + " to " + dst);
    }
}
