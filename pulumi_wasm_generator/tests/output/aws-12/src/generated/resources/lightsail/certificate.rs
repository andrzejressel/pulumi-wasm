/// Provides a lightsail certificate.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = certificate::create(
///         "test",
///         CertificateArgs::builder()
///             .domain_name("testdomain.com")
///             .name("test")
///             .subject_alternative_names(vec!["www.testdomain.com",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_lightsail_certificate` using the certificate name. For example:
///
/// ```sh
/// $ pulumi import aws:lightsail/certificate:Certificate test CertificateName
/// ```
pub mod certificate {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CertificateArgs {
        /// A domain name for which the certificate should be issued.
        #[builder(into, default)]
        pub domain_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Lightsail load balancer.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Set of domains that should be SANs in the issued certificate. `domain_name` attribute is automatically added as a Subject Alternative Name.
        #[builder(into, default)]
        pub subject_alternative_names: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// A map of tags to assign to the resource. To create a key-only tag, use an empty string as the value. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct CertificateResult {
        /// The ARN of the lightsail certificate.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The timestamp when the instance was created.
        pub created_at: pulumi_wasm_rust::Output<String>,
        /// A domain name for which the certificate should be issued.
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// Set of domain validation objects which can be used to complete certificate validation. Can have more than one element, e.g., if SANs are defined.
        pub domain_validation_options: pulumi_wasm_rust::Output<
            Vec<super::super::types::lightsail::CertificateDomainValidationOption>,
        >,
        /// The name of the Lightsail load balancer.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Set of domains that should be SANs in the issued certificate. `domain_name` attribute is automatically added as a Subject Alternative Name.
        pub subject_alternative_names: pulumi_wasm_rust::Output<Vec<String>>,
        /// A map of tags to assign to the resource. To create a key-only tag, use an empty string as the value. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: CertificateArgs,
    ) -> CertificateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let domain_name_binding = args.domain_name.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let subject_alternative_names_binding = args
            .subject_alternative_names
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lightsail/certificate:Certificate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "subjectAlternativeNames".into(),
                    value: &subject_alternative_names_binding,
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
                    name: "createdAt".into(),
                },
                register_interface::ResultField {
                    name: "domainName".into(),
                },
                register_interface::ResultField {
                    name: "domainValidationOptions".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "subjectAlternativeNames".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CertificateResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            created_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdAt").unwrap(),
            ),
            domain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainName").unwrap(),
            ),
            domain_validation_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainValidationOptions").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            subject_alternative_names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subjectAlternativeNames").unwrap(),
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
