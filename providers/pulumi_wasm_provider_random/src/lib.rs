mod resource;

#[allow(unused_braces)]
mod bindings;
bindings::export!(Component with_types_in bindings);

struct Component {}
