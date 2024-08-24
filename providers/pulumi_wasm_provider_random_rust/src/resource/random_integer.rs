//! The resource `random.RandomInteger` generates random values from a given range, described by the `min` and `max` attributes of a given resource.
//!
//! This resource can be used in conjunction with resources that have the `create_before_destroy` lifecycle flag set, to avoid conflicts with unique names during the brief period where both the old and new resources exist concurrently.
//!
//! ## Example Usage
//!
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as aws from "@pulumi/aws";
//! import * as random from "@pulumi/random";
//!
//! // The following example shows how to generate a random priority
//! // between 1 and 50000 for a aws_alb_listener_rule resource:
//! const priority = new random.RandomInteger("priority", {
//!     min: 1,
//!     max: 50000,
//!     keepers: {
//!         listener_arn: _var.listener_arn,
//!     },
//! });
//! const main = new aws.alb.ListenerRule("main", {
//!     listenerArn: priority.keepers.apply(keepers => keepers?.listenerArn),
//!     priority: priority.result,
//!     actions: [{
//!         type: "forward",
//!         targetGroupArn: _var.target_group_arn,
//!     }],
//! });
//! // ... (other aws_alb_listener_rule arguments) ...
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_aws as aws
//! import pulumi_random as random
//!
//! # The following example shows how to generate a random priority
//! # between 1 and 50000 for a aws_alb_listener_rule resource:
//! priority = random.RandomInteger("priority",
//!     min=1,
//!     max=50000,
//!     keepers={
//!         "listener_arn": var["listener_arn"],
//!     })
//! main = aws.alb.ListenerRule("main",
//!     listener_arn=priority.keepers["listenerArn"],
//!     priority=priority.result,
//!     actions=[aws.alb.ListenerRuleActionArgs(
//!         type="forward",
//!         target_group_arn=var["target_group_arn"],
//!     )])
//! # ... (other aws_alb_listener_rule arguments) ...
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
//!     // The following example shows how to generate a random priority
//!     // between 1 and 50000 for a aws_alb_listener_rule resource:
//!     var priority = new Random.RandomInteger("priority", new()
//!     {
//!         Min = 1,
//!         Max = 50000,
//!         Keepers =
//!         {
//!             { "listener_arn", @var.Listener_arn },
//!         },
//!     });
//!
//!     var main = new Aws.Alb.ListenerRule("main", new()
//!     {
//!         ListenerArn = priority.Keepers.Apply(keepers => keepers?.ListenerArn),
//!         Priority = priority.Result,
//!         Actions = new[]
//!         {
//!             new Aws.Alb.Inputs.ListenerRuleActionArgs
//!             {
//!                 Type = "forward",
//!                 TargetGroupArn = @var.Target_group_arn,
//!             },
//!         },
//!     });
//!
//!     // ... (other aws_alb_listener_rule arguments) ...
//! });
//! ```
//! ### Go
//! ```go
//! package main
//!
//! import (
//! 	"github.com/pulumi/pulumi-aws/sdk/v5/go/aws/alb"
//! 	"github.com/pulumi/pulumi-random/sdk/v4/go/random"
//! 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
//! )
//!
//! func main() {
//! 	pulumi.Run(func(ctx *pulumi.Context) error {
//! 		priority, err := random.NewRandomInteger(ctx, "priority", &random.RandomIntegerArgs{
//! 			Min: pulumi.Int(1),
//! 			Max: pulumi.Int(50000),
//! 			Keepers: pulumi.StringMap{
//! 				"listener_arn": pulumi.Any(_var.Listener_arn),
//! 			},
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		_, err = alb.NewListenerRule(ctx, "main", &alb.ListenerRuleArgs{
//! 			ListenerArn: priority.Keepers.ApplyT(func(keepers interface{}) (*string, error) {
//! 				return &keepers.ListenerArn, nil
//! 			}).(pulumi.StringPtrOutput),
//! 			Priority: priority.Result,
//! 			Actions: alb.ListenerRuleActionArray{
//! 				&alb.ListenerRuleActionArgs{
//! 					Type:           pulumi.String("forward"),
//! 					TargetGroupArn: pulumi.Any(_var.Target_group_arn),
//! 				},
//! 			},
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
//! import com.pulumi.random.RandomInteger;
//! import com.pulumi.random.RandomIntegerArgs;
//! import com.pulumi.aws.alb.ListenerRule;
//! import com.pulumi.aws.alb.ListenerRuleArgs;
//! import com.pulumi.aws.alb.inputs.ListenerRuleActionArgs;
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
//!         var priority = new RandomInteger("priority", RandomIntegerArgs.builder()        
//!             .min(1)
//!             .max(50000)
//!             .keepers(Map.of("listener_arn", var_.listener_arn()))
//!             .build());
//!
//!         var main = new ListenerRule("main", ListenerRuleArgs.builder()        
//!             .listenerArn(priority.keepers().applyValue(keepers -> keepers.listenerArn()))
//!             .priority(priority.result())
//!             .actions(ListenerRuleActionArgs.builder()
//!                 .type("forward")
//!                 .targetGroupArn(var_.target_group_arn())
//!                 .build())
//!             .build());
//!
//!     }
//! }
//! ```
//! ### YAML
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

pub struct RandomIntegerArgs {
    /// Arbitrary map of values that, when changed, will trigger recreation of resource. See the main provider documentation for more information.
    pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    /// The maximum inclusive value of the range.
    pub max: pulumi_wasm_rust::Output<i32>,
    /// The minimum inclusive value of the range.
    pub min: pulumi_wasm_rust::Output<i32>,
    /// A custom seed to always produce the same value.
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
    let result = crate::bindings::pulumi::random::random_integer::invoke(
        name,
        &crate::bindings::pulumi::random::random_integer::Args {
            keepers: args.keepers.get_inner(),
            max: args.max.get_inner(),
            min: args.min.get_inner(),
            seed: args.seed.get_inner(),
        },
    );

    RandomIntegerResult {
        keepers: crate::into_domain(result.keepers),
        max: crate::into_domain(result.max),
        min: crate::into_domain(result.min),
        result: crate::into_domain(result.result),
        seed: crate::into_domain(result.seed),
    }
}
