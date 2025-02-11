#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_open_zfs_snapshot {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOpenZfsSnapshotArgs {
        /// One or more name/value pairs to filter off of. The
        /// supported names are file-system-id or volume-id.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::fsx::GetOpenZfsSnapshotFilter>>,
        >,
        /// If more than one result is returned, use the most recent snapshot.
        #[builder(into, default)]
        pub most_recent: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Name of the snapshot.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Returns information on a specific snapshot_id.
        #[builder(into, default)]
        pub snapshot_ids: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// List of Tag values, with a maximum of 50 elements.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetOpenZfsSnapshotResult {
        /// Amazon Resource Name of the snapshot.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Time that the resource was created.
        pub creation_time: pulumi_gestalt_rust::Output<String>,
        pub filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::fsx::GetOpenZfsSnapshotFilter>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub most_recent: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Name of the snapshot.
        pub name: pulumi_gestalt_rust::Output<Option<String>>,
        /// ID of the snapshot.
        pub snapshot_id: pulumi_gestalt_rust::Output<String>,
        pub snapshot_ids: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// List of Tag values, with a maximum of 50 elements.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// ID of the volume that the snapshot is of.
        pub volume_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetOpenZfsSnapshotArgs,
    ) -> GetOpenZfsSnapshotResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filters_binding = args.filters.get_output(context);
        let most_recent_binding = args.most_recent.get_output(context);
        let name_binding = args.name.get_output(context);
        let snapshot_ids_binding = args.snapshot_ids.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:fsx/getOpenZfsSnapshot:getOpenZfsSnapshot".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: &filters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mostRecent".into(),
                    value: &most_recent_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "snapshotIds".into(),
                    value: &snapshot_ids_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetOpenZfsSnapshotResult {
            arn: o.get_field("arn"),
            creation_time: o.get_field("creationTime"),
            filters: o.get_field("filters"),
            id: o.get_field("id"),
            most_recent: o.get_field("mostRecent"),
            name: o.get_field("name"),
            snapshot_id: o.get_field("snapshotId"),
            snapshot_ids: o.get_field("snapshotIds"),
            tags: o.get_field("tags"),
            volume_id: o.get_field("volumeId"),
        }
    }
}
