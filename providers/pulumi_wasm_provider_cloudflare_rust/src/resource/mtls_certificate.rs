//! Provides a Cloudflare mTLS certificate resource. These certificates may be used with mTLS enabled Cloudflare services.
//!
//! ## Example Usage
//!
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//!
//! const example = new cloudflare.MtlsCertificate("example", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     ca: true,
//!     certificates: `-----BEGIN CERTIFICATE-----
//! MIIDmDCCAoCgAwIBAgIUKTOAZNj...i4JhqeoTewsxndhDDE
//! -----END CERTIFICATE-----
//! `,
//!     name: "example",
//!     privateKey: `-----BEGIN PRIVATE KEY-----
//! MIIEvQIBADANBgkqhkiG9w0BAQE...1IS3EnQRrz6WMYA=
//! -----END PRIVATE KEY-----
//! `,
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//!
//! example = cloudflare.MtlsCertificate("example",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     ca=True,
//!     certificates="""-----BEGIN CERTIFICATE-----
//! MIIDmDCCAoCgAwIBAgIUKTOAZNj...i4JhqeoTewsxndhDDE
//! -----END CERTIFICATE-----
//! """,
//!     name="example",
//!     private_key="""-----BEGIN PRIVATE KEY-----
//! MIIEvQIBADANBgkqhkiG9w0BAQE...1IS3EnQRrz6WMYA=
//! -----END PRIVATE KEY-----
//! """)
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
//!     var example = new Cloudflare.MtlsCertificate("example", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Ca = true,
//!         Certificates = @"-----BEGIN CERTIFICATE-----
//! MIIDmDCCAoCgAwIBAgIUKTOAZNj...i4JhqeoTewsxndhDDE
//! -----END CERTIFICATE-----
//! ",
//!         Name = "example",
//!         PrivateKey = @"-----BEGIN PRIVATE KEY-----
//! MIIEvQIBADANBgkqhkiG9w0BAQE...1IS3EnQRrz6WMYA=
//! -----END PRIVATE KEY-----
//! ",
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
//! 		_, err := cloudflare.NewMtlsCertificate(ctx, "example", &cloudflare.MtlsCertificateArgs{
//! 			AccountId:    pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Ca:           pulumi.Bool(true),
//! 			Certificates: pulumi.String("-----BEGIN CERTIFICATE-----\nMIIDmDCCAoCgAwIBAgIUKTOAZNj...i4JhqeoTewsxndhDDE\n-----END CERTIFICATE-----\n"),
//! 			Name:         pulumi.String("example"),
//! 			PrivateKey:   pulumi.String("-----BEGIN PRIVATE KEY-----\nMIIEvQIBADANBgkqhkiG9w0BAQE...1IS3EnQRrz6WMYA=\n-----END PRIVATE KEY-----\n"),
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
//! import com.pulumi.cloudflare.MtlsCertificate;
//! import com.pulumi.cloudflare.MtlsCertificateArgs;
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
//!         var example = new MtlsCertificate("example", MtlsCertificateArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .ca(true)
//!             .certificates("""
//! -----BEGIN CERTIFICATE-----
//! MIIDmDCCAoCgAwIBAgIUKTOAZNj...i4JhqeoTewsxndhDDE
//! -----END CERTIFICATE-----
//!             """)
//!             .name("example")
//!             .privateKey("""
//! -----BEGIN PRIVATE KEY-----
//! MIIEvQIBADANBgkqhkiG9w0BAQE...1IS3EnQRrz6WMYA=
//! -----END PRIVATE KEY-----
//!             """)
//!             .build());
//!
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:MtlsCertificate
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       ca: true
//!       certificates: |
//!         -----BEGIN CERTIFICATE-----
//!         MIIDmDCCAoCgAwIBAgIUKTOAZNj...i4JhqeoTewsxndhDDE
//!         -----END CERTIFICATE-----
//!       name: example
//!       privateKey: |
//!         -----BEGIN PRIVATE KEY-----
//!         MIIEvQIBADANBgkqhkiG9w0BAQE...1IS3EnQRrz6WMYA=
//!         -----END PRIVATE KEY-----
//! ```
//! <!--End PulumiCodeChooser -->
//!
//! ## Import
//!
//! ```sh
//! $ pulumi import cloudflare:index/mtlsCertificate:MtlsCertificate example <account_id>/<mtls_certificate_id>
//! ```
//!

pub struct MtlsCertificateArgs {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Whether this is a CA or leaf certificate. **Modifying this attribute will force creation of a new resource.**
    pub ca: pulumi_wasm_rust::Output<bool>,
    /// Certificate you intend to use with mTLS-enabled services. **Modifying this attribute will force creation of a new resource.**
    pub certificates: pulumi_wasm_rust::Output<String>,
    /// Optional unique name for the certificate. **Modifying this attribute will force creation of a new resource.**
    pub name: pulumi_wasm_rust::Output<Option<String>>,
    /// The certificate's private key. **Modifying this attribute will force creation of a new resource.**
    pub private_key: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct MtlsCertificateResult {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Whether this is a CA or leaf certificate. **Modifying this attribute will force creation of a new resource.**
    pub ca: pulumi_wasm_rust::Output<bool>,
    /// Certificate you intend to use with mTLS-enabled services. **Modifying this attribute will force creation of a new resource.**
    pub certificates: pulumi_wasm_rust::Output<String>,
    /// **Modifying this attribute will force creation of a new resource.**
    pub expires_on: pulumi_wasm_rust::Output<String>,
    /// **Modifying this attribute will force creation of a new resource.**
    pub issuer: pulumi_wasm_rust::Output<String>,
    /// Optional unique name for the certificate. **Modifying this attribute will force creation of a new resource.**
    pub name: pulumi_wasm_rust::Output<Option<String>>,
    /// The certificate's private key. **Modifying this attribute will force creation of a new resource.**
    pub private_key: pulumi_wasm_rust::Output<Option<String>>,
    /// **Modifying this attribute will force creation of a new resource.**
    pub serial_number: pulumi_wasm_rust::Output<String>,
    /// **Modifying this attribute will force creation of a new resource.**
    pub signature: pulumi_wasm_rust::Output<String>,
    /// **Modifying this attribute will force creation of a new resource.**
    pub uploaded_on: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: MtlsCertificateArgs) -> MtlsCertificateResult {
    let result = crate::bindings::pulumi::cloudflare::mtls_certificate::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::mtls_certificate::Args {
            account_id: &args.account_id.get_inner(),
            ca: &args.ca.get_inner(),
            certificates: &args.certificates.get_inner(),
            name: &args.name.get_inner(),
            private_key: &args.private_key.get_inner(),
        },
    );

    MtlsCertificateResult {
        account_id: crate::into_domain(result.account_id),
        ca: crate::into_domain(result.ca),
        certificates: crate::into_domain(result.certificates),
        expires_on: crate::into_domain(result.expires_on),
        issuer: crate::into_domain(result.issuer),
        name: crate::into_domain(result.name),
        private_key: crate::into_domain(result.private_key),
        serial_number: crate::into_domain(result.serial_number),
        signature: crate::into_domain(result.signature),
        uploaded_on: crate::into_domain(result.uploaded_on),
    }
}
