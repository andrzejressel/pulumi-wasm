//! Use this data source to get the
//! [Origin CA root certificate](https://developers.cloudflare.com/ssl/origin-configuration/origin-ca#4-required-for-some-add-cloudflare-origin-ca-root-certificates)
//! for a given algorithm."
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const example = cloudflare.getOriginCaRootCertificate({
//!     algorithm: "rsa",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.get_origin_ca_root_certificate(algorithm="rsa")
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
//!     var example = Cloudflare.GetOriginCaRootCertificate.Invoke(new()
//!     {
//!         Algorithm = "rsa",
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
//! 		_, err := cloudflare.GetOriginCaRootCertificate(ctx, &cloudflare.GetOriginCaRootCertificateArgs{
//! 			Algorithm: "rsa",
//! 		}, nil)
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
//! import com.pulumi.cloudflare.CloudflareFunctions;
//! import com.pulumi.cloudflare.inputs.GetOriginCaRootCertificateArgs;
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
//!         final var example = CloudflareFunctions.getOriginCaRootCertificate(GetOriginCaRootCertificateArgs.builder()
//!             .algorithm("rsa")
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! variables:
//!   example:
//!     fn::invoke:
//!       Function: cloudflare:getOriginCaRootCertificate
//!       Arguments:
//!         algorithm: rsa
//! ```
//! <!--End PulumiCodeChooser -->

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct GetOriginCaRootCertificateArgs {
    /// The name of the algorithm used when creating an Origin CA certificate. Available values: `rsa`, `ecc`.
    #[builder(into)]
    pub algorithm: pulumi_wasm_rust::Output<String>,
}

pub struct GetOriginCaRootCertificateResult {
    /// The name of the algorithm used when creating an Origin CA certificate. Available values: `rsa`, `ecc`.
    pub algorithm: pulumi_wasm_rust::Output<String>,
    /// The Origin CA root certificate in PEM format.
    pub cert_pem: pulumi_wasm_rust::Output<String>,
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(args: GetOriginCaRootCertificateArgs) -> GetOriginCaRootCertificateResult {

    let result = crate::bindings::pulumi::cloudflare::get_origin_ca_root_certificate::invoke(&crate::bindings::pulumi::cloudflare::get_origin_ca_root_certificate::Args {
        algorithm: &args.algorithm.get_inner(),
    });

    GetOriginCaRootCertificateResult {
        algorithm: crate::into_domain(result.algorithm),
        cert_pem: crate::into_domain(result.cert_pem),
        id: crate::into_domain(result.id),
    }
}
