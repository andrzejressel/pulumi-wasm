mod resource;

mod bindings {
    wit_bindgen::generate!({
    // the name of the world in the `*.wit` input file
        world: "random-pulumi",
        generate_all,
    });
}
bindings::export!(Component with_types_in bindings);

struct Component {}
