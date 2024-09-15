mod resource;
use resource::*;

mod bindings;
bindings::export!(Component with_types_in bindings);

struct Component {}
