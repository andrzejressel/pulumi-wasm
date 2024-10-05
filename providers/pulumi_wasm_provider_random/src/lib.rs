mod resource;

#[allow(unused_braces)]
#[allow(unused_imports)]
mod bindings;
bindings::export!(Component with_types_in bindings);

struct Component {}
