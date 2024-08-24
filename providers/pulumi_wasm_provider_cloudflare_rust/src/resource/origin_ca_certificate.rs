//! Provides a Cloudflare Origin CA certificate used to protect traffic to your origin without involving a third party Certificate Authority.
//!
//! > Since v3.32.0
//!    all authentication schemes are supported for managing Origin CA certificates.
//!    Versions prior to v3.32.0 will still need to use `api_user_service_key`.
//!
//! ## Example Usage
//!
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! import * as tls from "@pulumi/tls";
//!
//! const examplePrivateKey = new tls.PrivateKey("examplePrivateKey", {algorithm: "RSA"});
//! const exampleCertRequest = new tls.CertRequest("exampleCertRequest", {
//!     privateKeyPem: examplePrivateKey.privateKeyPem,
//!     subjects: [{
//!         commonName: "",
//!         organization: "Terraform Test",
//!     }],
//! });
//! const exampleOriginCaCertificate = new cloudflare.OriginCaCertificate("exampleOriginCaCertificate", {
//!     csr: exampleCertRequest.certRequestPem,
//!     hostnames: ["example.com"],
//!     requestType: "origin-rsa",
//!     requestedValidity: 7,
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! import pulumi_tls as tls
//!
//! example_private_key = tls.PrivateKey("examplePrivateKey", algorithm="RSA")
//! example_cert_request = tls.CertRequest("exampleCertRequest",
//!     private_key_pem=example_private_key.private_key_pem,
//!     subjects=[tls.CertRequestSubjectArgs(
//!         common_name="",
//!         organization="Terraform Test",
//!     )])
//! example_origin_ca_certificate = cloudflare.OriginCaCertificate("exampleOriginCaCertificate",
//!     csr=example_cert_request.cert_request_pem,
//!     hostnames=["example.com"],
//!     request_type="origin-rsa",
//!     requested_validity=7)
//! ```
//! ### C#
//! ```csharp
//! using System.Collections.Generic;
//! using System.Linq;
//! using Pulumi;
//! using Cloudflare = Pulumi.Cloudflare;
//! using Tls = Pulumi.Tls;
//!
//! return await Deployment.RunAsync(() =>
//! {
//!     var examplePrivateKey = new Tls.PrivateKey("examplePrivateKey", new()
//!     {
//!         Algorithm = "RSA",
//!     });
//!
//!     var exampleCertRequest = new Tls.CertRequest("exampleCertRequest", new()
//!     {
//!         PrivateKeyPem = examplePrivateKey.PrivateKeyPem,
//!         Subjects = new[]
//!         {
//!             new Tls.Inputs.CertRequestSubjectArgs
//!             {
//!                 CommonName = "",
//!                 Organization = "Terraform Test",
//!             },
//!         },
//!     });
//!
//!     var exampleOriginCaCertificate = new Cloudflare.OriginCaCertificate("exampleOriginCaCertificate", new()
//!     {
//!         Csr = exampleCertRequest.CertRequestPem,
//!         Hostnames = new[]
//!         {
//!             "example.com",
//!         },
//!         RequestType = "origin-rsa",
//!         RequestedValidity = 7,
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
//! 	"github.com/pulumi/pulumi-tls/sdk/v4/go/tls"
//! 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
//! )
//!
//! func main() {
//! 	pulumi.Run(func(ctx *pulumi.Context) error {
//! 		examplePrivateKey, err := tls.NewPrivateKey(ctx, "examplePrivateKey", &tls.PrivateKeyArgs{
//! 			Algorithm: pulumi.String("RSA"),
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		exampleCertRequest, err := tls.NewCertRequest(ctx, "exampleCertRequest", &tls.CertRequestArgs{
//! 			PrivateKeyPem: examplePrivateKey.PrivateKeyPem,
//! 			Subjects: tls.CertRequestSubjectArray{
//! 				&tls.CertRequestSubjectArgs{
//! 					CommonName:   pulumi.String(""),
//! 					Organization: pulumi.String("Terraform Test"),
//! 				},
//! 			},
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		_, err = cloudflare.NewOriginCaCertificate(ctx, "exampleOriginCaCertificate", &cloudflare.OriginCaCertificateArgs{
//! 			Csr: exampleCertRequest.CertRequestPem,
//! 			Hostnames: pulumi.StringArray{
//! 				pulumi.String("example.com"),
//! 			},
//! 			RequestType:       pulumi.String("origin-rsa"),
//! 			RequestedValidity: pulumi.Int(7),
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
//! import com.pulumi.tls.PrivateKey;
//! import com.pulumi.tls.PrivateKeyArgs;
//! import com.pulumi.tls.CertRequest;
//! import com.pulumi.tls.CertRequestArgs;
//! import com.pulumi.tls.inputs.CertRequestSubjectArgs;
//! import com.pulumi.cloudflare.OriginCaCertificate;
//! import com.pulumi.cloudflare.OriginCaCertificateArgs;
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
//!         var examplePrivateKey = new PrivateKey("examplePrivateKey", PrivateKeyArgs.builder()        
//!             .algorithm("RSA")
//!             .build());
//!
//!         var exampleCertRequest = new CertRequest("exampleCertRequest", CertRequestArgs.builder()        
//!             .privateKeyPem(examplePrivateKey.privateKeyPem())
//!             .subjects(CertRequestSubjectArgs.builder()
//!                 .commonName("")
//!                 .organization("Terraform Test")
//!                 .build())
//!             .build());
//!
//!         var exampleOriginCaCertificate = new OriginCaCertificate("exampleOriginCaCertificate", OriginCaCertificateArgs.builder()        
//!             .csr(exampleCertRequest.certRequestPem())
//!             .hostnames("example.com")
//!             .requestType("origin-rsa")
//!             .requestedValidity(7)
//!             .build());
//!
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   examplePrivateKey:
//!     type: tls:PrivateKey
//!     properties:
//!       algorithm: RSA
//!   exampleCertRequest:
//!     type: tls:CertRequest
//!     properties:
//!       privateKeyPem: ${examplePrivateKey.privateKeyPem}
//!       subjects:
//!         - commonName:
//!           organization: Terraform Test
//!   exampleOriginCaCertificate:
//!     type: cloudflare:OriginCaCertificate
//!     properties:
//!       csr: ${exampleCertRequest.certRequestPem}
//!       hostnames:
//!         - example.com
//!       requestType: origin-rsa
//!       requestedValidity: 7
//! ```
//! <!--End PulumiCodeChooser -->
//!
//! ## Import
//!
//! ```sh
//! $ pulumi import cloudflare:index/originCaCertificate:OriginCaCertificate example <certificate_id>
//! ```
//!

pub struct OriginCaCertificateArgs {
    /// The Certificate Signing Request. Must be newline-encoded. **Modifying this attribute will force creation of a new resource.**
    pub csr: pulumi_wasm_rust::Output<String>,
    /// A list of hostnames or wildcard names bound to the certificate. **Modifying this attribute will force creation of a new resource.**
    pub hostnames: pulumi_wasm_rust::Output<Vec<String>>,
    /// Number of days prior to the expiry to trigger a renewal of the certificate if a Terraform operation is run.
    pub min_days_for_renewal: pulumi_wasm_rust::Output<Option<i32>>,
    /// The signature type desired on the certificate. Available values: `origin-rsa`, `origin-ecc`, `keyless-certificate`. **Modifying this attribute will force creation of a new resource.**
    pub request_type: pulumi_wasm_rust::Output<String>,
    /// The number of days for which the certificate should be valid. Available values: `7`, `30`, `90`, `365`, `730`, `1095`, `5475`. **Modifying this attribute will force creation of a new resource.**
    pub requested_validity: pulumi_wasm_rust::Output<Option<i32>>,
}

pub struct OriginCaCertificateResult {
    /// The Origin CA certificate.
    pub certificate: pulumi_wasm_rust::Output<String>,
    /// The Certificate Signing Request. Must be newline-encoded. **Modifying this attribute will force creation of a new resource.**
    pub csr: pulumi_wasm_rust::Output<String>,
    /// The datetime when the certificate will expire.
    pub expires_on: pulumi_wasm_rust::Output<String>,
    /// A list of hostnames or wildcard names bound to the certificate. **Modifying this attribute will force creation of a new resource.**
    pub hostnames: pulumi_wasm_rust::Output<Vec<String>>,
    /// Number of days prior to the expiry to trigger a renewal of the certificate if a Terraform operation is run.
    pub min_days_for_renewal: pulumi_wasm_rust::Output<Option<i32>>,
    /// The signature type desired on the certificate. Available values: `origin-rsa`, `origin-ecc`, `keyless-certificate`. **Modifying this attribute will force creation of a new resource.**
    pub request_type: pulumi_wasm_rust::Output<String>,
    /// The number of days for which the certificate should be valid. Available values: `7`, `30`, `90`, `365`, `730`, `1095`, `5475`. **Modifying this attribute will force creation of a new resource.**
    pub requested_validity: pulumi_wasm_rust::Output<i32>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: OriginCaCertificateArgs) -> OriginCaCertificateResult {
    let result = crate::bindings::pulumi::cloudflare::origin_ca_certificate::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::origin_ca_certificate::Args {
            csr: &args.csr.get_inner(),
            hostnames: &args.hostnames.get_inner(),
            min_days_for_renewal: &args.min_days_for_renewal.get_inner(),
            request_type: &args.request_type.get_inner(),
            requested_validity: &args.requested_validity.get_inner(),
        },
    );

    OriginCaCertificateResult {
        certificate: crate::into_domain(result.certificate),
        csr: crate::into_domain(result.csr),
        expires_on: crate::into_domain(result.expires_on),
        hostnames: crate::into_domain(result.hostnames),
        min_days_for_renewal: crate::into_domain(result.min_days_for_renewal),
        request_type: crate::into_domain(result.request_type),
        requested_validity: crate::into_domain(result.requested_validity),
    }
}
