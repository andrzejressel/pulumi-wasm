/// Creates an HSM client certificate that an Amazon Redshift cluster will use to connect to the client's HSM in order to store and retrieve the keys used to encrypt the cluster databases.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = hsm_client_certificate::create(
///         "example",
///         HsmClientCertificateArgs::builder()
///             .hsm_client_certificate_identifier("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Redshift HSM Client Certificates using `hsm_client_certificate_identifier`. For example:
///
/// ```sh
/// $ pulumi import aws:redshift/hsmClientCertificate:HsmClientCertificate test example
/// ```
pub mod hsm_client_certificate {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HsmClientCertificateArgs {
        /// The identifier of the HSM client certificate.
        #[builder(into)]
        pub hsm_client_certificate_identifier: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct HsmClientCertificateResult {
        /// Amazon Resource Name (ARN) of the Hsm Client Certificate.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The identifier of the HSM client certificate.
        pub hsm_client_certificate_identifier: pulumi_wasm_rust::Output<String>,
        /// The public key that the Amazon Redshift cluster will use to connect to the HSM. You must register the public key in the HSM.
        pub hsm_client_certificate_public_key: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: HsmClientCertificateArgs,
    ) -> HsmClientCertificateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let hsm_client_certificate_identifier_binding = args
            .hsm_client_certificate_identifier
            .get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:redshift/hsmClientCertificate:HsmClientCertificate".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "hsmClientCertificateIdentifier".into(),
                    value: &hsm_client_certificate_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "hsmClientCertificateIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "hsmClientCertificatePublicKey".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        HsmClientCertificateResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            hsm_client_certificate_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hsmClientCertificateIdentifier").unwrap(),
            ),
            hsm_client_certificate_public_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hsmClientCertificatePublicKey").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
