#![allow(clippy::all)]
#![allow(clippy::pedantic)]
pub mod grpc {
    #[cfg(not(feature = "connectivity"))]
    include!(concat!(env!("OUT_DIR"), "/mini/pulumirpc.rs"));

    #[cfg(feature = "connectivity")]
    include!(concat!(env!("OUT_DIR"), "/full/pulumirpc.rs"));
}
