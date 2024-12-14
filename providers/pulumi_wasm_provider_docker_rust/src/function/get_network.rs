//! `docker.Network` provides details about a specific Docker Network.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let main = get_network::invoke(
//!         GetNetworkArgs::builder().name("main").build_struct(),
//!     );
//! }
//! ```

    #[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct GetNetworkArgs {
    /// The name of the Docker network.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
}

pub struct GetNetworkResult {
    /// The driver of the Docker network. Possible values are `bridge`, `host`, `overlay`, `macvlan`. See [network docs](https://docs.docker.com/network/#network-drivers) for more details.
    pub driver: pulumi_wasm_rust::Output<String>,
    /// The ID of this resource.
    pub id: pulumi_wasm_rust::Output<String>,
    /// If `true`, the network is internal.
    pub internal: pulumi_wasm_rust::Output<bool>,
    /// The IPAM configuration options
    pub ipam_configs: pulumi_wasm_rust::Output<Vec<crate::types::GetNetworkIpamConfig>>,
    /// The name of the Docker network.
    pub name: pulumi_wasm_rust::Output<String>,
    /// Only available with bridge networks. See [bridge options docs](https://docs.docker.com/engine/reference/commandline/network_create/#bridge-driver-options) for more details.
    pub options: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    /// Scope of the network. One of `swarm`, `global`, or `local`.
    pub scope: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: GetNetworkArgs
) -> GetNetworkResult {

    let result = crate::bindings::pulumi::docker::get_network::invoke(
        &crate::bindings::pulumi::docker::get_network::Args {
                name: &args.name.get_inner(),
        }
    );

    GetNetworkResult {
        driver: crate::into_domain(result.driver),
        id: crate::into_domain(result.id),
        internal: crate::into_domain(result.internal),
        ipam_configs: crate::into_domain(result.ipam_configs),
        name: crate::into_domain(result.name),
        options: crate::into_domain(result.options),
        scope: crate::into_domain(result.scope),
    }
}
