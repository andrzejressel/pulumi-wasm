/// Provides a Glue resource policy. Only one can exist per region.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:glue:ResourcePolicy
///     properties:
///       policy: ${["glue-example-policy"].json}
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
///   currentGetPartition:
///     fn::invoke:
///       function: aws:getPartition
///       arguments: {}
///   currentGetRegion:
///     fn::invoke:
///       function: aws:getRegion
///       arguments: {}
///   glue-example-policy:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - actions:
///               - glue:CreateTable
///             resources:
///               - arn:${currentGetPartition.partition}:glue:${currentGetRegion.name}:${current.accountId}:*
///             principals:
///               - identifiers:
///                   - '*'
///                 type: AWS
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Glue Resource Policy using the account ID. For example:
///
/// ```sh
/// $ pulumi import aws:glue/resourcePolicy:ResourcePolicy Test 12356789012
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod resource_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourcePolicyArgs {
        /// Indicates that you are using both methods to grant cross-account. Valid values are `TRUE` and `FALSE`. Note the provider will not perform drift detetction on this field as its not return on read.
        #[builder(into, default)]
        pub enable_hybrid: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The policy to be applied to the aws glue data catalog.
        #[builder(into)]
        pub policy: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ResourcePolicyResult {
        /// Indicates that you are using both methods to grant cross-account. Valid values are `TRUE` and `FALSE`. Note the provider will not perform drift detetction on this field as its not return on read.
        pub enable_hybrid: pulumi_gestalt_rust::Output<Option<String>>,
        /// The policy to be applied to the aws glue data catalog.
        pub policy: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ResourcePolicyArgs,
    ) -> ResourcePolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let enable_hybrid_binding_1 = args.enable_hybrid.get_output(context);
        let enable_hybrid_binding = enable_hybrid_binding_1.get_inner();
        let policy_binding_1 = args.policy.get_output(context);
        let policy_binding = policy_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:glue/resourcePolicy:ResourcePolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "enableHybrid".into(),
                    value: &enable_hybrid_binding,
                },
                register_interface::ObjectField {
                    name: "policy".into(),
                    value: &policy_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ResourcePolicyResult {
            enable_hybrid: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableHybrid"),
            ),
            policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("policy"),
            ),
        }
    }
}
