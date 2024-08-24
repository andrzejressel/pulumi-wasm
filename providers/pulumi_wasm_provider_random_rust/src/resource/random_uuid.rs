//! The resource `random.RandomUuid` generates a random uuid string that is intended to be used as a unique identifier for other resources.
//! 
//! This resource uses [hashicorp/go-uuid](https://github.com/hashicorp/go-uuid) to generate a UUID-formatted string for use with services needing a unique string identifier.
//! 
//! ## Example Usage
//! 
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as azure from "@pulumi/azure";
//! import * as random from "@pulumi/random";
//! 
//! const testRandomUuid = new random.RandomUuid("testRandomUuid", {});
//! const testResourceGroup = new azure.core.ResourceGroup("testResourceGroup", {location: "Central US"});
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_azure as azure
//! import pulumi_random as random
//! 
//! test_random_uuid = random.RandomUuid("testRandomUuid")
//! test_resource_group = azure.core.ResourceGroup("testResourceGroup", location="Central US")
//! ```
//! ### C#
//! ```csharp
//! using System.Collections.Generic;
//! using System.Linq;
//! using Pulumi;
//! using Azure = Pulumi.Azure;
//! using Random = Pulumi.Random;
//! 
//! return await Deployment.RunAsync(() => 
//! {
//!     var testRandomUuid = new Random.RandomUuid("testRandomUuid");
//! 
//!     var testResourceGroup = new Azure.Core.ResourceGroup("testResourceGroup", new()
//!     {
//!         Location = "Central US",
//!     });
//! 
//! });
//! ```
//! ### Go
//! ```go
//! package main
//! 
//! import (
//! 	"github.com/pulumi/pulumi-azure/sdk/v5/go/azure/core"
//! 	"github.com/pulumi/pulumi-random/sdk/v4/go/random"
//! 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
//! )
//! 
//! func main() {
//! 	pulumi.Run(func(ctx *pulumi.Context) error {
//! 		_, err := random.NewRandomUuid(ctx, "testRandomUuid", nil)
//! 		if err != nil {
//! 			return err
//! 		}
//! 		_, err = core.NewResourceGroup(ctx, "testResourceGroup", &core.ResourceGroupArgs{
//! 			Location: pulumi.String("Central US"),
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
//! import com.pulumi.random.RandomUuid;
//! import com.pulumi.azure.core.ResourceGroup;
//! import com.pulumi.azure.core.ResourceGroupArgs;
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
//!         var testRandomUuid = new RandomUuid("testRandomUuid");
//! 
//!         var testResourceGroup = new ResourceGroup("testResourceGroup", ResourceGroupArgs.builder()        
//!             .location("Central US")
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
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

pub struct RandomUuidArgs {
    /// Arbitrary map of values that, when changed, will trigger recreation of resource. See the main provider documentation for more information.
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
        keepers: args.keepers.get_inner(),
    });

    RandomUuidResult {
        keepers: crate::into_domain(result.keepers),
        result: crate::into_domain(result.result),
    }
}
