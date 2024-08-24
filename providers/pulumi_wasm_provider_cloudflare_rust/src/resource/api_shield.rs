//! Provides a resource to manage API Shield configurations.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const example = new cloudflare.ApiShield("example", {
//!     authIdCharacteristics: [{
//!         name: "my-example-header",
//!         type: "header",
//!     }],
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.ApiShield("example",
//!     auth_id_characteristics=[cloudflare.ApiShieldAuthIdCharacteristicArgs(
//!         name="my-example-header",
//!         type="header",
//!     )],
//!     zone_id="0da42c8d2132a9ddaf714f9e7c920711")
//! ```
//! ### C#
//! ```csharp
//! using System.Collections.Generic;
//! using System.Linq;
//! using Pulumi;
//! using Cloudflare = Pulumi.Cloudflare;
//! 
//! return await Deployment.RunAsync(() => 
//! {
//!     var example = new Cloudflare.ApiShield("example", new()
//!     {
//!         AuthIdCharacteristics = new[]
//!         {
//!             new Cloudflare.Inputs.ApiShieldAuthIdCharacteristicArgs
//!             {
//!                 Name = "my-example-header",
//!                 Type = "header",
//!             },
//!         },
//!         ZoneId = "0da42c8d2132a9ddaf714f9e7c920711",
//!     });
//! 
//! });
//! ```
//! ### Go
//! ```go
//! package main
//! 
//! import (
//! 	"github.com/pulumi/pulumi-cloudflare/sdk/v5/go/cloudflare"
//! 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
//! )
//! 
//! func main() {
//! 	pulumi.Run(func(ctx *pulumi.Context) error {
//! 		_, err := cloudflare.NewApiShield(ctx, "example", &cloudflare.ApiShieldArgs{
//! 			AuthIdCharacteristics: cloudflare.ApiShieldAuthIdCharacteristicArray{
//! 				&cloudflare.ApiShieldAuthIdCharacteristicArgs{
//! 					Name: pulumi.String("my-example-header"),
//! 					Type: pulumi.String("header"),
//! 				},
//! 			},
//! 			ZoneId: pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
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
//! import com.pulumi.cloudflare.ApiShield;
//! import com.pulumi.cloudflare.ApiShieldArgs;
//! import com.pulumi.cloudflare.inputs.ApiShieldAuthIdCharacteristicArgs;
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
//!         var example = new ApiShield("example", ApiShieldArgs.builder()        
//!             .authIdCharacteristics(ApiShieldAuthIdCharacteristicArgs.builder()
//!                 .name("my-example-header")
//!                 .type("header")
//!                 .build())
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:ApiShield
//!     properties:
//!       authIdCharacteristics:
//!         - name: my-example-header
//!           type: header
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//! ```
//! <!--End PulumiCodeChooser -->

pub struct ApiShieldArgs {
    /// Characteristics define properties across which auth-ids can be computed in a privacy-preserving manner.
    pub auth_id_characteristics: pulumi_wasm_rust::Output<Option<Vec<crate::types::ApiShieldAuthIdCharacteristic>>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct ApiShieldResult {
    /// Characteristics define properties across which auth-ids can be computed in a privacy-preserving manner.
    pub auth_id_characteristics: pulumi_wasm_rust::Output<Option<Vec<crate::types::ApiShieldAuthIdCharacteristic>>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ApiShieldArgs) -> ApiShieldResult {

    let result = crate::bindings::pulumi::cloudflare::api_shield::invoke(name, &crate::bindings::pulumi::cloudflare::api_shield::Args {
        auth_id_characteristics: args.auth_id_characteristics.get_inner(),
        zone_id: args.zone_id.get_inner(),
    });

    ApiShieldResult {
        auth_id_characteristics: crate::into_domain(result.auth_id_characteristics),
        zone_id: crate::into_domain(result.zone_id),
    }
}
