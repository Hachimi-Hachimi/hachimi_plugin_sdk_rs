use std::ffi::CStr;

#[cfg(feature = "il2cpp_api")]
use crate::il2cpp::types::{Il2CppImage, Il2CppClass, MethodInfo, Il2CppTypeEnum, FieldInfo, Il2CppObject, Il2CppThread, Il2CppArray, il2cpp_array_size_t};

use crate::sys;

#[derive(Debug, Clone, Copy)]
pub struct HachimiApi {
    vtable: sys::Vtable,
    version: i32
}

impl HachimiApi {
    pub unsafe fn new(vtable: *mut sys::Vtable, version: i32) -> Self {
        let vtable = (*vtable).clone();
        Self {
            vtable,
            version
        }
    }

    pub fn vtable(&self) -> &sys::Vtable {
        &self.vtable
    }

    pub fn version(&self) -> i32 {
        self.version
    }

    pub fn log(&self, level: LogLevel, target: &CStr, message: &CStr) {
        unsafe { (self.vtable.log)(level as _, target.as_ptr(), message.as_ptr()) }
    }

    pub fn il2cpp(&self) -> HachimiIl2CppApi {
        HachimiIl2CppApi(self)
    }
}

pub struct HachimiIl2CppApi<'a>(&'a HachimiApi);

impl<'a> HachimiIl2CppApi<'a> {
    pub fn resolve_symbol(&self, name: &CStr) -> usize {
        unsafe { (self.0.vtable.il2cpp_resolve_symbol)(name.as_ptr()) as _ }
    }
}

#[cfg(feature = "il2cpp_api")]
impl<'a> HachimiIl2CppApi<'a> {
    pub fn get_assembly_image(&self, assembly_name: &CStr) -> *const Il2CppImage {
        unsafe { (self.0.vtable.il2cpp_get_assembly_image)(assembly_name.as_ptr()) }
    }

    pub fn get_class(
        &self, image: *const Il2CppImage, namespace: &CStr, class_name: &CStr
    ) -> *mut Il2CppClass {
        unsafe { (self.0.vtable.il2cpp_get_class)(image, namespace.as_ptr(), class_name.as_ptr()) }
    }

    pub fn get_method(
        &self, class: *mut Il2CppClass, name: &CStr, args_count: i32
    ) -> *const MethodInfo {
        unsafe { (self.0.vtable.il2cpp_get_method)(class, name.as_ptr(), args_count) }
    }

    pub fn get_method_overload(
        &self, class: *mut Il2CppClass, name: &CStr, params: &[Il2CppTypeEnum]
    ) -> *const MethodInfo {
        unsafe { (self.0.vtable.il2cpp_get_method_overload)(class, name.as_ptr(), params.as_ptr(), params.len()) }
    }

    pub fn get_method_addr(
        &self, class: *mut Il2CppClass, name: &CStr, args_count: i32
    ) -> usize {
        unsafe { (self.0.vtable.il2cpp_get_method_addr)(class, name.as_ptr(), args_count) as _ }
    }

    pub fn get_method_overload_addr(
        &self, class: *mut Il2CppClass, name: &CStr, params: &[Il2CppTypeEnum]
    ) -> usize {
        unsafe { (self.0.vtable.il2cpp_get_method_overload_addr)(class, name.as_ptr(), params.as_ptr(), params.len()) as _ }
    }

    pub fn get_method_cached(
        &self, class: *mut Il2CppClass, name: &CStr, args_count: i32
    ) -> *const MethodInfo {
        unsafe { (self.0.vtable.il2cpp_get_method_cached)(class, name.as_ptr(), args_count) }
    }

    pub fn get_method_addr_cached(
        &self, class: *mut Il2CppClass, name: &CStr, args_count: i32
    ) -> usize {
        unsafe { (self.0.vtable.il2cpp_get_method_addr_cached)(class, name.as_ptr(), args_count) as _ }
    }

    pub fn find_nested_class(
        &self, class: *mut Il2CppClass, name: &CStr
    ) -> *mut Il2CppClass {
        unsafe { (self.0.vtable.il2cpp_find_nested_class)(class, name.as_ptr()) }
    }

    pub fn get_field_from_name(
        &self, class: *mut Il2CppClass, name: &CStr
    ) -> *mut FieldInfo {
        unsafe { (self.0.vtable.il2cpp_get_field_from_name)(class, name.as_ptr()) }
    }

    pub unsafe fn il2cpp_get_field_value<T>(
        &self, obj: *mut Il2CppObject, field: *mut FieldInfo
    ) -> T {
        let mut value = std::mem::MaybeUninit::uninit();
        (self.0.vtable.il2cpp_get_field_value)(obj, field, value.as_mut_ptr() as _);
        value.assume_init()
    }

