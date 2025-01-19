/// Provides an AppSync Domain Name.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = domain_name::create(
///         "example",
///         DomainNameArgs::builder()
///             .certificate_arn("${exampleAwsAcmCertificate.arn}")
///             .domain_name("api.example.com")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_appsync_domain_name` using the AppSync domain name. For example:
///
/// ```sh
/// $ pulumi import aws:appsync/domainName:DomainName example example.com
/// ```
pub mod domain_name {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainNameArgs {
        /// ARN of the certificate. This can be an Certificate Manager (ACM) certificate or an Identity and Access Management (IAM) server certificate. The certifiacte must reside in us-east-1.
        #[builder(into)]
        pub certificate_arn: pulumi_wasm_rust::Output<String>,
        /// A description of the Domain Name.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Domain name.
        #[builder(into)]
        pub domain_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct DomainNameResult {
        /// Domain name that AppSync provides.
        pub appsync_domain_name: pulumi_wasm_rust::Output<String>,
        /// ARN of the certificate. This can be an Certificate Manager (ACM) certificate or an Identity and Access Management (IAM) server certificate. The certifiacte must reside in us-east-1.
        pub certificate_arn: pulumi_wasm_rust::Output<String>,
        /// A description of the Domain Name.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Domain name.
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// ID of your Amazon Route 53 hosted zone.
        pub hosted_zone_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DomainNameArgs) -> DomainNameResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let certificate_arn_binding = args.certificate_arn.get_inner();
        let description_binding = args.description.get_inner();
        let domain_name_binding = args.domain_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:appsync/domainName:DomainName".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "certificateArn".into(),
                    value: &certificate_arn_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "appsyncDomainName".into(),
                },
                register_interface::ResultField {
                    name: "certificateArn".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "domainName".into(),
                },
                register_interface::ResultField {
                    name: "hostedZoneId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DomainNameResult {
            appsync_domain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appsyncDomainName").unwrap(),
            ),
            certificate_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateArn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            domain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainName").unwrap(),
            ),
            hosted_zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostedZoneId").unwrap(),
            ),
        }
    }
}
