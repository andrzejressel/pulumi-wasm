//! The resource `random.RandomId` generates random numbers that are intended to be
//! used as unique identifiers for other resources. If the output is considered
//! sensitive, and should not be displayed in the CLI, use `random.RandomBytes`
//! instead.
//!
//! This resource *does* use a cryptographic random number generator in order
//! to minimize the chance of collisions, making the results of this resource
//! when a 16-byte identifier is requested of equivalent uniqueness to a
//! type-4 UUID.
//!
//! This resource can be used in conjunction with resources that have
//! the `create_before_destroy` lifecycle flag set to avoid conflicts with
//! unique names during the brief period where both the old and new resources
//! exist concurrently.
//!
//! ## Example Usage
//!
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as aws from "@pulumi/aws";
//! import * as random from "@pulumi/random";
//!
//! // The following example shows how to generate a unique name for an AWS EC2
//! // instance that changes each time a new AMI id is selected.
//! const serverRandomId = new random.RandomId("serverRandomId", {
//!     keepers: {
//!         ami_id: _var.ami_id,
//!     },
//!     byteLength: 8,
//! });
//! const serverInstance = new aws.ec2.Instance("serverInstance", {
//!     tags: {
//!         Name: pulumi.interpolate`web-server ${serverRandomId.hex}`,
//!     },
//!     ami: serverRandomId.keepers.apply(keepers => keepers?.amiId),
//! });
//! // ... (other aws_instance arguments) ...
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_aws as aws
//! import pulumi_random as random
//!
//! # The following example shows how to generate a unique name for an AWS EC2
//! # instance that changes each time a new AMI id is selected.
//! server_random_id = random.RandomId("serverRandomId",
//!     keepers={
//!         "ami_id": var["ami_id"],
//!     },
//!     byte_length=8)
//! server_instance = aws.ec2.Instance("serverInstance",
//!     tags={
//!         "Name": server_random_id.hex.apply(lambda hex: f"web-server {hex}"),
//!     },
//!     ami=server_random_id.keepers["amiId"])
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
//!     // The following example shows how to generate a unique name for an AWS EC2
//!     // instance that changes each time a new AMI id is selected.
//!     var serverRandomId = new Random.RandomId("serverRandomId", new()
//!     {
//!         Keepers =
//!         {
//!             { "ami_id", @var.Ami_id },
//!         },
//!         ByteLength = 8,
//!     });
//!
//!     var serverInstance = new Aws.Ec2.Instance("serverInstance", new()
//!     {
//!         Tags =
//!         {
//!             { "Name", serverRandomId.Hex.Apply(hex => $"web-server {hex}") },
//!         },
//!         Ami = serverRandomId.Keepers.Apply(keepers => keepers?.AmiId),
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
//! 		serverRandomId, err := random.NewRandomId(ctx, "serverRandomId", &random.RandomIdArgs{
//! 			Keepers: pulumi.StringMap{
//! 				"ami_id": pulumi.Any(_var.Ami_id),
//! 			},
//! 			ByteLength: pulumi.Int(8),
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		_, err = ec2.NewInstance(ctx, "serverInstance", &ec2.InstanceArgs{
//! 			Tags: pulumi.StringMap{
//! 				"Name": serverRandomId.Hex.ApplyT(func(hex string) (string, error) {
//! 					return fmt.Sprintf("web-server %v", hex), nil
//! 				}).(pulumi.StringOutput),
//! 			},
//! 			Ami: serverRandomId.Keepers.ApplyT(func(keepers interface{}) (*string, error) {
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
//! import com.pulumi.random.RandomId;
//! import com.pulumi.random.RandomIdArgs;
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
//!         var serverRandomId = new RandomId("serverRandomId", RandomIdArgs.builder()        
//!             .keepers(Map.of("ami_id", var_.ami_id()))
//!             .byteLength(8)
//!             .build());
//!
//!         var serverInstance = new Instance("serverInstance", InstanceArgs.builder()        
//!             .tags(Map.of("Name", serverRandomId.hex().applyValue(hex -> String.format("web-server %s", hex))))
//!             .ami(serverRandomId.keepers().applyValue(keepers -> keepers.amiId()))
//!             .build());
//!
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   # The following example shows how to generate a unique name for an AWS EC2
//!   # instance that changes each time a new AMI id is selected.
//!   serverRandomId:
//!     type: random:RandomId
//!     properties:
//!       keepers:
//!         ami_id: ${var.ami_id}
//!       byteLength: 8
//!   serverInstance:
//!     type: aws:ec2:Instance
//!     properties:
//!       tags:
//!         Name: web-server ${serverRandomId.hex}
//!       # Read the AMI id "through" the random_id resource to ensure that
//!       #   # both will change together.
//!       ami: ${serverRandomId.keepers.amiId}
//! ```
//!
//! ## Import
//!
//! Random IDs can be imported using the b64_url with an optional prefix. This can be used to replace a config value with a value interpolated from the random provider without experiencing diffs. Example with no prefix
//!
//! ```sh
//!  $ pulumi import random:index/randomId:RandomId server p-9hUg
//! ```
//!
//!  Example with prefix (prefix is separated by a ,)
//!
//! ```sh
//!  $ pulumi import random:index/randomId:RandomId server my-prefix-,p-9hUg
//! ```
//!
//!  

pub struct RandomIdArgs {
    /// The number of random bytes to produce. The minimum value is 1, which produces eight bits of randomness.
    pub byte_length: pulumi_wasm_rust::Output<i32>,
    /// Arbitrary map of values that, when changed, will trigger recreation of resource. See the main provider documentation for more information.
    pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    /// Arbitrary string to prefix the output value with. This string is supplied as-is, meaning it is not guaranteed to be URL-safe or base64 encoded.
    pub prefix: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct RandomIdResult {
    /// The generated id presented in base64 without additional transformations.
    pub b64_std: pulumi_wasm_rust::Output<String>,
    /// The generated id presented in base64, using the URL-friendly character set: case-sensitive letters, digits and the characters `_` and `-`.
    pub b64_url: pulumi_wasm_rust::Output<String>,
    /// The number of random bytes to produce. The minimum value is 1, which produces eight bits of randomness.
    pub byte_length: pulumi_wasm_rust::Output<i32>,
    /// The generated id presented in non-padded decimal digits.
    pub dec: pulumi_wasm_rust::Output<String>,
    /// The generated id presented in padded hexadecimal digits. This result will always be twice as long as the requested byte length.
    pub hex: pulumi_wasm_rust::Output<String>,
    /// Arbitrary map of values that, when changed, will trigger recreation of resource. See the main provider documentation for more information.
    pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    /// Arbitrary string to prefix the output value with. This string is supplied as-is, meaning it is not guaranteed to be URL-safe or base64 encoded.
    pub prefix: pulumi_wasm_rust::Output<Option<String>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: RandomIdArgs) -> RandomIdResult {
    let result = crate::bindings::pulumi::random::random_id::invoke(
        name,
        &crate::bindings::pulumi::random::random_id::Args {
            byte_length: &args.byte_length.get_inner(),
            keepers: &args.keepers.get_inner(),
            prefix: &args.prefix.get_inner(),
        },
    );

    RandomIdResult {
        b64_std: crate::into_domain(result.b64_std),
        b64_url: crate::into_domain(result.b64_url),
        byte_length: crate::into_domain(result.byte_length),
        dec: crate::into_domain(result.dec),
        hex: crate::into_domain(result.hex),
        keepers: crate::into_domain(result.keepers),
        prefix: crate::into_domain(result.prefix),
    }
}
