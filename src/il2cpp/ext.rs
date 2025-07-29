use widestring::{Utf16Str, Utf16String};

use crate::{api::HachimiIl2CppApi, il2cpp::{resolve_api::{self, SymbolResolver}, types::*}};

pub trait StringExt {
    fn to_il2cpp_string(&self, resolver: &impl SymbolResolver) -> Option<*mut Il2CppString>;
}

impl StringExt for str {
    fn to_il2cpp_string(&self, resolver: &impl SymbolResolver) -> Option<*mut Il2CppString> {
        let text_utf16 = Utf16String::from_str(self);
        let len = text_utf16.len().try_into().ok()?;
        resolve_api::il2cpp_string_new_utf16(resolver)
            .map(|f| f(text_utf16.as_ptr(), len))
    }
}

impl StringExt for String {
    fn to_il2cpp_string(&self, resolver: &impl SymbolResolver) -> Option<*mut Il2CppString> {
        str::to_il2cpp_string(self, resolver)
    }
}

pub trait Il2CppStringExt {
    fn chars_ptr(&self) -> *const Il2CppChar;
    fn as_utf16str(&self) -> &Utf16Str;
}

#[cfg(feature = "il2cpp")]
impl Il2CppStringExt for Il2CppString {
    fn chars_ptr(&self) -> *const Il2CppChar {
        self.chars.as_ptr()
    }

    fn as_utf16str(&self) -> &Utf16Str {
        unsafe { Utf16Str::from_slice_unchecked(std::slice::from_raw_parts(self.chars.as_ptr(), self.length as usize)) }
    }
}

#[cfg(feature = "il2cpp_2020")]
impl Il2CppStringExt for Il2CppString {
    fn chars_ptr(&self) -> *const Il2CppChar {
        &self.chars
    }

    fn as_utf16str(&self) -> &Utf16Str {
        unsafe { Utf16Str::from_slice_unchecked(std::slice::from_raw_parts(&self.chars, self.length as usize)) }
    }
}

pub trait Il2CppObjectExt {
    fn klass(&self) -> *mut Il2CppClass;
}

#[cfg(feature = "il2cpp")]
impl Il2CppObjectExt for Il2CppObject {
    fn klass(&self) -> *mut Il2CppClass {
        unsafe { *self.__bindgen_anon_1.klass.as_ref() }
    }
}

#[cfg(feature = "il2cpp_2020")]
impl Il2CppObjectExt for Il2CppObject {
    fn klass(&self) -> *mut Il2CppClass {
        unsafe { self.__bindgen_anon_1.klass }
    }
}

pub trait HachimiIl2CppApiExt {
    fn create_delegate(&self, delegate_class: *mut Il2CppClass, args_count: i32, method_ptr: fn()) -> Option<*mut Il2CppDelegate>;
}

impl<'a> HachimiIl2CppApiExt for HachimiIl2CppApi<'a> {
    fn create_delegate(&self, delegate_class: *mut Il2CppClass, args_count: i32, method_ptr: fn()) -> Option<*mut Il2CppDelegate> {
        let delegate_invoke = self.get_method_cached(delegate_class, c"Invoke", args_count);
        if delegate_invoke.is_null() { return None; }

        let delegate_ctor_addr = self.get_method_addr_cached(delegate_class, c".ctor", 2);
        if delegate_ctor_addr == 0 {
            return None;
        }
        let delegate_ctor: extern "C" fn(*mut Il2CppObject, *mut Il2CppObject, *const MethodInfo) = unsafe {
            std::mem::transmute(delegate_ctor_addr)
        };

        let delegate_obj = resolve_api::il2cpp_object_new(self).map(|f| f(delegate_class))?;
        delegate_ctor(delegate_obj, delegate_obj, delegate_invoke);
        let delegate = delegate_obj as *mut Il2CppDelegate;
        unsafe {
            (*delegate).method_ptr = method_ptr as _;
        }

        Some(delegate)
    }
}