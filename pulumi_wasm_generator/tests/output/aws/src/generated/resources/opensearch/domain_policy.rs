/// Allows setting policy to an OpenSearch domain while referencing domain attributes (e.g., ARN).
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:opensearch:Domain
///     properties:
///       domainName: tf-test
///       engineVersion: OpenSearch_1.1
///   mainDomainPolicy:
///     type: aws:opensearch:DomainPolicy
///     name: main
///     properties:
///       domainName: ${example.domainName}
///       accessPolicies: ${main.json}
/// variables:
///   main:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: '*'
///                 identifiers:
///                   - '*'
///             actions:
///               - es:*
///             resources:
///               - ${example.arn}/*
///             conditions:
///               - test: IpAddress
///                 variable: aws:SourceIp
///                 values:
///                   - 127.0.0.1/32
/// ```
pub mod domain_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainPolicyArgs {
        /// IAM policy document specifying the access policies for the domain
        #[builder(into)]
        pub access_policies: pulumi_wasm_rust::Output<String>,
        /// Name of the domain.
        #[builder(into)]
        pub domain_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct DomainPolicyResult {
        /// IAM policy document specifying the access policies for the domain
        pub access_policies: pulumi_wasm_rust::Output<String>,
        /// Name of the domain.
        pub domain_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DomainPolicyArgs) -> DomainPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_policies_binding = args.access_policies.get_inner();
        let domain_name_binding = args.domain_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:opensearch/domainPolicy:DomainPolicy".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessPolicies".into(),
                    value: &access_policies_binding,
                },
                register_interface::ObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessPolicies".into(),
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
        DomainPolicyResult {
            access_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessPolicies").unwrap(),
            ),
            domain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainName").unwrap(),
            ),
        }
    }
}
