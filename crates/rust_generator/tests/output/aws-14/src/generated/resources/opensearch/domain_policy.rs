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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod domain_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainPolicyArgs {
        /// IAM policy document specifying the access policies for the domain
        #[builder(into)]
        pub access_policies: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the domain.
        #[builder(into)]
        pub domain_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DomainPolicyResult {
        /// IAM policy document specifying the access policies for the domain
        pub access_policies: pulumi_gestalt_rust::Output<String>,
        /// Name of the domain.
        pub domain_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: DomainPolicyArgs,
    ) -> DomainPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let access_policies_binding = args
            .access_policies
            .get_output(context)
            .get_inner();
        let domain_name_binding = args.domain_name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:opensearch/domainPolicy:DomainPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        DomainPolicyResult {
            access_policies: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accessPolicies"),
            ),
            domain_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("domainName"),
            ),
        }
    }
}
