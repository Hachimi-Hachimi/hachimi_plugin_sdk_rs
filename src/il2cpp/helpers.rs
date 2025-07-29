use std::{ffi::c_void, marker::PhantomData};

use crate::api::HachimiIl2CppApi;

use super::{ext::Il2CppObjectExt, types::*, resolve_api};

#[repr(transparent)]
pub struct IEnumerable<T = *mut Il2CppObject> {
    pub this: *mut Il2CppObject,
    _phantom: PhantomData<T>
}

impl<T> IEnumerable<T> {
    pub fn enumerator(&self, il2cpp: &HachimiIl2CppApi) -> Option<IEnumerator> {
        if self.this.is_null() {
            return None;
        }

        let class = unsafe { (*self.this).klass() };
        let get_enumerator_addr = il2cpp.get_method_addr_cached(class, c"GetEnumerator", 0);
        if get_enumerator_addr == 0 {
            return None;
        }
        
        let get_enumerator: extern "C" fn(*mut Il2CppObject) -> *mut Il2CppObject = unsafe {
            std::mem::transmute(get_enumerator_addr)
        };

        Some(IEnumerator::from(get_enumerator(self.this)))
    }
}

impl<T> From<*mut Il2CppObject> for IEnumerable<T> {
    fn from(value: *mut Il2CppObject) -> Self {
        IEnumerable {
            this: value,
            _phantom: PhantomData
        }
    }
}

#[repr(transparent)]
pub struct IEnumerator<T = *mut Il2CppObject> {
    pub this: *mut Il2CppObject,
    _phantom: PhantomData<T>
}

pub type MoveNextFn = extern "C" fn(*mut Il2CppObject) -> bool;

impl<T> IEnumerator<T> {
    pub fn iter(&self, il2cpp: &HachimiIl2CppApi) -> Option<IEnumeratorIterator<T>> {
        if self.this.is_null() {
            return None;
        }

        let class = unsafe { (*self.this).klass() };
        // Get addr manually to avoid nullptr warning
        let get_current_method = il2cpp.get_method_cached(class, c"get_Current", 0);
        let get_current_addr = if get_current_method.is_null() {
            0
        }
        else {
            unsafe { (*get_current_method).methodPointer }
        };
        let move_next_addr = il2cpp.get_method_addr_cached(class, c"MoveNext", 0);

        if move_next_addr == 0 {
            return None;
        }

        Some(IEnumeratorIterator {
            this: self.this,
            get_Current: unsafe { std::mem::transmute(get_current_addr) },
            MoveNext: unsafe { std::mem::transmute(move_next_addr) }
        })
    }
}

impl<T> From<*mut Il2CppObject> for IEnumerator<T> {
    fn from(value: *mut Il2CppObject) -> Self {
        IEnumerator {
            this: value,
            _phantom: PhantomData
        }
    }
}

#[allow(non_snake_case)]
pub struct IEnumeratorIterator<T> {
    this: *mut Il2CppObject,
    get_Current: Option<extern "C" fn(*mut Il2CppObject) -> T>,
    MoveNext: MoveNextFn
}

impl<T> Iterator for IEnumeratorIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: properly handle enumerators that returns nothing
        let Some(get_current) = self.get_Current else {
            return None;
        };

        if (self.MoveNext)(self.this) {
            Some(get_current(self.this))
        }
        else {
            None
        }
    }
}

#[allow(non_snake_case)]
pub struct IList<T = *mut Il2CppObject> {
    pub this: *mut Il2CppObject,
    get_Item: extern "C" fn(*mut Il2CppObject, i32) -> T,
    set_Item: extern "C" fn(*mut Il2CppObject, i32, T),
    get_Count: extern "C" fn(*mut Il2CppObject) -> i32
}

