/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = certificate::create(
///         "example",
///         CertificateArgs::builder()
///             .certificate_identifier("rds-ca-rsa4096-g1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import the default EBS encryption state. For example:
///
/// ```sh
/// $ pulumi import aws:rds/certificate:Certificate example default
/// ```
pub mod certificate {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CertificateArgs {
        /// Certificate identifier. For example, `rds-ca-rsa4096-g1`. Refer to [AWS RDS (Relational Database) Certificate Identifier](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/UsingWithRDS.SSL.html#UsingWithRDS.SSL.CertificateIdentifier) for more information.
        #[builder(into)]
        pub certificate_identifier: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct CertificateResult {
        /// Certificate identifier. For example, `rds-ca-rsa4096-g1`. Refer to [AWS RDS (Relational Database) Certificate Identifier](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/UsingWithRDS.SSL.html#UsingWithRDS.SSL.CertificateIdentifier) for more information.
        pub certificate_identifier: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: CertificateArgs) -> CertificateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let certificate_identifier_binding = args.certificate_identifier.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:rds/certificate:Certificate".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "certificateIdentifier".into(),
                    value: &certificate_identifier_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "certificateIdentifier".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CertificateResult {
            certificate_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateIdentifier").unwrap(),
            ),
        }
    }
}