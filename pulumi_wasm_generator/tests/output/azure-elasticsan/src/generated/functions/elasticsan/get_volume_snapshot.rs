pub mod get_volume_snapshot {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVolumeSnapshotArgs {
        /// The name of the Elastic SAN Volume Snapshot.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The Elastic SAN Volume Group ID within which the Elastic SAN Volume Snapshot exists.
        #[builder(into)]
        pub volume_group_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetVolumeSnapshotResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// The resource ID from which the Snapshot is created.
        pub source_id: pulumi_wasm_rust::Output<String>,
        /// The size of source volume.
        pub source_volume_size_in_gib: pulumi_wasm_rust::Output<i32>,
        pub volume_group_id: pulumi_wasm_rust::Output<String>,
        /// The source volume name of the Snapshot.
        pub volume_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetVolumeSnapshotArgs) -> GetVolumeSnapshotResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let volume_group_id_binding = args.volume_group_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:elasticsan/getVolumeSnapshot:getVolumeSnapshot".into(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "sourceId".into(),
                },
                register_interface::ResultField {
                    name: "sourceVolumeSizeInGib".into(),
                },
                register_interface::ResultField {
                    name: "volumeGroupId".into(),
                },
                register_interface::ResultField {
                    name: "volumeName".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetVolumeSnapshotResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            source_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceId").unwrap(),
            ),
            source_volume_size_in_gib: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceVolumeSizeInGib").unwrap(),
            ),
            volume_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("volumeGroupId").unwrap(),
            ),
            volume_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("volumeName").unwrap(),
            ),
        }
    }
}
