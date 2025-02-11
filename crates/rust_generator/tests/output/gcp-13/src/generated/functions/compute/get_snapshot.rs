#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_snapshot {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSnapshotArgs {
        /// A filter to retrieve the compute snapshot.
        /// See [API filter parameter documentation](https://cloud.google.com/compute/docs/reference/rest/v1/snapshots/list#body.QUERY_PARAMETERS.filter) for reference.
        /// If multiple compute snapshot match, either adjust the filter or specify `most_recent`. One of `name` or `filter` must be provided.
        /// If you want to use a regular expression, use the `eq` (equal) or `ne` (not equal) operator against a single un-parenthesized expression with or without quotes or against multiple parenthesized expressions. Example `sourceDisk eq '.*(.*/data-disk$).*'`. More details for golang Snapshots list call filters [here](https://pkg.go.dev/google.golang.org/api/compute/v1#SnapshotsListCall.Filter).
        #[builder(into, default)]
        pub filter: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If `filter` is provided, ensures the most recent snapshot is returned when multiple compute snapshot match.
        ///
        /// - - -
        #[builder(into, default)]
        pub most_recent: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the compute snapshot. One of `name` or `filter` must be provided.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetSnapshotResult {
        pub chain_name: pulumi_gestalt_rust::Output<String>,
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        pub description: pulumi_gestalt_rust::Output<String>,
        pub disk_size_gb: pulumi_gestalt_rust::Output<i32>,
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub filter: pulumi_gestalt_rust::Output<Option<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub label_fingerprint: pulumi_gestalt_rust::Output<String>,
        pub labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub licenses: pulumi_gestalt_rust::Output<Vec<String>>,
        pub most_recent: pulumi_gestalt_rust::Output<Option<bool>>,
        pub name: pulumi_gestalt_rust::Output<Option<String>>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub self_link: pulumi_gestalt_rust::Output<String>,
        pub snapshot_encryption_keys: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetSnapshotSnapshotEncryptionKey>,
        >,
        pub snapshot_id: pulumi_gestalt_rust::Output<i32>,
        pub source_disk: pulumi_gestalt_rust::Output<String>,
        pub source_disk_encryption_keys: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetSnapshotSourceDiskEncryptionKey>,
        >,
        pub storage_bytes: pulumi_gestalt_rust::Output<i32>,
        pub storage_locations: pulumi_gestalt_rust::Output<Vec<String>>,
        pub zone: pulumi_gestalt_rust::Output<String>,
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
        let filter_binding = args.filter.get_output(context);
        let most_recent_binding = args.most_recent.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:compute/getSnapshot:getSnapshot".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filter".into(),
                    value: &filter_binding.drop_type(),
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
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetSnapshotResult {
            chain_name: o.get_field("chainName"),
            creation_timestamp: o.get_field("creationTimestamp"),
            description: o.get_field("description"),
            disk_size_gb: o.get_field("diskSizeGb"),
            effective_labels: o.get_field("effectiveLabels"),
            filter: o.get_field("filter"),
            id: o.get_field("id"),
            label_fingerprint: o.get_field("labelFingerprint"),
            labels: o.get_field("labels"),
            licenses: o.get_field("licenses"),
            most_recent: o.get_field("mostRecent"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            self_link: o.get_field("selfLink"),
            snapshot_encryption_keys: o.get_field("snapshotEncryptionKeys"),
            snapshot_id: o.get_field("snapshotId"),
            source_disk: o.get_field("sourceDisk"),
            source_disk_encryption_keys: o.get_field("sourceDiskEncryptionKeys"),
            storage_bytes: o.get_field("storageBytes"),
            storage_locations: o.get_field("storageLocations"),
            zone: o.get_field("zone"),
        }
    }
}
