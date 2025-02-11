/// Manages an Amazon MSK Serverless cluster.
///
/// > **Note:** To manage a _provisioned_ Amazon MSK cluster, use the `aws.msk.Cluster` resource.
///
/// ## Example Usage
///
///
/// ## Import
///
/// Using `pulumi import`, import MSK serverless clusters using the cluster `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:msk/serverlessCluster:ServerlessCluster example arn:aws:kafka:us-west-2:123456789012:cluster/example/279c0212-d057-4dba-9aa9-1c4e5a25bfc7-3
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod serverless_cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServerlessClusterArgs {
        /// Specifies client authentication information for the serverless cluster. See below.
        #[builder(into)]
        pub client_authentication: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::msk::ServerlessClusterClientAuthentication,
        >,
        /// The name of the serverless cluster.
        #[builder(into, default)]
        pub cluster_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// VPC configuration information. See below.
        #[builder(into)]
        pub vpc_configs: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::msk::ServerlessClusterVpcConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct ServerlessClusterResult {
        /// The ARN of the serverless cluster.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Specifies client authentication information for the serverless cluster. See below.
        pub client_authentication: pulumi_gestalt_rust::Output<
            super::super::types::msk::ServerlessClusterClientAuthentication,
        >,
        /// The name of the serverless cluster.
        pub cluster_name: pulumi_gestalt_rust::Output<String>,
        /// UUID of the serverless cluster, for use in IAM policies.
        pub cluster_uuid: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// VPC configuration information. See below.
        pub vpc_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::types::msk::ServerlessClusterVpcConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServerlessClusterArgs,
    ) -> ServerlessClusterResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let client_authentication_binding = args
            .client_authentication
            .get_output(context);
        let cluster_name_binding = args.cluster_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let vpc_configs_binding = args.vpc_configs.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:msk/serverlessCluster:ServerlessCluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientAuthentication".into(),
                    value: &client_authentication_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterName".into(),
                    value: &cluster_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcConfigs".into(),
                    value: &vpc_configs_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ServerlessClusterResult {
            arn: o.get_field("arn"),
            client_authentication: o.get_field("clientAuthentication"),
            cluster_name: o.get_field("clusterName"),
            cluster_uuid: o.get_field("clusterUuid"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            vpc_configs: o.get_field("vpcConfigs"),
        }
    }
}
