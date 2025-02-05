pub mod get_open_zfs_snapshot {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOpenZfsSnapshotArgs {
        /// One or more name/value pairs to filter off of. The
        /// supported names are file-system-id or volume-id.
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::super::types::fsx::GetOpenZfsSnapshotFilter>>,
        >,
        /// If more than one result is returned, use the most recent snapshot.
        #[builder(into, default)]
        pub most_recent: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Name of the snapshot.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Returns information on a specific snapshot_id.
        #[builder(into, default)]
        pub snapshot_ids: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// List of Tag values, with a maximum of 50 elements.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetOpenZfsSnapshotResult {
        /// Amazon Resource Name of the snapshot.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Time that the resource was created.
        pub creation_time: pulumi_wasm_rust::Output<String>,
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::fsx::GetOpenZfsSnapshotFilter>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub most_recent: pulumi_wasm_rust::Output<Option<bool>>,
        /// Name of the snapshot.
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// ID of the snapshot.
        pub snapshot_id: pulumi_wasm_rust::Output<String>,
        pub snapshot_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// List of Tag values, with a maximum of 50 elements.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// ID of the volume that the snapshot is of.
        pub volume_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetOpenZfsSnapshotArgs,
    ) -> GetOpenZfsSnapshotResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_output(context).get_inner();
        let most_recent_binding = args.most_recent.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let snapshot_ids_binding = args.snapshot_ids.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:fsx/getOpenZfsSnapshot:getOpenZfsSnapshot".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "mostRecent".into(),
                    value: &most_recent_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "snapshotIds".into(),
                    value: &snapshot_ids_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetOpenZfsSnapshotResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            creation_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("creationTime"),
            ),
            filters: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("filters"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            most_recent: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("mostRecent"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            snapshot_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("snapshotId"),
            ),
            snapshot_ids: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("snapshotIds"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            volume_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("volumeId"),
            ),
        }
    }
}
