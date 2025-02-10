/// Resource for managing an AWS Managed Streaming for Kafka VPC Connection.
///
/// ## Example Usage
///
///
/// ## Import
///
/// Using `pulumi import`, import MSK configurations using the configuration ARN. For example:
///
/// ```sh
/// $ pulumi import aws:msk/vpcConnection:VpcConnection example arn:aws:kafka:eu-west-2:123456789012:vpc-connection/123456789012/example/38173259-79cd-4ee8-87f3-682ea6023f48-2
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vpc_connection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcConnectionArgs {
        /// The authentication type for the client VPC connection. Specify one of these auth type strings: SASL_IAM, SASL_SCRAM, or TLS.
        #[builder(into)]
        pub authentication: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The list of subnets in the client VPC to connect to.
        #[builder(into)]
        pub client_subnets: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The security groups to attach to the ENIs for the broker nodes.
        #[builder(into)]
        pub security_groups: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Amazon Resource Name (ARN) of the cluster.
        #[builder(into)]
        pub target_cluster_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The VPC ID of the remote client.
        #[builder(into)]
        pub vpc_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VpcConnectionResult {
        /// Amazon Resource Name (ARN) of the VPC connection.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The authentication type for the client VPC connection. Specify one of these auth type strings: SASL_IAM, SASL_SCRAM, or TLS.
        pub authentication: pulumi_gestalt_rust::Output<String>,
        /// The list of subnets in the client VPC to connect to.
        pub client_subnets: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The security groups to attach to the ENIs for the broker nodes.
        pub security_groups: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The Amazon Resource Name (ARN) of the cluster.
        pub target_cluster_arn: pulumi_gestalt_rust::Output<String>,
        /// The VPC ID of the remote client.
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpcConnectionArgs,
    ) -> VpcConnectionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let authentication_binding = args.authentication.get_output(context);
        let client_subnets_binding = args.client_subnets.get_output(context);
        let security_groups_binding = args.security_groups.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let target_cluster_arn_binding = args.target_cluster_arn.get_output(context);
        let vpc_id_binding = args.vpc_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:msk/vpcConnection:VpcConnection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authentication".into(),
                    value: authentication_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientSubnets".into(),
                    value: client_subnets_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "securityGroups".into(),
                    value: security_groups_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetClusterArn".into(),
                    value: target_cluster_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcId".into(),
                    value: vpc_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        VpcConnectionResult {
            arn: o.get_field("arn"),
            authentication: o.get_field("authentication"),
            client_subnets: o.get_field("clientSubnets"),
            security_groups: o.get_field("securityGroups"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            target_cluster_arn: o.get_field("targetClusterArn"),
            vpc_id: o.get_field("vpcId"),
        }
    }
}
