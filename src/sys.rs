use std::ffi::{c_char, c_void};

#[cfg(feature = "il2cpp_api")]
use crate::il2cpp::types::{Il2CppImage, Il2CppClass, Il2CppTypeEnum, MethodInfo};

/// Private
pub struct Hachimi;

/// Private
pub struct Interceptor;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vtable {
    /* Hachimi */
    pub hachimi_instance: unsafe extern "C" fn() -> *const Hachimi,
    pub hachimi_get_interceptor: unsafe extern "C" fn(this: *const Hachimi) -> *const Interceptor,

    /* Interceptor */
    pub interceptor_hook: unsafe extern "C" fn(
        this: *const Interceptor, orig_addr: *mut c_void, hook_addr: *mut c_void
    ) -> *mut c_void,
    pub interceptor_hook_vtable: unsafe extern "C" fn(
        this: *const Interceptor, vtable: *mut *mut c_void, vtable_index: usize, hook_addr: *mut c_void
    ) -> *mut c_void,
    pub interceptor_get_trampoline_addr: unsafe extern "C" fn(
        this: *const Interceptor, hook_addr: *mut c_void
    ) -> *mut c_void,
    pub interceptor_unhook: unsafe extern "C" fn(this: *const Interceptor, hook_addr: *mut c_void) -> *mut c_void,

    /* Il2Cpp */
    pub il2cpp_resolve_symbol: unsafe extern "C" fn(name: *const c_char) -> *mut c_void,
    #[cfg(feature = "il2cpp_api")]
    pub il2cpp_get_assembly_image: unsafe extern "C" fn(assembly_name: *const c_char) -> *const Il2CppImage,
    #[cfg(feature = "il2cpp_api")]
    pub il2cpp_get_class: unsafe extern "C" fn(
        image: *const Il2CppImage, namespace: *const c_char, class_name: *const c_char
    ) -> *mut Il2CppClass,
    #[cfg(feature = "il2cpp_api")]
    pub il2cpp_get_method: unsafe extern "C" fn(
        class: *mut Il2CppClass, name: *const c_char, args_count: i32
    ) -> *const MethodInfo,
    #[cfg(feature = "il2cpp_api")]
    pub il2cpp_get_method_overload: unsafe extern "C" fn(
        class: *mut Il2CppClass, name: *const c_char, params: *const Il2CppTypeEnum, param_count: usize
    ) -> *const MethodInfo,
    #[cfg(feature = "il2cpp_api")]
    pub il2cpp_get_method_addr: unsafe extern "C" fn(
        class: *mut Il2CppClass, name: *const c_char, args_count: i32
    ) -> *mut c_void,
    #[cfg(feature = "il2cpp_api")]
    pub il2cpp_get_method_overload_addr: unsafe extern "C" fn(
        class: *mut Il2CppClass, name: *const c_char, params: *const Il2CppTypeEnum, param_count: usize
    ) -> *mut c_void,

    /* Logging */
    pub log: unsafe extern "C" fn(level: i32, target: *const c_char, message: *const c_char)
}