/// Resource for managing an AWS Managed Streaming for Kafka Replicator.
///
/// ## Example Usage
///
/// ### Basic Usage
///
///
/// ## Import
///
/// Using `pulumi import`, import MSK replicators using the replicator ARN. For example:
///
/// ```sh
/// $ pulumi import aws:msk/replicator:Replicator example arn:aws:kafka:us-west-2:123456789012:configuration/example/279c0212-d057-4dba-9aa9-1c4e5a25bfc7-3
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod replicator {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ReplicatorArgs {
        /// A summary description of the replicator.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of Kafka clusters which are targets of the replicator.
        #[builder(into)]
        pub kafka_clusters: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::msk::ReplicatorKafkaCluster>,
        >,
        /// A list of replication configurations, where each configuration targets a given source cluster to target cluster replication flow.
        #[builder(into)]
        pub replication_info_list: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::msk::ReplicatorReplicationInfoList,
        >,
        /// The name of the replicator.
        #[builder(into)]
        pub replicator_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ARN of the IAM role used by the replicator to access resources in the customer's account (e.g source and target clusters).
        #[builder(into)]
        pub service_execution_role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ReplicatorResult {
        /// ARN of the Replicator. Do not begin the description with "An", "The", "Defines", "Indicates", or "Specifies," as these are verbose. In other words, "Indicates the amount of storage," can be rewritten as "Amount of storage," without losing any information.
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub current_version: pulumi_gestalt_rust::Output<String>,
        /// A summary description of the replicator.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// A list of Kafka clusters which are targets of the replicator.
        pub kafka_clusters: pulumi_gestalt_rust::Output<
            Vec<super::super::types::msk::ReplicatorKafkaCluster>,
        >,
        /// A list of replication configurations, where each configuration targets a given source cluster to target cluster replication flow.
        pub replication_info_list: pulumi_gestalt_rust::Output<
            super::super::types::msk::ReplicatorReplicationInfoList,
        >,
        /// The name of the replicator.
        pub replicator_name: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the IAM role used by the replicator to access resources in the customer's account (e.g source and target clusters).
        pub service_execution_role_arn: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ReplicatorArgs,
    ) -> ReplicatorResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let kafka_clusters_binding = args.kafka_clusters.get_output(context);
        let replication_info_list_binding = args
            .replication_info_list
            .get_output(context);
        let replicator_name_binding = args.replicator_name.get_output(context);
        let service_execution_role_arn_binding = args
            .service_execution_role_arn
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:msk/replicator:Replicator".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kafkaClusters".into(),
                    value: kafka_clusters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "replicationInfoList".into(),
                    value: replication_info_list_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "replicatorName".into(),
                    value: replicator_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceExecutionRoleArn".into(),
                    value: service_execution_role_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ReplicatorResult {
            arn: o.get_field("arn"),
            current_version: o.get_field("currentVersion"),
            description: o.get_field("description"),
            kafka_clusters: o.get_field("kafkaClusters"),
            replication_info_list: o.get_field("replicationInfoList"),
            replicator_name: o.get_field("replicatorName"),
            service_execution_role_arn: o.get_field("serviceExecutionRoleArn"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
