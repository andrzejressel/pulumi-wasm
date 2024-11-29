//! Provides a Cloudflare Access Mutual TLS Certificate resource.
//! Mutual TLS authentication ensures that the traffic is secure and
//! trusted in both directions between a client and server and can be
//!  used with Access to only allows requests from devices with a
//!  corresponding client certificate.
//! 
//! > It's required that an `account_id` or `zone_id` is provided and in
//!    most cases using either is fine. However, if you're using a scoped
//!    access token, you must provide the argument that matches the token's
//!    scope. For example, an access token that is scoped to the "example.com"
//!    zone needs to use the `zone_id` argument.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const myCert = new cloudflare.ZeroTrustAccessMtlsCertificate("my_cert", {
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//!     name: "My Root Cert",
//!     certificate: caPem,
//!     associatedHostnames: ["staging.example.com"],
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! my_cert = cloudflare.ZeroTrustAccessMtlsCertificate("my_cert",
//!     zone_id="0da42c8d2132a9ddaf714f9e7c920711",
//!     name="My Root Cert",
//!     certificate=ca_pem,
//!     associated_hostnames=["staging.example.com"])
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
//!     var myCert = new Cloudflare.ZeroTrustAccessMtlsCertificate("my_cert", new()
//!     {
//!         ZoneId = "0da42c8d2132a9ddaf714f9e7c920711",
//!         Name = "My Root Cert",
//!         Certificate = caPem,
//!         AssociatedHostnames = new[]
//!         {
//!             "staging.example.com",
//!         },
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
//! 		_, err := cloudflare.NewZeroTrustAccessMtlsCertificate(ctx, "my_cert", &cloudflare.ZeroTrustAccessMtlsCertificateArgs{
//! 			ZoneId:      pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
//! 			Name:        pulumi.String("My Root Cert"),
//! 			Certificate: pulumi.Any(caPem),
//! 			AssociatedHostnames: pulumi.StringArray{
//! 				pulumi.String("staging.example.com"),
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
//! import com.pulumi.cloudflare.ZeroTrustAccessMtlsCertificate;
//! import com.pulumi.cloudflare.ZeroTrustAccessMtlsCertificateArgs;
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
//!         var myCert = new ZeroTrustAccessMtlsCertificate("myCert", ZeroTrustAccessMtlsCertificateArgs.builder()
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .name("My Root Cert")
//!             .certificate(caPem)
//!             .associatedHostnames("staging.example.com")
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   myCert:
//!     type: cloudflare:ZeroTrustAccessMtlsCertificate
//!     name: my_cert
//!     properties:
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!       name: My Root Cert
//!       certificate: ${caPem}
//!       associatedHostnames:
//!         - staging.example.com
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! Account level import.
//! 
//! ```sh
//! $ pulumi import cloudflare:index/zeroTrustAccessMtlsCertificate:ZeroTrustAccessMtlsCertificate cloudflare_zero_sd -t_access_mtls_certificate.example account/<account_id>/<mutual_tls_certificate_id>
//! ```
//! 
//! Zone level import.
//! 
//! ```sh
//! $ pulumi import cloudflare:index/zeroTrustAccessMtlsCertificate:ZeroTrustAccessMtlsCertificate cloudflare_zero_sd -t_access_mtls_certificate.example zone/<zone_id>/<mutual_tls_certificate_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustAccessMtlsCertificateArgs {
    /// The account identifier to target for the resource. Conflicts with `zone_id`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// The hostnames that will be prompted for this certificate.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub associated_hostnames: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// The Root CA for your certificates.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub certificate: pulumi_wasm_rust::Output<Option<String>>,
    /// The name of the certificate.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct ZeroTrustAccessMtlsCertificateResult {
    /// The account identifier to target for the resource. Conflicts with `zone_id`.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The hostnames that will be prompted for this certificate.
    pub associated_hostnames: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// The Root CA for your certificates.
    pub certificate: pulumi_wasm_rust::Output<Option<String>>,
    pub fingerprint: pulumi_wasm_rust::Output<String>,
    /// The name of the certificate.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ZeroTrustAccessMtlsCertificateArgs) -> ZeroTrustAccessMtlsCertificateResult {

    let result = crate::bindings::pulumi::cloudflare::zero_trust_access_mtls_certificate::invoke(name, &crate::bindings::pulumi::cloudflare::zero_trust_access_mtls_certificate::Args {
        account_id: &args.account_id.get_inner(),
        associated_hostnames: &args.associated_hostnames.get_inner(),
        certificate: &args.certificate.get_inner(),
        name: &args.name.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    ZeroTrustAccessMtlsCertificateResult {
        account_id: crate::into_domain(result.account_id),
        associated_hostnames: crate::into_domain(result.associated_hostnames),
        certificate: crate::into_domain(result.certificate),
        fingerprint: crate::into_domain(result.fingerprint),
        name: crate::into_domain(result.name),
        zone_id: crate::into_domain(result.zone_id),
    }
}
