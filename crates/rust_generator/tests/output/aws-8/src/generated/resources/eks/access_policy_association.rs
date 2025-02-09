/// Access Entry Policy Association for an EKS Cluster.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:eks:AccessPolicyAssociation
///     properties:
///       clusterName: ${exampleAwsEksCluster.name}
///       policyArn: arn:aws:eks::aws:cluster-access-policy/AmazonEKSViewPolicy
///       principalArn: ${exampleAwsIamUser.arn}
///       accessScope:
///         type: namespace
///         namespaces:
///           - example-namespace
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import EKS access entry using the `cluster_name` `principal_arn` and `policy_arn` separated by an octothorp (`#`). For example:
///
/// ```sh
/// $ pulumi import aws:eks/accessPolicyAssociation:AccessPolicyAssociation my_eks_access_entry my_cluster_name#my_principal_arn#my_policy_arn
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod access_policy_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccessPolicyAssociationArgs {
        /// The configuration block to determine the scope of the access. See `access_scope` Block below.
        #[builder(into)]
        pub access_scope: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::eks::AccessPolicyAssociationAccessScope,
        >,
        /// Name of the EKS Cluster.
        #[builder(into)]
        pub cluster_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ARN of the access policy that you're associating.
        #[builder(into)]
        pub policy_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The IAM Principal ARN which requires Authentication access to the EKS cluster.
        #[builder(into)]
        pub principal_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AccessPolicyAssociationResult {
        /// The configuration block to determine the scope of the access. See `access_scope` Block below.
        pub access_scope: pulumi_gestalt_rust::Output<
            super::super::types::eks::AccessPolicyAssociationAccessScope,
        >,
        /// Date and time in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8) that the policy was associated.
        pub associated_at: pulumi_gestalt_rust::Output<String>,
        /// Name of the EKS Cluster.
        pub cluster_name: pulumi_gestalt_rust::Output<String>,
        /// Date and time in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8) that the policy was updated.
        pub modified_at: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the access policy that you're associating.
        pub policy_arn: pulumi_gestalt_rust::Output<String>,
        /// The IAM Principal ARN which requires Authentication access to the EKS cluster.
        pub principal_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AccessPolicyAssociationArgs,
    ) -> AccessPolicyAssociationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let access_scope_binding = args.access_scope.get_output(context);
        let cluster_name_binding = args.cluster_name.get_output(context);
        let policy_arn_binding = args.policy_arn.get_output(context);
        let principal_arn_binding = args.principal_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:eks/accessPolicyAssociation:AccessPolicyAssociation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accessScope".into(),
                    value: access_scope_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterName".into(),
                    value: cluster_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyArn".into(),
                    value: policy_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "principalArn".into(),
                    value: principal_arn_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AccessPolicyAssociationResult {
            access_scope: o.get_field("accessScope"),
            associated_at: o.get_field("associatedAt"),
            cluster_name: o.get_field("clusterName"),
            modified_at: o.get_field("modifiedAt"),
            policy_arn: o.get_field("policyArn"),
            principal_arn: o.get_field("principalArn"),
        }
    }
}
