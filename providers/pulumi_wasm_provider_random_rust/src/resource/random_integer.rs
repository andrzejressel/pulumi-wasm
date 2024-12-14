//! The resource `random.RandomInteger` generates random values from a given range, described by the `min` and `max` attributes of a given resource.
//! 
//! This resource can be used in conjunction with resources that have the `create_before_destroy` lifecycle flag set, to avoid conflicts with unique names during the brief period where both the old and new resources exist concurrently.
//! 
//! ## Example Usage
//! 
//! ```yaml
//! resources:
//!   # The following example shows how to generate a random priority
//!   # between 1 and 50000 for a aws_alb_listener_rule resource:
//!   priority:
//!     type: random:RandomInteger
//!     properties:
//!       min: 1
//!       max: 50000
//!       keepers:
//!         listener_arn: ${var.listener_arn}
//!   main:
//!     type: aws:alb:ListenerRule
//!     properties:
//!       listenerArn: ${priority.keepers.listenerArn}
//!       priority: ${priority.result}
//!       actions:
//!         - type: forward
//!           targetGroupArn: ${var.target_group_arn}
//! ```
//! 
//! ## Import
//! 
//! Random integers can be imported using the result, min, and max, with an optional seed. This can be used to replace a config value with a value interpolated from the random provider without experiencing diffs. Example (values are separated by a ,)
//! 
//! ```sh
//!  $ pulumi import random:index/randomInteger:RandomInteger priority 15390,1,50000
//! ```
//! 
//!  

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct RandomIntegerArgs {
    /// Arbitrary map of values that, when changed, will trigger recreation of resource. See the main provider documentation for more information.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    /// The maximum inclusive value of the range.
    #[builder(into)]
    pub max: pulumi_wasm_rust::Output<i32>,
    /// The minimum inclusive value of the range.
    #[builder(into)]
    pub min: pulumi_wasm_rust::Output<i32>,
    /// A custom seed to always produce the same value.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub seed: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct RandomIntegerResult {
    /// Arbitrary map of values that, when changed, will trigger recreation of resource. See the main provider documentation for more information.
    pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    /// The maximum inclusive value of the range.
    pub max: pulumi_wasm_rust::Output<i32>,
    /// The minimum inclusive value of the range.
    pub min: pulumi_wasm_rust::Output<i32>,
    /// The random integer result.
    pub result: pulumi_wasm_rust::Output<i32>,
    /// A custom seed to always produce the same value.
    pub seed: pulumi_wasm_rust::Output<Option<String>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: RandomIntegerArgs) -> RandomIntegerResult {

    let result = crate::bindings::pulumi::random::random_integer::invoke(name, &crate::bindings::pulumi::random::random_integer::Args {
        keepers: &args.keepers.get_inner(),
        max: &args.max.get_inner(),
        min: &args.min.get_inner(),
        seed: &args.seed.get_inner(),
    });

    RandomIntegerResult {
        keepers: crate::into_domain(result.keepers),
        max: crate::into_domain(result.max),
        min: crate::into_domain(result.min),
        result: crate::into_domain(result.result),
        seed: crate::into_domain(result.seed),
    }
}
