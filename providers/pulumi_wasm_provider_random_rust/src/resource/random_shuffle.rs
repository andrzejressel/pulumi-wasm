//! The resource `random.RandomShuffle` generates a random permutation of a list of strings given as an argument.
//! 
//! ## Example Usage
//! 
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as aws from "@pulumi/aws";
//! import * as random from "@pulumi/random";
//! 
//! const az = new random.RandomShuffle("az", {
//!     inputs: [
//!         "us-west-1a",
//!         "us-west-1c",
//!         "us-west-1d",
//!         "us-west-1e",
//!     ],
//!     resultCount: 2,
//! });
//! const example = new aws.elb.LoadBalancer("example", {availabilityZones: az.results});
//! // ... and other aws_elb arguments ...
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_aws as aws
//! import pulumi_random as random
//! 
//! az = random.RandomShuffle("az",
//!     inputs=[
//!         "us-west-1a",
//!         "us-west-1c",
//!         "us-west-1d",
//!         "us-west-1e",
//!     ],
//!     result_count=2)
//! example = aws.elb.LoadBalancer("example", availability_zones=az.results)
//! # ... and other aws_elb arguments ...
//! ```
//! ### C#
//! ```csharp
//! using System.Collections.Generic;
//! using System.Linq;
//! using Pulumi;
//! using Aws = Pulumi.Aws;
//! using Random = Pulumi.Random;
//! 
//! return await Deployment.RunAsync(() => 
//! {
//!     var az = new Random.RandomShuffle("az", new()
//!     {
//!         Inputs = new[]
//!         {
//!             "us-west-1a",
//!             "us-west-1c",
//!             "us-west-1d",
//!             "us-west-1e",
//!         },
//!         ResultCount = 2,
//!     });
//! 
//!     var example = new Aws.Elb.LoadBalancer("example", new()
//!     {
//!         AvailabilityZones = az.Results,
//!     });
//! 
//!     // ... and other aws_elb arguments ...
//! });
//! ```
//! ### Go
//! ```go
//! package main
//! 
//! import (
//! 	"github.com/pulumi/pulumi-aws/sdk/v5/go/aws/elb"
//! 	"github.com/pulumi/pulumi-random/sdk/v4/go/random"
//! 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
//! )
//! 
//! func main() {
//! 	pulumi.Run(func(ctx *pulumi.Context) error {
//! 		az, err := random.NewRandomShuffle(ctx, "az", &random.RandomShuffleArgs{
//! 			Inputs: pulumi.StringArray{
//! 				pulumi.String("us-west-1a"),
//! 				pulumi.String("us-west-1c"),
//! 				pulumi.String("us-west-1d"),
//! 				pulumi.String("us-west-1e"),
//! 			},
//! 			ResultCount: pulumi.Int(2),
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		_, err = elb.NewLoadBalancer(ctx, "example", &elb.LoadBalancerArgs{
//! 			AvailabilityZones: az.Results,
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		return nil
//! 	})
//! }
//! ```
//! ### Java
//! ```java
//! package generated_program;
//! 
//! import com.pulumi.Context;
//! import com.pulumi.Pulumi;
//! import com.pulumi.core.Output;
//! import com.pulumi.random.RandomShuffle;
//! import com.pulumi.random.RandomShuffleArgs;
//! import com.pulumi.aws.elb.LoadBalancer;
//! import com.pulumi.aws.elb.LoadBalancerArgs;
//! import java.util.List;
//! import java.util.ArrayList;
//! import java.util.Map;
//! import java.io.File;
//! import java.nio.file.Files;
//! import java.nio.file.Paths;
//! 
//! public class App {
//!     public static void main(String[] args) {
//!         Pulumi.run(App::stack);
//!     }
//! 
//!     public static void stack(Context ctx) {
//!         var az = new RandomShuffle("az", RandomShuffleArgs.builder()        
//!             .inputs(            
//!                 "us-west-1a",
//!                 "us-west-1c",
//!                 "us-west-1d",
//!                 "us-west-1e")
//!             .resultCount(2)
//!             .build());
//! 
//!         var example = new LoadBalancer("example", LoadBalancerArgs.builder()        
//!             .availabilityZones(az.results())
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
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

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct RandomShuffleArgs {
    /// The list of strings to shuffle.
    #[builder(into)]
    pub inputs: pulumi_wasm_rust::Output<Vec<String>>,
    /// Arbitrary map of values that, when changed, will trigger recreation of resource. See the main provider documentation for more information.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    /// The number of results to return. Defaults to the number of items in the `input` list. If fewer items are requested, some elements will be excluded from the result. If more items are requested, items will be repeated in the result but not more frequently than the number of items in the input list.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub result_count: pulumi_wasm_rust::Output<Option<i32>>,
    /// Arbitrary string with which to seed the random number generator, in order to produce less-volatile permutations of the list.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
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
