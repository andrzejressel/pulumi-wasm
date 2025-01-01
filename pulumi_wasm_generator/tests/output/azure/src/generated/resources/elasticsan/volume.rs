/// Manages an Elastic SAN Volume resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-rg
///       location: West Europe
///   exampleElasticSan:
///     type: azure:elasticsan:ElasticSan
///     name: example
///     properties:
///       name: example-es
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       baseSizeInTib: 1
///       sku:
///         name: Premium_LRS
///   exampleVolumeGroup:
///     type: azure:elasticsan:VolumeGroup
///     name: example
///     properties:
///       name: example-esvg
///       elasticSanId: ${exampleElasticSan.id}
///   exampleVolume:
///     type: azure:elasticsan:Volume
///     name: example
///     properties:
///       name: example-esv
///       volumeGroupId: ${exampleVolumeGroup.id}
///       sizeInGib: 1
/// outputs:
///   targetIqn: ${exampleVolume.targetIqn}
/// ```
///
/// ## Example of creating an Elastic SAN Volume from a Disk Snapshot
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-rg
///       location: West Europe
///   exampleElasticSan:
///     type: azure:elasticsan:ElasticSan
///     name: example
///     properties:
///       name: example-es
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       baseSizeInTib: 1
///       sku:
///         name: Premium_LRS
///   exampleVolumeGroup:
///     type: azure:elasticsan:VolumeGroup
///     name: example
///     properties:
///       name: example-esvg
///       elasticSanId: ${exampleElasticSan.id}
///   exampleManagedDisk:
///     type: azure:compute:ManagedDisk
///     name: example
///     properties:
///       name: example-disk
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       createOption: Empty
///       storageAccountType: Standard_LRS
///       diskSizeGb: 2
///   exampleSnapshot:
///     type: azure:compute:Snapshot
///     name: example
///     properties:
///       name: example-ss
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       createOption: Copy
///       sourceUri: ${exampleManagedDisk.id}
///   example2:
///     type: azure:elasticsan:Volume
///     properties:
///       name: example-esv2
///       volumeGroupId: ${exampleVolumeGroup.id}
///       sizeInGib: 2
///       createSource:
///         sourceType: DiskSnapshot
///         sourceId: ${exampleSnapshot.id}
/// ```
///
/// ## Import
///
/// An existing Elastic SAN Volume can be imported into Pulumi using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:elasticsan/volume:Volume example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/rg1/providers/Microsoft.ElasticSan/elasticSans/esan1/volumeGroups/vg1/volumes/vol1
/// ```
///
pub mod volume {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VolumeArgs {
        /// A `create_source` block as defined below.
        #[builder(into, default)]
        pub create_source: pulumi_wasm_rust::Output<
            Option<super::super::types::elasticsan::VolumeCreateSource>,
        >,
        /// Specifies the name of this Elastic SAN Volume. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the size of the Elastic SAN Volume in GiB. The size should be within the remaining capacity of the parent Elastic SAN. Possible values are between `1` and `65536` (16 TiB).
        ///
        /// > **NOTE:** The size can only be increased. If `create_source` is specified, then the size must be equal to or greater than the source's size.
        #[builder(into)]
        pub size_in_gib: pulumi_wasm_rust::Output<i32>,
        /// Specifies the Volume Group ID within which this Elastic SAN Volume should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub volume_group_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct VolumeResult {
        /// A `create_source` block as defined below.
        pub create_source: pulumi_wasm_rust::Output<
            Option<super::super::types::elasticsan::VolumeCreateSource>,
        >,
        /// Specifies the name of this Elastic SAN Volume. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the size of the Elastic SAN Volume in GiB. The size should be within the remaining capacity of the parent Elastic SAN. Possible values are between `1` and `65536` (16 TiB).
        ///
        /// > **NOTE:** The size can only be increased. If `create_source` is specified, then the size must be equal to or greater than the source's size.
        pub size_in_gib: pulumi_wasm_rust::Output<i32>,
        /// The iSCSI Target IQN of the Elastic SAN Volume.
        pub target_iqn: pulumi_wasm_rust::Output<String>,
        /// The iSCSI Target Portal Host Name of the Elastic SAN Volume.
        pub target_portal_hostname: pulumi_wasm_rust::Output<String>,
        /// The iSCSI Target Portal Port of the Elastic SAN Volume.
        pub target_portal_port: pulumi_wasm_rust::Output<i32>,
        /// Specifies the Volume Group ID within which this Elastic SAN Volume should exist. Changing this forces a new resource to be created.
        pub volume_group_id: pulumi_wasm_rust::Output<String>,
        /// The UUID of the Elastic SAN Volume.
        pub volume_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: VolumeArgs) -> VolumeResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let create_source_binding = args.create_source.get_inner();
        let name_binding = args.name.get_inner();
        let size_in_gib_binding = args.size_in_gib.get_inner();
        let volume_group_id_binding = args.volume_group_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:elasticsan/volume:Volume".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "createSource".into(),
                    value: &create_source_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "sizeInGib".into(),
                    value: &size_in_gib_binding,
                },
                register_interface::ObjectField {
                    name: "volumeGroupId".into(),
                    value: &volume_group_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "createSource".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "sizeInGib".into(),
                },
                register_interface::ResultField {
                    name: "targetIqn".into(),
                },
                register_interface::ResultField {
                    name: "targetPortalHostname".into(),
                },
                register_interface::ResultField {
                    name: "targetPortalPort".into(),
                },
                register_interface::ResultField {
                    name: "volumeGroupId".into(),
                },
                register_interface::ResultField {
                    name: "volumeId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VolumeResult {
            create_source: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createSource").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            size_in_gib: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sizeInGib").unwrap(),
            ),
            target_iqn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetIqn").unwrap(),
            ),
            target_portal_hostname: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetPortalHostname").unwrap(),
            ),
            target_portal_port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetPortalPort").unwrap(),
            ),
            volume_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("volumeGroupId").unwrap(),
            ),
            volume_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("volumeId").unwrap(),
            ),
        }
    }
}
