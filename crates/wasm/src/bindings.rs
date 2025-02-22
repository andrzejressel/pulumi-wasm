#![allow(clippy::all)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_unsafe)]

use crate::Component;

wit_bindgen::generate!({
    world: "pulumi-gestalt",
    generate_all
});

export!(Component);
