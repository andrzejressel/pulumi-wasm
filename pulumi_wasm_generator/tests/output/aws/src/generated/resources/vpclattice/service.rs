/// Resource for managing an AWS VPC Lattice Service.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = service::create(
///         "example",
///         ServiceArgs::builder()
///             .auth_type("AWS_IAM")
///             .custom_domain_name("example.com")
///             .name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import VPC Lattice Service using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:vpclattice/service:Service example svc-06728e2357ea55f8a
/// ```
pub mod service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceArgs {
        /// Type of IAM policy. Either `NONE` or `AWS_IAM`.
        #[builder(into, default)]
        pub auth_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Amazon Resource Name (ARN) of the certificate.
        #[builder(into, default)]
        pub certificate_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Custom domain name of the service.
        #[builder(into, default)]
        pub custom_domain_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the service. The name must be unique within the account. The valid characters are a-z, 0-9, and hyphens (-). You can't use a hyphen as the first or last character, or immediately after another hyphen.Must be between 3 and 40 characters in length.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ServiceResult {
        /// ARN of the service.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Type of IAM policy. Either `NONE` or `AWS_IAM`.
        pub auth_type: pulumi_wasm_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the certificate.
        pub certificate_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Custom domain name of the service.
        pub custom_domain_name: pulumi_wasm_rust::Output<Option<String>>,
        /// DNS name of the service.
        pub dns_entries: pulumi_wasm_rust::Output<
            Vec<super::super::types::vpclattice::ServiceDnsEntry>,
        >,
        /// Name of the service. The name must be unique within the account. The valid characters are a-z, 0-9, and hyphens (-). You can't use a hyphen as the first or last character, or immediately after another hyphen.Must be between 3 and 40 characters in length.
        ///
        /// The following arguments are optional:
        pub name: pulumi_wasm_rust::Output<String>,
        /// Status of the service.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ServiceArgs) -> ServiceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let auth_type_binding = args.auth_type.get_inner();
        let certificate_arn_binding = args.certificate_arn.get_inner();
        let custom_domain_name_binding = args.custom_domain_name.get_inner();
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:vpclattice/service:Service".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "authType".into(),
                    value: &auth_type_binding,
                },
                register_interface::ObjectField {
                    name: "certificateArn".into(),
                    value: &certificate_arn_binding,
                },
                register_interface::ObjectField {
                    name: "customDomainName".into(),
                    value: &custom_domain_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
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
                    name: "authType".into(),
                },
                register_interface::ResultField {
                    name: "certificateArn".into(),
                },
                register_interface::ResultField {
                    name: "customDomainName".into(),
                },
                register_interface::ResultField {
                    name: "dnsEntries".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
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
        ServiceResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            auth_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authType").unwrap(),
            ),
            certificate_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateArn").unwrap(),
            ),
            custom_domain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customDomainName").unwrap(),
            ),
            dns_entries: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsEntries").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
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