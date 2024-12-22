//! The resource `random.RandomPet` generates random pet names that are intended to be used as unique identifiers for other resources.
//! 
//! This resource can be used in conjunction with resources that have the `create_before_destroy` lifecycle flag set, to avoid conflicts with unique names during the brief period where both the old and new resources exist concurrently.
//! 
//! ## Example Usage
//! 
//! ```yaml
//! resources:
//!   # The following example shows how to generate a unique pet name
//!   # for an AWS EC2 instance that changes each time a new AMI id is
//!   # selected.
//!   serverRandomPet:
//!     type: random:RandomPet
//!     properties:
//!       keepers:
//!         ami_id: ${var.ami_id}
//!   serverInstance:
//!     type: aws:ec2:Instance
//!     properties:
//!       tags:
//!         Name: web-server-${serverRandomPet.id}
//!       # Read the AMI id "through" the random_pet resource to ensure that
//!       #   # both will change together.
//!       ami: ${serverRandomPet.keepers.amiId}
//! ```

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct RandomPetArgs {
    /// Arbitrary map of values that, when changed, will trigger recreation of resource. See the main provider documentation for more information.
    #[builder(into, default)]
    pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    /// The length (in words) of the pet name. Defaults to 2
    #[builder(into, default)]
    pub length: pulumi_wasm_rust::Output<Option<i32>>,
    /// A string to prefix the name with.
    #[builder(into, default)]
    pub prefix: pulumi_wasm_rust::Output<Option<String>>,
    /// The character to separate words in the pet name. Defaults to "-"
    #[builder(into, default)]
    pub separator: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct RandomPetResult {
    /// Arbitrary map of values that, when changed, will trigger recreation of resource. See the main provider documentation for more information.
    pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    /// The length (in words) of the pet name. Defaults to 2
    pub length: pulumi_wasm_rust::Output<i32>,
    /// A string to prefix the name with.
    pub prefix: pulumi_wasm_rust::Output<Option<String>>,
    /// The character to separate words in the pet name. Defaults to "-"
    pub separator: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(
    name: &str,
    args: RandomPetArgs
) -> RandomPetResult {

    let result = crate::bindings::pulumi::random::random_pet::invoke(
        name,
        &crate::bindings::pulumi::random::random_pet::Args {
                keepers: &args.keepers.get_inner(),
                length: &args.length.get_inner(),
                prefix: &args.prefix.get_inner(),
                separator: &args.separator.get_inner(),
        }
    );

    RandomPetResult {
        keepers: crate::into_domain(result.keepers),
        length: crate::into_domain(result.length),
        prefix: crate::into_domain(result.prefix),
        separator: crate::into_domain(result.separator),
    }
}
