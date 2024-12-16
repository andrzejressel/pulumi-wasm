//! The resource `random.RandomShuffle` generates a random permutation of a list of strings given as an argument.
//! 
//! ## Example Usage
//! 
//! ```yaml
//! resources:
//!   az:
//!     type: random:RandomShuffle
//!     properties:
//!       inputs:
//!         - us-west-1a
//!         - us-west-1c
//!         - us-west-1d
//!         - us-west-1e
//!       resultCount: 2
//!   example:
//!     type: aws:elb:LoadBalancer
//!     properties:
//!       # Place the ELB in any two of the given availability zones, selected
//!       #   # at random.
//!       availabilityZones: ${az.results}
//! ```

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct RandomShuffleArgs {
    /// The list of strings to shuffle.
    #[builder(into)]
    pub inputs: pulumi_wasm_rust::Output<Vec<String>>,
    /// Arbitrary map of values that, when changed, will trigger recreation of resource. See the main provider documentation for more information.
    #[builder(into, default)]
    pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    /// The number of results to return. Defaults to the number of items in the `input` list. If fewer items are requested, some elements will be excluded from the result. If more items are requested, items will be repeated in the result but not more frequently than the number of items in the input list.
    #[builder(into, default)]
    pub result_count: pulumi_wasm_rust::Output<Option<i32>>,
    /// Arbitrary string with which to seed the random number generator, in order to produce less-volatile permutations of the list.
    #[builder(into, default)]
    pub seed: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct RandomShuffleResult {
    /// The list of strings to shuffle.
    pub inputs: pulumi_wasm_rust::Output<Vec<String>>,
    /// Arbitrary map of values that, when changed, will trigger recreation of resource. See the main provider documentation for more information.
    pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    /// The number of results to return. Defaults to the number of items in the `input` list. If fewer items are requested, some elements will be excluded from the result. If more items are requested, items will be repeated in the result but not more frequently than the number of items in the input list.
    pub result_count: pulumi_wasm_rust::Output<Option<i32>>,
    /// Random permutation of the list of strings given in `input`.
    pub results: pulumi_wasm_rust::Output<Vec<String>>,
    /// Arbitrary string with which to seed the random number generator, in order to produce less-volatile permutations of the list.
    pub seed: pulumi_wasm_rust::Output<Option<String>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: RandomShuffleArgs) -> RandomShuffleResult {

    let result = crate::bindings::pulumi::random::random_shuffle::invoke(name, &crate::bindings::pulumi::random::random_shuffle::Args {
        inputs: &args.inputs.get_inner(),
        keepers: &args.keepers.get_inner(),
        result_count: &args.result_count.get_inner(),
        seed: &args.seed.get_inner(),
    });

    RandomShuffleResult {
        inputs: crate::into_domain(result.inputs),
        keepers: crate::into_domain(result.keepers),
        result_count: crate::into_domain(result.result_count),
        results: crate::into_domain(result.results),
        seed: crate::into_domain(result.seed),
    }
}
