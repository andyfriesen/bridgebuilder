use bridgebuilder_attrs::export_enum;
use std::alloc::{Layout, alloc, dealloc};

#[derive(Default)]
#[export_enum]
pub enum Foo {
    #[default]
    Nil,
    Boolean(bool),
    Integer(i32)
    // Vec3(Vector3),
}

#[unsafe(no_mangle)]
pub extern "C" fn make_Foo(variant: i32, value: i32) -> *mut Foo {
    let res = if variant == 0 {
        Foo::Nil
    } else if variant == 1 {
        Foo::Boolean(value != 0)
    } else {
        Foo::Integer(value)
    };

    unsafe {
        let layout = Layout::new::<Foo>();
        let ptr = alloc(layout) as *mut Foo;
        *ptr = res;
        return ptr;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn drop_Foo(this: *mut Foo) {
    // core::mem::drop(*this); // ???

    let layout = Layout::new::<Foo>();
    unsafe {
        dealloc(this as *mut u8, layout);
    }
}

/*

C++

struct Vector3 {
    double x;
    double y;
    double z;
}

struct Foo {
    struct Nil {};
    type Boolean = bool;
    type Vector3 = ::Vector3;
};

extern "C" const void* seedoubleplus_FooToNil(const Foo*);
extern "C" const bool* seedoubleplus_FooToBool(const Foo*);
extern "C" const Vector3* seedoubleplus_FooToVector3(const Foo*);

template <typename T> const T* get(const Foo* self);
template <> auto get<Foo::Nil>(const Foo* self) -> const void* { return seedoubleplus_FooToNil(self); }
template <> auto get<Foo::Bool>(const Foo* self) -> const bool* { return seedoubleplus_FooToBoolean(self); }
template <> auto get<Foo::Vector3>(const Foo* self) -> const Vector3* { return seedoubleplus_FooToVector3(self); }

*/

/*

Rust

#[no_mangle] pub extern "C" fn seedoubleplus_FooToNil<'a>(self: &'a Foo) -> &'a void // ???

#[no_mangle] pub extern "C" fn seedoubleplus_FooToBoolean(self: &Foo) -> Option<&bool> {
   match &self {
       Boolean(a) => Some(&a),
       _ => None
   },

}

#[no_mangle] pub extern "C" fn seedoubleplus_FooToVector3<'a>(self: &'a Foo) -> &'a Vector3

extern "C" seedoubleplus_FooToNil(const Foo*);
extern "C" seedoubleplus_FooToTrue(const Foo*);
extern "C" seedoubleplus_FooToFalse(const Foo*);

*/
