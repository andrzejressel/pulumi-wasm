#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_snapshot {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSnapshotArgs {
        /// Name of the snapshot.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Map of tags assigned to the snapshot.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetSnapshotResult {
        /// ARN of the snapshot.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The configuration of the cluster from which the snapshot was taken.
        pub cluster_configurations: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::memorydb::GetSnapshotClusterConfiguration>,
        >,
        /// Name of the MemoryDB cluster that this snapshot was taken from.
        pub cluster_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// ARN of the KMS key used to encrypt the snapshot at rest.
        pub kms_key_arn: pulumi_gestalt_rust::Output<String>,
        /// Name of the cluster.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Whether the snapshot is from an automatic backup (`automated`) or was created manually (`manual`).
        pub source: pulumi_gestalt_rust::Output<String>,
        /// Map of tags assigned to the snapshot.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetSnapshotArgs,
    ) -> GetSnapshotResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:memorydb/getSnapshot:getSnapshot".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetSnapshotResult {
            arn: o.get_field("arn"),
            cluster_configurations: o.get_field("clusterConfigurations"),
            cluster_name: o.get_field("clusterName"),
            id: o.get_field("id"),
            kms_key_arn: o.get_field("kmsKeyArn"),
            name: o.get_field("name"),
            source: o.get_field("source"),
            tags: o.get_field("tags"),
        }
    }
}
