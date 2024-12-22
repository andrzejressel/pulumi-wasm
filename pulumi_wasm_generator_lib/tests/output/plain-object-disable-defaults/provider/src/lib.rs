mod resource;
mod function;

#[allow(unused_braces)]
#[allow(unused_imports)]
#[allow(static_mut_refs)]
mod bindings;
bindings::export!(Component with_types_in bindings);

struct Component {}
