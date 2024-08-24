//! The resource `random.RandomPet` generates random pet names that are intended to be used as unique identifiers for other resources.
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
//! // The following example shows how to generate a unique pet name
//! // for an AWS EC2 instance that changes each time a new AMI id is
//! // selected.
//! const serverRandomPet = new random.RandomPet("serverRandomPet", {keepers: {
//!     ami_id: _var.ami_id,
//! }});
//! const serverInstance = new aws.ec2.Instance("serverInstance", {
//!     tags: {
//!         Name: pulumi.interpolate`web-server-${serverRandomPet.id}`,
//!     },
//!     ami: serverRandomPet.keepers.apply(keepers => keepers?.amiId),
//! });
//! // ... (other aws_instance arguments) ...
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_aws as aws
//! import pulumi_random as random
//! 
//! # The following example shows how to generate a unique pet name
//! # for an AWS EC2 instance that changes each time a new AMI id is
//! # selected.
//! server_random_pet = random.RandomPet("serverRandomPet", keepers={
//!     "ami_id": var["ami_id"],
//! })
//! server_instance = aws.ec2.Instance("serverInstance",
//!     tags={
//!         "Name": server_random_pet.id.apply(lambda id: f"web-server-{id}"),
//!     },
//!     ami=server_random_pet.keepers["amiId"])
//! # ... (other aws_instance arguments) ...
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
//!     // The following example shows how to generate a unique pet name
//!     // for an AWS EC2 instance that changes each time a new AMI id is
//!     // selected.
//!     var serverRandomPet = new Random.RandomPet("serverRandomPet", new()
//!     {
//!         Keepers = 
//!         {
//!             { "ami_id", @var.Ami_id },
//!         },
//!     });
//! 
//!     var serverInstance = new Aws.Ec2.Instance("serverInstance", new()
//!     {
//!         Tags = 
//!         {
//!             { "Name", serverRandomPet.Id.Apply(id => $"web-server-{id}") },
//!         },
//!         Ami = serverRandomPet.Keepers.Apply(keepers => keepers?.AmiId),
//!     });
//! 
//!     // ... (other aws_instance arguments) ...
//! });
//! ```
//! ### Go
//! ```go
//! package main
//! 
//! import (
//! 	"fmt"
//! 
//! 	"github.com/pulumi/pulumi-aws/sdk/v5/go/aws/ec2"
//! 	"github.com/pulumi/pulumi-random/sdk/v4/go/random"
//! 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
//! )
//! 
//! func main() {
//! 	pulumi.Run(func(ctx *pulumi.Context) error {
//! 		serverRandomPet, err := random.NewRandomPet(ctx, "serverRandomPet", &random.RandomPetArgs{
//! 			Keepers: pulumi.StringMap{
//! 				"ami_id": pulumi.Any(_var.Ami_id),
//! 			},
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		_, err = ec2.NewInstance(ctx, "serverInstance", &ec2.InstanceArgs{
//! 			Tags: pulumi.StringMap{
//! 				"Name": serverRandomPet.ID().ApplyT(func(id string) (string, error) {
//! 					return fmt.Sprintf("web-server-%v", id), nil
//! 				}).(pulumi.StringOutput),
//! 			},
//! 			Ami: serverRandomPet.Keepers.ApplyT(func(keepers interface{}) (*string, error) {
//! 				return &keepers.AmiId, nil
//! 			}).(pulumi.StringPtrOutput),
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
//! import com.pulumi.random.RandomPet;
//! import com.pulumi.random.RandomPetArgs;
//! import com.pulumi.aws.ec2.Instance;
//! import com.pulumi.aws.ec2.InstanceArgs;
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
//!         var serverRandomPet = new RandomPet("serverRandomPet", RandomPetArgs.builder()        
//!             .keepers(Map.of("ami_id", var_.ami_id()))
//!             .build());
//! 
//!         var serverInstance = new Instance("serverInstance", InstanceArgs.builder()        
//!             .tags(Map.of("Name", serverRandomPet.id().applyValue(id -> String.format("web-server-%s", id))))
//!             .ami(serverRandomPet.keepers().applyValue(keepers -> keepers.amiId()))
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
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

pub struct RandomPetArgs {
    /// Arbitrary map of values that, when changed, will trigger recreation of resource. See the main provider documentation for more information.
    pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    /// The length (in words) of the pet name. Defaults to 2
    pub length: pulumi_wasm_rust::Output<Option<i32>>,
    /// A string to prefix the name with.
    pub prefix: pulumi_wasm_rust::Output<Option<String>>,
    /// The character to separate words in the pet name. Defaults to "-"
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
pub fn create(name: &str, args: RandomPetArgs) -> RandomPetResult {

    let result = crate::bindings::pulumi::random::random_pet::invoke(name, &crate::bindings::pulumi::random::random_pet::Args {
        keepers: args.keepers.get_inner(),
        length: args.length.get_inner(),
        prefix: args.prefix.get_inner(),
        separator: args.separator.get_inner(),
    });

    RandomPetResult {
        keepers: crate::into_domain(result.keepers),
        length: crate::into_domain(result.length),
        prefix: crate::into_domain(result.prefix),
        separator: crate::into_domain(result.separator),
    }
}
