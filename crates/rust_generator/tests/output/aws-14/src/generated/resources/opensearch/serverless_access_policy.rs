/// Resource for managing an AWS OpenSearch Serverless Access Policy. See AWS documentation for [data access policies](https://docs.aws.amazon.com/opensearch-service/latest/developerguide/serverless-data-access.html) and [supported data access policy permissions](https://docs.aws.amazon.com/opensearch-service/latest/developerguide/serverless-data-access.html#serverless-data-supported-permissions).
///
/// ## Example Usage
///
/// ### Grant all collection and index permissions
///
/// ```yaml
/// resources:
///   example:
///     type: aws:opensearch:ServerlessAccessPolicy
///     properties:
///       name: example
///       type: data
///       description: read and write permissions
///       policy:
///         fn::toJSON:
///           - Rules:
///               - ResourceType: index
///                 Resource:
///                   - index/example-collection/*
///                 Permission:
///                   - aoss:*
///               - ResourceType: collection
///                 Resource:
///                   - collection/example-collection
///                 Permission:
///                   - aoss:*
///             Principal:
///               - ${current.arn}
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
/// ```
///
/// ### Grant read-only collection and index permissions
///
/// ```yaml
/// resources:
///   example:
///     type: aws:opensearch:ServerlessAccessPolicy
///     properties:
///       name: example
///       type: data
///       description: read-only permissions
///       policy:
///         fn::toJSON:
///           - Rules:
///               - ResourceType: index
///                 Resource:
///                   - index/example-collection/*
///                 Permission:
///                   - aoss:DescribeIndex
///                   - aoss:ReadDocument
///               - ResourceType: collection
///                 Resource:
///                   - collection/example-collection
///                 Permission:
///                   - aoss:DescribeCollectionItems
///             Principal:
///               - ${current.arn}
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
/// ```
///
/// ### Grant SAML identity permissions
///
/// ```yaml
/// resources:
///   example:
///     type: aws:opensearch:ServerlessAccessPolicy
///     properties:
///       name: example
///       type: data
///       description: saml permissions
///       policy:
///         fn::toJSON:
///           - Rules:
///               - ResourceType: index
///                 Resource:
///                   - index/example-collection/*
///                 Permission:
///                   - aoss:*
///               - ResourceType: collection
///                 Resource:
///                   - collection/example-collection
///                 Permission:
///                   - aoss:*
///             Principal:
///               - saml/123456789012/myprovider/user/Annie
///               - saml/123456789012/anotherprovider/group/Accounting
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import OpenSearchServerless Access Policy using the `name` and `type` arguments separated by a slash (`/`). For example:
///
/// ```sh
/// $ pulumi import aws:opensearch/serverlessAccessPolicy:ServerlessAccessPolicy example example/data
/// ```
#[allow(clippy::doc_lazy_continuation)]
pub mod serverless_access_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServerlessAccessPolicyArgs {
        /// Description of the policy. Typically used to store information about the permissions defined in the policy.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the policy.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// JSON policy document to use as the content for the new policy
        #[builder(into)]
        pub policy: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Type of access policy. Must be `data`.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ServerlessAccessPolicyResult {
        /// Description of the policy. Typically used to store information about the permissions defined in the policy.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the policy.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// JSON policy document to use as the content for the new policy
        pub policy: pulumi_gestalt_rust::Output<String>,
        /// Version of the policy.
        pub policy_version: pulumi_gestalt_rust::Output<String>,
        /// Type of access policy. Must be `data`.
        ///
        /// The following arguments are optional:
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ServerlessAccessPolicyArgs,
    ) -> ServerlessAccessPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let policy_binding = args.policy.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:opensearch/serverlessAccessPolicy:ServerlessAccessPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "policy".into(),
                    value: &policy_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ServerlessAccessPolicyResult {
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("policy"),
            ),
            policy_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("policyVersion"),
            ),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}
