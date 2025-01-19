/// Starts and stops asynchronous persistent disk replication. For more information
/// see [the official documentation](https://cloud.google.com/compute/docs/disks/async-pd/about)
/// and the [API](https://cloud.google.com/compute/docs/reference/rest/v1/disks).
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   primary-disk:
///     type: gcp:compute:Disk
///     properties:
///       name: primary-disk
///       type: pd-ssd
///       zone: europe-west4-a
///       physicalBlockSizeBytes: 4096
///   secondary-disk:
///     type: gcp:compute:Disk
///     properties:
///       name: secondary-disk
///       type: pd-ssd
///       zone: europe-west3-a
///       asyncPrimaryDisk:
///         disk: ${["primary-disk"].id}
///       physicalBlockSizeBytes: 4096
///   replication:
///     type: gcp:compute:DiskAsyncReplication
///     properties:
///       primaryDisk: ${["primary-disk"].id}
///       secondaryDisk:
///         disk: ${["secondary-disk"].id}
/// ```
pub mod disk_async_replication {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DiskAsyncReplicationArgs {
        /// The primary disk (source of replication).
        #[builder(into)]
        pub primary_disk: pulumi_wasm_rust::Output<String>,
        /// The secondary disk (target of replication). You can specify only one value. Structure is documented below.
        ///
        /// The `secondary_disk` block includes:
        #[builder(into)]
        pub secondary_disk: pulumi_wasm_rust::Output<
            super::super::types::compute::DiskAsyncReplicationSecondaryDisk,
        >,
    }
    #[allow(dead_code)]
    pub struct DiskAsyncReplicationResult {
        /// The primary disk (source of replication).
        pub primary_disk: pulumi_wasm_rust::Output<String>,
        /// The secondary disk (target of replication). You can specify only one value. Structure is documented below.
        ///
        /// The `secondary_disk` block includes:
        pub secondary_disk: pulumi_wasm_rust::Output<
            super::super::types::compute::DiskAsyncReplicationSecondaryDisk,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: DiskAsyncReplicationArgs,
    ) -> DiskAsyncReplicationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let primary_disk_binding = args.primary_disk.get_inner();
        let secondary_disk_binding = args.secondary_disk.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/diskAsyncReplication:DiskAsyncReplication".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "primaryDisk".into(),
                    value: &primary_disk_binding,
                },
                register_interface::ObjectField {
                    name: "secondaryDisk".into(),
                    value: &secondary_disk_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "primaryDisk".into(),
                },
                register_interface::ResultField {
                    name: "secondaryDisk".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DiskAsyncReplicationResult {
            primary_disk: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryDisk").unwrap(),
            ),
            secondary_disk: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryDisk").unwrap(),
            ),
        }
    }
}
