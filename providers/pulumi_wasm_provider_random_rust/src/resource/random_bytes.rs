//! The resource `random.RandomBytes` generates random bytes that are intended to be used as a secret, or key. Use this in preference to `random.RandomId` when the output is considered sensitive, and should not be displayed in the CLI.
//! 
//! ## Example Usage
//! 
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as azure from "@pulumi/azure";
//! import * as random from "@pulumi/random";
//! 
//! const jwtSecretRandomBytes = new random.RandomBytes("jwtSecretRandomBytes", {length: 64});
//! const jwtSecretSecret = new azure.keyvault.Secret("jwtSecretSecret", {
//!     keyVaultId: "some-azure-key-vault-id",
//!     value: jwtSecretRandomBytes.base64,
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_azure as azure
//! import pulumi_random as random
//! 
//! jwt_secret_random_bytes = random.RandomBytes("jwtSecretRandomBytes", length=64)
//! jwt_secret_secret = azure.keyvault.Secret("jwtSecretSecret",
//!     key_vault_id="some-azure-key-vault-id",
//!     value=jwt_secret_random_bytes.base64)
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
//!     var jwtSecretRandomBytes = new Random.RandomBytes("jwtSecretRandomBytes", new()
//!     {
//!         Length = 64,
//!     });
//! 
//!     var jwtSecretSecret = new Azure.KeyVault.Secret("jwtSecretSecret", new()
//!     {
//!         KeyVaultId = "some-azure-key-vault-id",
//!         Value = jwtSecretRandomBytes.Base64,
//!     });
//! 
//! });
//! ```
//! ### Go
//! ```go
//! package main
//! 
//! import (
//! 	"github.com/pulumi/pulumi-azure/sdk/v5/go/azure/keyvault"
//! 	"github.com/pulumi/pulumi-random/sdk/v4/go/random"
//! 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
//! )
//! 
//! func main() {
//! 	pulumi.Run(func(ctx *pulumi.Context) error {
//! 		jwtSecretRandomBytes, err := random.NewRandomBytes(ctx, "jwtSecretRandomBytes", &random.RandomBytesArgs{
//! 			Length: pulumi.Int(64),
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		_, err = keyvault.NewSecret(ctx, "jwtSecretSecret", &keyvault.SecretArgs{
//! 			KeyVaultId: pulumi.String("some-azure-key-vault-id"),
//! 			Value:      jwtSecretRandomBytes.Base64,
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
//! import com.pulumi.random.RandomBytes;
//! import com.pulumi.random.RandomBytesArgs;
//! import com.pulumi.azure.keyvault.Secret;
//! import com.pulumi.azure.keyvault.SecretArgs;
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
//!         var jwtSecretRandomBytes = new RandomBytes("jwtSecretRandomBytes", RandomBytesArgs.builder()        
//!             .length(64)
//!             .build());
//! 
//!         var jwtSecretSecret = new Secret("jwtSecretSecret", SecretArgs.builder()        
//!             .keyVaultId("some-azure-key-vault-id")
//!             .value(jwtSecretRandomBytes.base64())
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   jwtSecretRandomBytes:
//!     type: random:RandomBytes
//!     properties:
//!       length: 64
//!   jwtSecretSecret:
//!     type: azure:keyvault:Secret
//!     properties:
//!       keyVaultId: some-azure-key-vault-id
//!       value: ${jwtSecretRandomBytes.base64}
//! ```
//! 
//! ## Import
//! 
//! Random bytes can be imported by specifying the value as base64 string.
//! 
//! ```sh
//!  $ pulumi import random:index/randomBytes:RandomBytes basic "8/fu3q+2DcgSJ19i0jZ5Cw=="
//! ```
//! 
//!  

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct RandomBytesArgs {
    /// Arbitrary map of values that, when changed, will trigger recreation of resource. See the main provider documentation for more information.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    /// The number of bytes requested. The minimum value for length is 1.
    #[builder(into)]
    pub length: pulumi_wasm_rust::Output<i32>,
}

pub struct RandomBytesResult {
    /// The generated bytes presented in base64 string format.
    pub base64: pulumi_wasm_rust::Output<String>,
    /// The generated bytes presented in hex string format.
    pub hex: pulumi_wasm_rust::Output<String>,
    /// Arbitrary map of values that, when changed, will trigger recreation of resource. See the main provider documentation for more information.
    pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    /// The number of bytes requested. The minimum value for length is 1.
    pub length: pulumi_wasm_rust::Output<i32>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: RandomBytesArgs) -> RandomBytesResult {

    let result = crate::bindings::pulumi::random::random_bytes::invoke(name, &crate::bindings::pulumi::random::random_bytes::Args {
        keepers: &args.keepers.get_inner(),
        length: &args.length.get_inner(),
    });

    RandomBytesResult {
        base64: crate::into_domain(result.base64),
        hex: crate::into_domain(result.hex),
        keepers: crate::into_domain(result.keepers),
        length: crate::into_domain(result.length),
    }
}
