//! The resource `random.RandomUuid` generates a random uuid string that is intended to be used as a unique identifier for other resources.
//! 
//! This resource uses [hashicorp/go-uuid](https://github.com/hashicorp/go-uuid) to generate a UUID-formatted string for use with services needing a unique string identifier.
//! 
//! ## Example Usage
//! 
//! ```yaml
//! resources:
//!   testRandomUuid:
//!     type: random:RandomUuid
//!   testResourceGroup:
//!     type: azure:core:ResourceGroup
//!     properties:
//!       location: Central US
//! ```
//! 
//! ## Import
//! 
//! Random UUID's can be imported. This can be used to replace a config value with a value interpolated from the random provider without experiencing diffs.
//! 
//! ```sh
//!  $ pulumi import random:index/randomUuid:RandomUuid main aabbccdd-eeff-0011-2233-445566778899
//! ```
//! 
//!  

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct RandomUuidArgs {
    /// Arbitrary map of values that, when changed, will trigger recreation of resource. See the main provider documentation for more information.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
}

pub struct RandomUuidResult {
    /// Arbitrary map of values that, when changed, will trigger recreation of resource. See the main provider documentation for more information.
    pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    /// The generated uuid presented in string format.
    pub result: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: RandomUuidArgs) -> RandomUuidResult {

    let result = crate::bindings::pulumi::random::random_uuid::invoke(name, &crate::bindings::pulumi::random::random_uuid::Args {
        keepers: &args.keepers.get_inner(),
    });

    RandomUuidResult {
        keepers: crate::into_domain(result.keepers),
        result: crate::into_domain(result.result),
    }
}
