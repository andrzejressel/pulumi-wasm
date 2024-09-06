mod random_bytes;
mod random_id;
mod random_integer;
mod random_password;
mod random_pet;
mod random_shuffle;
mod random_string;
mod random_uuid;

mod bindings;
bindings::export!(Component with_types_in bindings);

struct Component {}
