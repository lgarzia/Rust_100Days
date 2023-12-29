/// Incorporating Reflection 
/// https://crates.io/crates/bevy_reflect
use bevy_reflect::Reflect;

#[derive(Reflect)]
struct Foo {
    a: u32,
    b: f32,
    c: i32,
    d: String,
}


#[cfg(test)]
mod tests {
use bevy_reflect::{Struct, TypeInfo};
use super::*;

#[test]
fn simple_test(){
    let foo = Foo {
        a: 10, 
        b: 20.2, 
        c: -10, 
        d: "col".to_string()
    };
    
    for (i, value) in foo.iter_fields().enumerate() {
        let field_name = foo.name_at(i).unwrap();
        let type_info = value.get_represented_type_info().unwrap();
        println!("{}-{:?}-{}-{:?}", i, value, field_name, type_info.type_path_table().short_path())
//        if let Some(value) = value.downcast_ref::<u32>() {
//            println!("{} is a u32 with the value: {}", field_name, *value);
        }
    assert_eq!(1,2);
}
}