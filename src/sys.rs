use std::ffi::{c_char, c_void};

#[cfg(feature = "il2cpp_api")]
use crate::il2cpp::types::{Il2CppImage, Il2CppClass, Il2CppTypeEnum, MethodInfo, Il2CppObject, FieldInfo, Il2CppThread, il2cpp_array_size_t, Il2CppArray};

pub const VERSION: i32 = 1;

#[repr(i32)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum InitResult {
    Error,
    Ok
}

/// Private
pub struct Hachimi;

/// Private
pub struct Interceptor;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vtable {
    pub hachimi_instance: unsafe extern "C" fn() -> *const Hachimi,
    pub hachimi_get_interceptor: unsafe extern "C" fn(this: *const Hachimi) -> *const Interceptor,

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

    pub il2cpp_resolve_symbol: unsafe extern "C" fn(name: *const c_char) -> *mut c_void,
    #[cfg(feature = "il2cpp_api")] pub il2cpp_get_assembly_image: unsafe extern "C" fn(assembly_name: *const c_char) -> *const Il2CppImage,
    #[cfg(feature = "il2cpp_api")] pub il2cpp_get_class: unsafe extern "C" fn(
        image: *const Il2CppImage, namespace: *const c_char, class_name: *const c_char
    ) -> *mut Il2CppClass,
    #[cfg(feature = "il2cpp_api")] pub il2cpp_get_method: unsafe extern "C" fn(
        class: *mut Il2CppClass, name: *const c_char, args_count: i32
    ) -> *const MethodInfo,
    #[cfg(feature = "il2cpp_api")] pub il2cpp_get_method_overload: unsafe extern "C" fn(
        class: *mut Il2CppClass, name: *const c_char, params: *const Il2CppTypeEnum, param_count: usize
    ) -> *const MethodInfo,
    #[cfg(feature = "il2cpp_api")] pub il2cpp_get_method_addr: unsafe extern "C" fn(
        class: *mut Il2CppClass, name: *const c_char, args_count: i32
    ) -> *mut c_void,
    #[cfg(feature = "il2cpp_api")] pub il2cpp_get_method_overload_addr: unsafe extern "C" fn(
        class: *mut Il2CppClass, name: *const c_char, params: *const Il2CppTypeEnum, param_count: usize
    ) -> *mut c_void,
        #[cfg(feature = "il2cpp_api")] pub il2cpp_get_method_cached: unsafe extern "C" fn(
        class: *mut Il2CppClass, name: *const c_char, args_count: i32
    ) -> *const MethodInfo,
    #[cfg(feature = "il2cpp_api")] pub il2cpp_get_method_addr_cached: unsafe extern "C" fn(
        class: *mut Il2CppClass, name: *const c_char, args_count: i32
    ) -> *mut c_void,
    #[cfg(feature = "il2cpp_api")] pub il2cpp_find_nested_class: unsafe extern "C" fn(
        class: *mut Il2CppClass, name: *const c_char
    ) -> *mut Il2CppClass,
    #[cfg(feature = "il2cpp_api")] pub il2cpp_get_field_from_name: unsafe extern "C" fn(
        class: *mut Il2CppClass, name: *const c_char
    ) -> *mut FieldInfo,
    #[cfg(feature = "il2cpp_api")] pub il2cpp_get_field_value: unsafe extern "C" fn(
        obj: *mut Il2CppObject, field: *mut FieldInfo, out_value: *mut c_void
    ),
    #[cfg(feature = "il2cpp_api")] pub il2cpp_set_field_value: unsafe extern "C" fn(
        obj: *mut Il2CppObject, field: *mut FieldInfo, value: *const c_void
    ),
    #[cfg(feature = "il2cpp_api")] pub il2cpp_get_static_field_value: unsafe extern "C" fn(
        field: *mut FieldInfo, out_value: *mut c_void
    ),
    #[cfg(feature = "il2cpp_api")] pub il2cpp_set_static_field_value: unsafe extern "C" fn(
        field: *mut FieldInfo, value: *const c_void
    ),
    #[cfg(feature = "il2cpp_api")] pub il2cpp_unbox: unsafe extern "C" fn(obj: *mut Il2CppObject) -> *mut c_void,
    #[cfg(feature = "il2cpp_api")] pub il2cpp_get_main_thread: unsafe extern "C" fn() -> *mut Il2CppThread,
    #[cfg(feature = "il2cpp_api")] pub il2cpp_get_attached_threads: unsafe extern "C" fn(out_size: *mut usize) -> *mut *mut Il2CppThread,
    #[cfg(feature = "il2cpp_api")] pub il2cpp_schedule_on_thread: unsafe extern "C" fn(thread: *mut Il2CppThread, callback: unsafe extern "C" fn()),
    #[cfg(feature = "il2cpp_api")] pub il2cpp_create_array: unsafe extern "C" fn(
        element_type: *mut Il2CppClass, length: il2cpp_array_size_t
    ) -> *mut Il2CppArray,
    #[cfg(feature = "il2cpp_api")] pub il2cpp_get_singleton_like_instance: unsafe extern "C" fn(class: *mut Il2CppClass) -> *mut Il2CppObject,

    pub log: unsafe extern "C" fn(level: i32, target: *const c_char, message: *const c_char),
}