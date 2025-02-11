/// Resource for managing an AWS Managed Streaming for Kafka Cluster Policy.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:msk:ClusterPolicy
///     properties:
///       clusterArn: ${exampleAwsMskCluster.arn}
///       policy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Sid: ExampleMskClusterPolicy
///               Effect: Allow
///               Principal:
///                 AWS: arn:${currentGetPartition.partition}:iam::${current.accountId}:root
///               Action:
///                 - kafka:Describe*
///                 - kafka:Get*
///                 - kafka:CreateVpcConnection
///                 - kafka:GetBootstrapBrokers
///               Resource: ${exampleAwsMskCluster.arn}
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
/// Using `pulumi import`, import Managed Streaming for Kafka Cluster Policy using the `cluster_arn`. For example:
///
/// ```sh
/// $ pulumi import aws:msk/clusterPolicy:ClusterPolicy example arn:aws:kafka:us-west-2:123456789012:cluster/example/279c0212-d057-4dba-9aa9-1c4e5a25bfc7-3
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cluster_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterPolicyArgs {
        /// The Amazon Resource Name (ARN) that uniquely identifies the cluster.
        #[builder(into)]
        pub cluster_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Resource policy for cluster.
        #[builder(into)]
        pub policy: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ClusterPolicyResult {
        /// The Amazon Resource Name (ARN) that uniquely identifies the cluster.
        pub cluster_arn: pulumi_gestalt_rust::Output<String>,
        pub current_version: pulumi_gestalt_rust::Output<String>,
        /// Resource policy for cluster.
        pub policy: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ClusterPolicyArgs,
    ) -> ClusterPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cluster_arn_binding = args.cluster_arn.get_output(context);
        let policy_binding = args.policy.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:msk/clusterPolicy:ClusterPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterArn".into(),
                    value: &cluster_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policy".into(),
                    value: &policy_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ClusterPolicyResult {
            cluster_arn: o.get_field("clusterArn"),
            current_version: o.get_field("currentVersion"),
            policy: o.get_field("policy"),
        }
    }
}
