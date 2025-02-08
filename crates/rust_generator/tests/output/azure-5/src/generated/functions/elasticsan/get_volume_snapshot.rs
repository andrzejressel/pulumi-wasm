#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_volume_snapshot {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVolumeSnapshotArgs {
        /// The name of the Elastic SAN Volume Snapshot.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Elastic SAN Volume Group ID within which the Elastic SAN Volume Snapshot exists.
        #[builder(into)]
        pub volume_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetVolumeSnapshotResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The resource ID from which the Snapshot is created.
        pub source_id: pulumi_gestalt_rust::Output<String>,
        /// The size of source volume.
        pub source_volume_size_in_gib: pulumi_gestalt_rust::Output<i32>,
        pub volume_group_id: pulumi_gestalt_rust::Output<String>,
        /// The source volume name of the Snapshot.
        pub volume_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetVolumeSnapshotArgs,
    ) -> GetVolumeSnapshotResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let volume_group_id_binding = args
            .volume_group_id
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:elasticsan/getVolumeSnapshot:getVolumeSnapshot".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "volumeGroupId".into(),
                    value: &volume_group_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetVolumeSnapshotResult {
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            source_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceId"),
            ),
            source_volume_size_in_gib: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceVolumeSizeInGib"),
            ),
            volume_group_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("volumeGroupId"),
            ),
            volume_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("volumeName"),
            ),
        }
    }
}
