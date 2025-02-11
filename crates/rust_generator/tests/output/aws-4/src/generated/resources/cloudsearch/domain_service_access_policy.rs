/// Provides an CloudSearch domain service access policy resource.
///
/// The provider waits for the domain service access policy to become `Active` when applying a configuration.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleDomain:
///     type: aws:cloudsearch:Domain
///     name: example
///     properties:
///       name: example-domain
///   exampleDomainServiceAccessPolicy:
///     type: aws:cloudsearch:DomainServiceAccessPolicy
///     name: example
///     properties:
///       domainName: ${exampleDomain.id}
///       accessPolicy: ${example.json}
/// variables:
///   example:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - sid: search_only
///             effect: Allow
///             principals:
///               - type: '*'
///                 identifiers:
///                   - '*'
///             actions:
///               - cloudsearch:search
///               - cloudsearch:document
///             conditions:
///               - test: IpAddress
///                 variable: aws:SourceIp
///                 values:
///                   - 192.0.2.0/32
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CloudSearch domain service access policies using the domain name. For example:
///
/// ```sh
/// $ pulumi import aws:cloudsearch/domainServiceAccessPolicy:DomainServiceAccessPolicy example example-domain
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod domain_service_access_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainServiceAccessPolicyArgs {
        /// The access rules you want to configure. These rules replace any existing rules. See the [AWS documentation](https://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-access.html) for details.
        #[builder(into)]
        pub access_policy: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The CloudSearch domain name the policy applies to.
        #[builder(into)]
        pub domain_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DomainServiceAccessPolicyResult {
        /// The access rules you want to configure. These rules replace any existing rules. See the [AWS documentation](https://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-access.html) for details.
        pub access_policy: pulumi_gestalt_rust::Output<String>,
        /// The CloudSearch domain name the policy applies to.
        pub domain_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DomainServiceAccessPolicyArgs,
    ) -> DomainServiceAccessPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let access_policy_binding = args.access_policy.get_output(context);
        let domain_name_binding = args.domain_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cloudsearch/domainServiceAccessPolicy:DomainServiceAccessPolicy"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accessPolicy".into(),
                    value: &access_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        DomainServiceAccessPolicyResult {
            access_policy: o.get_field("accessPolicy"),
            domain_name: o.get_field("domainName"),
        }
    }
}
