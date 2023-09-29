let value = if !HEMTT.is_release() {
    return;
};

let script_mod = HEMTT_VFS.join("addons").join("main").join("script_mod.hpp");
let content = script_mod.open_file().read();
content.replace("// #define IS_RELEASE_BUILD", "#define IS_RELEASE_BUILD");

script_mod.create_file().write(content);
print("script_mod.hpp: IS_RELEASE_BUILD defined");
