# Hachimi Plugin SDK for Rust
This crate lets you develop Hachimi plugins using Rust. Under the hood, it wraps the plugin system's C API.

# Usage
Make sure your library is configured to build as a `cdylib`, and use Rust edition 2021. Include this crate as a dependency in your project:
```toml
hachimi_plugin_sdk = { git = "https://github.com/Hachimi-Hachimi/hachimi_plugin_sdk_rs.git", features = ["il2cpp"] }
```

There are a few features that you can enable:
- `macros`: Include proc macros. (Enabled by default)
- `il2cpp`: Include il2cpp API definitions and plugin APIs that depends on the il2cpp types.
- `il2cpp_2020`: Same as above, but for Unity 2020.

**Take special note of the `il2cpp` feature.** If you wanna use it, you will have to enable it manually as shown above, and select the correct variant for the game/Unity version that your plugin is targetting.

You must then add a plugin entry point using the `hachimi_plugin` proc macro attribute. You can then use the Hachimi API within the entry point.

```rs
// You don't need to call it "main", however this is a nice convention to have.
// This will export a C function called "hachimi_init". You cannot use that same name for this.
#[hachimi_plugin]
pub fn main(api: HachimiApi) {
    // Your code here...
}
```

It is also safe to clone the API object for use outside of the entry point.

# Examples
See [`/examples`](examples)

# License
[MIT](LICENSE)