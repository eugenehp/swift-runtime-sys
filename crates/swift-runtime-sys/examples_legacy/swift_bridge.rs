use std::ffi::{c_char, c_void, CStr};

#[repr(transparent)]
pub struct SwiftPerson(*mut c_void);

#[repr(transparent)]
pub struct SwiftCounter(*mut c_void);

unsafe extern "C" {
    fn swift_person_new(id: i32, age: i32) -> *mut c_void;
    fn swift_person_get_id(person: *mut c_void) -> i32;
    fn swift_person_get_age(person: *mut c_void) -> i32;
    fn swift_person_drop(person: *mut c_void);

    fn swift_counter_new(start: i32) -> *mut c_void;
    fn swift_counter_increment(counter: *mut c_void, delta: i32) -> i32;
    fn swift_counter_drop(counter: *mut c_void);

    fn swift_add(a: i32, b: i32) -> i32;
    fn swift_greet() -> *const c_char;
    fn swift_string_free(ptr: *mut c_char);
}

impl SwiftPerson {
    pub fn new(id: i32, age: i32) -> Self {
        let raw = unsafe { swift_person_new(id, age) };
        assert!(!raw.is_null(), "swift_person_new returned null");
        Self(raw)
    }

    pub fn id(&self) -> i32 {
        unsafe { swift_person_get_id(self.0) }
    }

    pub fn age(&self) -> i32 {
        unsafe { swift_person_get_age(self.0) }
    }
}

impl Drop for SwiftPerson {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { swift_person_drop(self.0) };
            self.0 = std::ptr::null_mut();
        }
    }
}

impl SwiftCounter {
    pub fn new(start: i32) -> Self {
        let raw = unsafe { swift_counter_new(start) };
        assert!(!raw.is_null(), "swift_counter_new returned null");
        Self(raw)
    }

    pub fn increment(&mut self, delta: i32) -> i32 {
        unsafe { swift_counter_increment(self.0, delta) }
    }
}

impl Drop for SwiftCounter {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { swift_counter_drop(self.0) };
            self.0 = std::ptr::null_mut();
        }
    }
}

pub fn swift_add_two(a: i32, b: i32) -> i32 {
    unsafe { swift_add(a, b) }
}

pub fn swift_greeting() -> Option<String> {
    let ptr = unsafe { swift_greet() };
    if ptr.is_null() {
        return None;
    }

    let message = unsafe { CStr::from_ptr(ptr) }
        .to_string_lossy()
        .into_owned();

    unsafe { swift_string_free(ptr as *mut c_char) };
    Some(message)
}

fn main() {
    let person = SwiftPerson::new(7, 42);
    println!("person.id={} person.age={}", person.id(), person.age());

    let mut counter = SwiftCounter::new(10);
    println!("counter={} ", counter.increment(5));

    println!("add={}", swift_add_two(20, 22));
    println!("greeting={:?}", swift_greeting());
}
