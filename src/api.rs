use std::ffi::CStr;

#[cfg(feature = "il2cpp_api")]
use crate::il2cpp::types::{Il2CppImage, Il2CppClass, MethodInfo, Il2CppTypeEnum};

use crate::sys;

#[derive(Debug, Clone, Copy)]
pub struct HachimiApi {
    vtable: sys::Vtable
}

impl HachimiApi {
    pub unsafe fn from_vtable(vtable: *mut sys::Vtable) -> Self {
        let vtable = (*vtable).clone();
        Self {
            vtable
        }
    }

    pub fn vtable(&self) -> &sys::Vtable {
        &self.vtable
    }

    pub fn il2cpp_resolve_symbol(&self, name: &CStr) -> usize {
        unsafe { (self.vtable.il2cpp_resolve_symbol)(name.as_ptr()) as _ }
    }

    pub fn log(&self, level: LogLevel, target: &CStr, message: &CStr) {
        unsafe { (self.vtable.log)(level as _, target.as_ptr(), message.as_ptr()) }
    }
}

#[cfg(feature = "il2cpp_api")]
impl HachimiApi {
    pub fn il2cpp_get_assembly_image(&self, assembly_name: &CStr) -> *const Il2CppImage {
        unsafe { (self.vtable.il2cpp_get_assembly_image)(assembly_name.as_ptr()) }
    }

    pub fn il2cpp_get_class(
        &self, image: *const Il2CppImage, namespace: &CStr, class_name: &CStr
    ) -> *mut Il2CppClass {
        unsafe { (self.vtable.il2cpp_get_class)(image, namespace.as_ptr(), class_name.as_ptr()) }
    }

    pub fn il2cpp_get_method(
        &self, class: *mut Il2CppClass, name: &CStr, args_count: i32
    ) -> *const MethodInfo {
        unsafe { (self.vtable.il2cpp_get_method)(class, name.as_ptr(), args_count) }
    }

    pub fn il2cpp_get_method_overload(
        &self, class: *mut Il2CppClass, name: &CStr, params: &[Il2CppTypeEnum]
    ) -> *const MethodInfo {
        unsafe { (self.vtable.il2cpp_get_method_overload)(class, name.as_ptr(), params.as_ptr(), params.len()) }
    }

    pub fn il2cpp_get_method_addr(
        &self, class: *mut Il2CppClass, name: &CStr, args_count: i32
    ) -> usize {
        unsafe { (self.vtable.il2cpp_get_method_addr)(class, name.as_ptr(), args_count) as _ }
    }

    pub fn il2cpp_get_method_overload_addr(
        &self, class: *mut Il2CppClass, name: &CStr, params: &[Il2CppTypeEnum]
    ) -> usize {
        unsafe { (self.vtable.il2cpp_get_method_overload_addr)(class, name.as_ptr(), params.as_ptr(), params.len()) as _ }
    }
}

#[cfg(feature = "il2cpp_api")]
impl crate::il2cpp::resolve_api::SymbolResolver for HachimiApi {
    fn il2cpp_resolve_symbol(&self, name: &CStr) -> usize {
        self.il2cpp_resolve_symbol(name)
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