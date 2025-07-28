pub mod api;
pub mod sys;

#[cfg(feature = "macros")]
pub use hachimi_plugin_macros::hachimi_plugin;

#[cfg(feature = "il2cpp")]
pub use hachimi_il2cpp as il2cpp;

#[cfg(feature = "il2cpp_2020")]
pub use hachimi_il2cpp_2020 as il2cpp;