impl<T> IList<T> {
    pub fn new(this: *mut Il2CppObject, il2cpp: &HachimiIl2CppApi) -> Option<IList<T>> {
        if this.is_null() {
            return None;
        }

        let class = unsafe { (*this).klass() };
        let get_item_addr = il2cpp.get_method_addr_cached(class, c"get_Item", 1);
        let set_item_addr = il2cpp.get_method_addr_cached(class, c"set_Item", 2);
        let get_count_addr = il2cpp.get_method_addr_cached(class, c"get_Count", 0);

        if get_item_addr == 0 || set_item_addr == 0 || get_count_addr == 0 {
            return None;
        }       

        Some(IList {
            this,
            get_Item: unsafe { std::mem::transmute(get_item_addr) },
            set_Item: unsafe { std::mem::transmute(set_item_addr) },
            get_Count: unsafe { std::mem::transmute(get_count_addr) }
        })
    }

    /// Returns `None` if `i` is out of range.
    pub fn get(&self, i: i32) -> Option<T> {
        if i >= 0 && i < self.count() {
            Some((self.get_Item)(self.this, i))
        }
        else {
            None
        }
    }

    /// Returns `false` if `i` is out of range.
    pub fn set(&self, i: i32, value: T) -> bool {
        if i >= 0 && i < self.count() {
            (self.set_Item)(self.this, i, value);
            true
        }
        else {
            false
        }
    }

    pub fn count(&self) -> i32 {
        (self.get_Count)(self.this)
    }

    pub fn iter<'a>(&'a self) -> IListIter<'a, T> {
        IListIter { list: self, i: -1 }
    }
}

impl<'a, T> IntoIterator for &'a IList<T> {
    type Item = T;
    type IntoIter = IListIter<'a, T>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T> Into<Vec<T>> for IList<T> {
    fn into(self) -> Vec<T> {
        self.iter().collect()
    }
}

pub struct IListIter<'a, T> {
    list: &'a IList<T>,
    i: i32
}

impl<'a, T> Iterator for IListIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.i += 1;
        self.list.get(self.i)
    }
}

// IDictionary wrapper
#[allow(non_snake_case)]
pub struct IDictionary<K, V> {
    pub this: *mut Il2CppObject,
    get_Item: extern "C" fn(*mut Il2CppObject, K) -> V,
    set_Item: extern "C" fn(*mut Il2CppObject, K, V),
    Contains: extern "C" fn(*mut Il2CppObject, K) -> bool
}

impl<K, V> IDictionary<K, V> {
    pub fn new(this: *mut Il2CppObject, il2cpp: &HachimiIl2CppApi) -> Option<IDictionary<K, V>> {
        if this.is_null() {
            return None;
        }

        let class = unsafe { (*this).klass() };
        let get_item_addr = il2cpp.get_method_addr_cached(class, c"get_Item", 1);
        let set_item_addr = il2cpp.get_method_addr_cached(class, c"set_Item", 2);
        let contains_addr = il2cpp.get_method_addr_cached(class, c"Contains", 1);

        if get_item_addr == 0 || set_item_addr == 0 || contains_addr == 0 {
            return None;
        }

        Some(IDictionary {
            this,
            get_Item: unsafe { std::mem::transmute(get_item_addr) },
            set_Item: unsafe { std::mem::transmute(set_item_addr) },
            Contains: unsafe { std::mem::transmute(contains_addr) }
        })
    }

    pub fn get(&self, key: K) -> V {
        (self.get_Item)(self.this, key)
    }

    pub fn set(&self, key: K, value: V) {
        (self.set_Item)(self.this, key, value);
    }

    pub fn contains(&self, key: K) -> bool {
        (self.Contains)(self.this, key)
    }
}

// Il2CppThread wrapper
#[repr(transparent)]
#[derive(Clone)]
pub struct Thread(*mut Il2CppThread);

impl Thread {
    pub fn from_raw(ptr: *mut Il2CppThread) -> Self {
        Self(ptr)
    }

    pub fn schedule(&self, il2cpp: &HachimiIl2CppApi, callback: fn()) {
        il2cpp.schedule_on_thread(self.0, callback);
    }

    pub fn attached_threads(il2cpp: &HachimiIl2CppApi) -> &'static [Thread] {
        // SAFETY: Thread is repr(transparent) of *mut Il2CppThread
        unsafe { std::mem::transmute(il2cpp.get_attached_threads()) }
    }

    pub fn main_thread(il2cpp: &HachimiIl2CppApi) -> Thread {
        Self(il2cpp.get_main_thread())
    }

    pub fn as_raw(&self) -> *mut Il2CppThread {
        self.0
    }
}

