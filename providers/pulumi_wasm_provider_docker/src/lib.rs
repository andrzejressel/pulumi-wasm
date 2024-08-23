mod container;
mod image;
mod network;
mod plugin;
mod registry_image;
mod remote_image;
mod secret;
mod service;
mod service_config;
mod tag;
mod volume;

mod bindings;
bindings::export!(Component with_types_in bindings);

struct Component {}
