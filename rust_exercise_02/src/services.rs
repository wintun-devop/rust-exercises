use std::any::type_name;


pub fn get_type<T>(_: &T) -> &'static str {
    return type_name::<T>()
}