    pub unsafe fn il2cpp_set_field_value<T>(
        &self, obj: *mut Il2CppObject, field: *mut FieldInfo, value: &T
    ) {
        (self.0.vtable.il2cpp_set_field_value)(obj, field, std::ptr::from_ref(value) as _);
    }

    pub unsafe fn il2cpp_get_static_field_value<T>(
        &self, field: *mut FieldInfo
    ) -> T {
        let mut value = std::mem::MaybeUninit::uninit();
        (self.0.vtable.il2cpp_get_static_field_value)(field, value.as_mut_ptr() as _);
        value.assume_init()
    }

    pub unsafe fn il2cpp_set_static_field_value<T>(
        &self, field: *mut FieldInfo, value: &T
    ) {
        (self.0.vtable.il2cpp_set_static_field_value)(field, std::ptr::from_ref(value) as _);
    }

    pub fn unbox(&self, obj: *mut Il2CppObject) -> *mut ::std::os::raw::c_void {
        unsafe { (self.0.vtable.il2cpp_unbox)(obj) }
    }

    pub fn get_main_thread(&self) -> *mut Il2CppThread {
        unsafe { (self.0.vtable.il2cpp_get_main_thread)() }
    }

    pub fn get_attached_threads(&self) -> &'static [*mut Il2CppThread] {
        let mut size = 0;
        let ptr = unsafe { (self.0.vtable.il2cpp_get_attached_threads)(&mut size) };
        unsafe { std::slice::from_raw_parts(ptr, size) }
    }

    pub fn schedule_on_thread(&self, thread: *mut Il2CppThread, callback: fn()) {
        unsafe { (self.0.vtable.il2cpp_schedule_on_thread)(thread, std::mem::transmute(callback)) }
    }

    pub fn create_array(
        &self, element_type: *mut Il2CppClass, length: il2cpp_array_size_t
    ) -> *mut Il2CppArray {
        unsafe { (self.0.vtable.il2cpp_create_array)(element_type, length) }
    }

    pub fn get_singleton_like_instance(&self, class: *mut Il2CppClass) -> *mut Il2CppObject {
        unsafe { (self.0.vtable.il2cpp_get_singleton_like_instance)(class) }
    }
}

#[cfg(feature = "il2cpp_api")]
impl crate::il2cpp::resolve_api::SymbolResolver for HachimiApi {
    fn il2cpp_resolve_symbol(&self, name: &CStr) -> usize {
        self.il2cpp().resolve_symbol(name)
    }
}

#[repr(i32)]
pub enum LogLevel {
    Error = 1,
    Warn,
    Info,
    Debug,
    Trace
}

pub struct Hachimi<'a> {
    api: &'a HachimiApi,
    ptr: *const sys::Hachimi
}

impl<'a> Hachimi<'a> {
    pub unsafe fn from_raw(api: &'a HachimiApi, ptr: *const sys::Hachimi) -> Self {
        Self { api, ptr }
    }

    pub fn as_raw(&self) -> *const sys::Hachimi {
        self.ptr
    }

    pub fn instance(api: &'a HachimiApi) -> Self {
        Self {
            api,
            ptr: unsafe { (api.vtable.hachimi_instance)() }
        }
    }

    pub fn interceptor(&self) -> Interceptor {
        unsafe { Interceptor::from_raw(self.api, (self.api.vtable.hachimi_get_interceptor)(self.ptr)) }
    }
}

pub struct Interceptor<'a> {
    api: &'a HachimiApi,
    ptr: *const sys::Interceptor
}

impl<'a> Interceptor<'a> {
    pub unsafe fn from_raw(api: &'a HachimiApi, ptr: *const sys::Interceptor) -> Self {
        Self { api, ptr }
    }

    pub fn as_raw(&self) -> *const sys::Interceptor {
        self.ptr
    }

    pub fn hook(&self, orig_addr: usize, hook_addr: usize) -> Option<usize> {
        let res = unsafe { (self.api.vtable.interceptor_hook)(self.ptr, orig_addr as _, hook_addr as _) };
        (!res.is_null()).then(|| res as _)
    }

    pub fn hook_vtable(&self, vtable: *mut usize, vtable_index: usize, hook_addr: usize) -> Option<usize> {
        let res = unsafe { (self.api.vtable.interceptor_hook_vtable)(self.ptr, vtable as _, vtable_index, hook_addr as _) };
        (!res.is_null()).then(|| res as _)
    }

    pub fn get_trampoline_addr(&self, hook_addr: usize) -> Option<usize> {
        let res = unsafe { (self.api.vtable.interceptor_get_trampoline_addr)(self.ptr, hook_addr as _) };
        (!res.is_null()).then(|| res as _)
    }

    pub fn unhook(&self, hook_addr: usize) -> Option<usize> {
        let res = unsafe { (self.api.vtable.interceptor_unhook)(self.ptr, hook_addr as _) };
        (!res.is_null()).then(|| res as _)
    }
}