// Il2CppArray wrapper
#[repr(transparent)]
pub struct Array<T = *mut Il2CppObject> {
    pub this: *mut Il2CppArray,
    _phantom: PhantomData<T>
}

impl<T> Array<T> {
    pub fn new(element_type: *mut Il2CppClass, length: il2cpp_array_size_t, il2cpp: &HachimiIl2CppApi) -> Array<T> {
        Array {
            this: il2cpp.create_array(element_type, length),
            _phantom: PhantomData,
        }
    }

    pub unsafe fn data_ptr(&self) -> *mut T {
        self.this.add(1) as _
    }

    pub unsafe fn as_slice(&self) -> &mut [T] {
        std::slice::from_raw_parts_mut(self.data_ptr(), (*self.this).max_length)
    }

    pub fn len(&self) -> usize {
        unsafe { (*self.this).max_length }
    }
}

impl<T> Into<*mut Il2CppArray> for Array<T> {
    fn into(self) -> *mut Il2CppArray {
        self.this
    }
}

impl<T> From<*mut Il2CppArray> for Array<T> {
    fn from(value: *mut Il2CppArray) -> Self {
        Self {
            this: value,
            _phantom: PhantomData
        }
    }
}

pub struct FieldsIter<'a> {
    class: *mut Il2CppClass,
    iter: *mut c_void,
    il2cpp: HachimiIl2CppApi<'a>
}

impl<'a> FieldsIter<'a> {
    pub fn new(class: *mut Il2CppClass, il2cpp: HachimiIl2CppApi<'a>) -> Self {
        Self {
            class,
            iter: 0 as _,
            il2cpp
        }
    }
}

impl<'a> Iterator for FieldsIter<'a> {
    type Item = *mut FieldInfo;

    fn next(&mut self) -> Option<Self::Item> {
        let field = resolve_api::il2cpp_class_get_fields(&self.il2cpp)
            .map(|f| f(self.class, &mut self.iter))?;
        if field.is_null() {
            return None;
        }
        Some(field)
    }
}

#[repr(C)]
pub struct Il2CppDictionary {
    pub obj: Il2CppObject,
    pub buckets: *mut Il2CppArray,
    pub entries: *mut Il2CppArray,
    pub count: i32,
    /* STUB */
}

#[repr(C)]
pub struct Il2CppDictionaryEntry<K, V> {
    pub hash_code: i32,
    pub next: i32,
    pub key: K,
    pub value: V
}

// Generic Dictionary wrapper
#[repr(transparent)]
pub struct Dictionary<K, V> {
    pub this: *mut Il2CppDictionary,
    _k: PhantomData<K>,
    _v: PhantomData<V>
}

impl<K, V> Into<*mut Il2CppDictionary> for Dictionary<K, V> {
    fn into(self) -> *mut Il2CppDictionary {
        self.this
    }
}

impl<K, V> From<*mut Il2CppDictionary> for Dictionary<K, V> {
    fn from(value: *mut Il2CppDictionary) -> Self {
        Self {
            this: value,
            _k: PhantomData,
            _v: PhantomData
        }
    }
}

impl<K, V> Dictionary<K, V> {
    pub fn buckets(&self) -> Array<i32> {
        unsafe { (*self.this).buckets.into() }
    }

    pub fn entries(&self) -> Array<Il2CppDictionaryEntry<K, V>> {
        unsafe { (*self.this).entries.into() }
    }

    pub fn count(&self) -> i32 {
        unsafe { (*self.this).count }
    }
}

impl<K: PartialEq, V> Dictionary<K, V> {
    pub fn find_entry(&self, key: &K) -> Option<&'static mut Il2CppDictionaryEntry<K, V>> {
        for entry in unsafe { self.entries().as_slice().iter_mut() } {
            if entry.key == *key {
                // freaky lifetime erasure
                return unsafe { std::ptr::from_mut(entry).as_mut() };
            }
        }

        None
    }
}

impl<K: PartialEq + 'static, V> Dictionary<K, V> {
    pub fn get(&self, key: &K) -> Option<&'static mut V> {
        self.find_entry(&key).map(|e| &mut e.value)
    }
}