/// Creates a domain resource for the specified domain (e.g., example.com).
/// You cannot register a new domain name using Lightsail. You must register
/// a domain name using Amazon Route 53 or another domain name registrar.
/// If you have already registered your domain, you can enter its name in
/// this parameter to manage the DNS records for that domain.
///
/// > **Note:** Lightsail is currently only supported in a limited number of AWS Regions, please see ["Regions and Availability Zones in Amazon Lightsail"](https://lightsail.aws.amazon.com/ls/docs/overview/article/understanding-regions-and-availability-zones-in-amazon-lightsail) for more details
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let domainTest = domain::create(
///         "domainTest",
///         DomainArgs::builder().domain_name("mydomain.com").build_struct(),
///     );
/// }
/// ```
pub mod domain {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainArgs {
        /// The name of the Lightsail domain to manage
        #[builder(into)]
        pub domain_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct DomainResult {
        /// The ARN of the Lightsail domain
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The name of the Lightsail domain to manage
        pub domain_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DomainArgs) -> DomainResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let domain_name_binding = args.domain_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lightsail/domain:Domain".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "domainName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DomainResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            domain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainName").unwrap(),
            ),
        }
    }
}
