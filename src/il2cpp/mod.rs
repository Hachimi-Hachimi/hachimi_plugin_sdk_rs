#[cfg(feature = "il2cpp")]
pub use hachimi_il2cpp::*;

#[cfg(feature = "il2cpp_2020")]
pub use hachimi_il2cpp_2020::*;

pub mod ext;
pub mod helpers;