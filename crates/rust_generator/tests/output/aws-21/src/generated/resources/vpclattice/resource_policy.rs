/// Resource for managing an AWS VPC Lattice Resource Policy.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:vpclattice:ServiceNetwork
///     properties:
///       name: example-vpclattice-service-network
///   exampleResourcePolicy:
///     type: aws:vpclattice:ResourcePolicy
///     name: example
///     properties:
///       resourceArn: ${example.arn}
///       policy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Sid: test-pol-principals-6
///               Effect: Allow
///               Principal:
///                 AWS: arn:${currentGetPartition.partition}:iam::${current.accountId}:root
///               Action:
///                 - vpc-lattice:CreateServiceNetworkVpcAssociation
///                 - vpc-lattice:CreateServiceNetworkServiceAssociation
///                 - vpc-lattice:GetServiceNetwork
///               Resource: ${example.arn}
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
///   currentGetPartition:
///     fn::invoke:
///       function: aws:getPartition
///       arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import VPC Lattice Resource Policy using the `resource_arn`. For example:
///
/// ```sh
/// $ pulumi import aws:vpclattice/resourcePolicy:ResourcePolicy example rft-8012925589
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod resource_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourcePolicyArgs {
        /// An IAM policy. The policy string in JSON must not contain newlines or blank lines.
        #[builder(into)]
        pub policy: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID or Amazon Resource Name (ARN) of the service network or service for which the policy is created.
        #[builder(into)]
        pub resource_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ResourcePolicyResult {
        /// An IAM policy. The policy string in JSON must not contain newlines or blank lines.
        pub policy: pulumi_gestalt_rust::Output<String>,
        /// The ID or Amazon Resource Name (ARN) of the service network or service for which the policy is created.
        pub resource_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ResourcePolicyArgs,
    ) -> ResourcePolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let policy_binding = args.policy.get_output(context);
        let resource_arn_binding = args.resource_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:vpclattice/resourcePolicy:ResourcePolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policy".into(),
                    value: policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceArn".into(),
                    value: resource_arn_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ResourcePolicyResult {
            policy: o.get_field("policy"),
            resource_arn: o.get_field("resourceArn"),
        }
    }
}
