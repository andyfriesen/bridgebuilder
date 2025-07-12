use std::default::Default;
use bridgebuilder_attrs::export_enum;

#[derive(Default)]
// #[seedoubleplus_export]
#[repr(C)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub enum IgnoreMe {
    One, Two
}

#[derive(Default)]
#[export_enum]
pub enum Foo {
    #[default]
    Nil,
    Boolean(bool),
    Integer(i32)
    // Vec3(Vector3),
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

fn main() {

    println!("Hello, world!");
}
