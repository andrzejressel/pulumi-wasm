//! Use this data source to retrieve an existing origin ca certificate.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const example = cloudflare.getOriginCaCertificate({
//!     id: "REPLACE_ME",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.get_origin_ca_certificate(id="REPLACE_ME")
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
//!     var example = Cloudflare.GetOriginCaCertificate.Invoke(new()
//!     {
//!         Id = "REPLACE_ME",
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
//! 		_, err := cloudflare.LookupOriginCaCertificate(ctx, &cloudflare.LookupOriginCaCertificateArgs{
//! 			Id: "REPLACE_ME",
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
//! import com.pulumi.cloudflare.inputs.GetOriginCaCertificateArgs;
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
//!         final var example = CloudflareFunctions.getOriginCaCertificate(GetOriginCaCertificateArgs.builder()
//!             .id("REPLACE_ME")
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
//!       Function: cloudflare:getOriginCaCertificate
//!       Arguments:
//!         id: REPLACE_ME
//! ```
//! <!--End PulumiCodeChooser -->

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct GetOriginCaCertificateArgs {
    /// The Origin CA Certificate unique identifier.
    #[builder(into)]
    pub id: pulumi_wasm_rust::Output<String>,
}

pub struct GetOriginCaCertificateResult {
    /// The Origin CA certificate.
    pub certificate: pulumi_wasm_rust::Output<String>,
    /// The timestamp when the certificate will expire.
    pub expires_on: pulumi_wasm_rust::Output<String>,
    /// A list of hostnames or wildcard names bound to the certificate.
    pub hostnames: pulumi_wasm_rust::Output<Vec<String>>,
    /// The Origin CA Certificate unique identifier.
    pub id: pulumi_wasm_rust::Output<String>,
    /// The signature type desired on the certificate. Available values: `origin-rsa`, `origin-ecc`, `keyless-certificate`
    pub request_type: pulumi_wasm_rust::Output<String>,
    /// The timestamp when the certificate was revoked.
    pub revoked_at: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: GetOriginCaCertificateArgs
) -> GetOriginCaCertificateResult {

    let result = crate::bindings::pulumi::cloudflare::get_origin_ca_certificate::invoke(
        &crate::bindings::pulumi::cloudflare::get_origin_ca_certificate::Args {
                id: &args.id.get_inner(),
        }
    );

    GetOriginCaCertificateResult {
        certificate: crate::into_domain(result.certificate),
        expires_on: crate::into_domain(result.expires_on),
        hostnames: crate::into_domain(result.hostnames),
        id: crate::into_domain(result.id),
        request_type: crate::into_domain(result.request_type),
        revoked_at: crate::into_domain(result.revoked_at),
    }
}