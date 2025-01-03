/// Manages an AWS Storage Gateway stored iSCSI volume.
///
/// > **NOTE:** The gateway must have a working storage added (e.g., via the `aws.storagegateway.WorkingStorage` resource) before the volume is operational to clients, however the Storage Gateway API will allow volume creation without error in that case and return volume status as `WORKING STORAGE NOT CONFIGURED`.
///
/// ## Example Usage
///
/// ### Create Empty Stored iSCSI Volume
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = stored_iscsi_volume::create(
///         "example",
///         StoredIscsiVolumeArgs::builder()
///             .disk_id("${test.id}")
///             .gateway_arn("${exampleAwsStoragegatewayCache.gatewayArn}")
///             .network_interface_id("${exampleAwsInstance.privateIp}")
///             .preserve_existing_data(false)
///             .target_name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Create Stored iSCSI Volume From Snapshot
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = stored_iscsi_volume::create(
///         "example",
///         StoredIscsiVolumeArgs::builder()
///             .disk_id("${test.id}")
///             .gateway_arn("${exampleAwsStoragegatewayCache.gatewayArn}")
///             .network_interface_id("${exampleAwsInstance.privateIp}")
///             .preserve_existing_data(false)
///             .snapshot_id("${exampleAwsEbsSnapshot.id}")
///             .target_name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_storagegateway_stored_iscsi_volume` using the volume Amazon Resource Name (ARN). For example:
///
/// ```sh
/// $ pulumi import aws:storagegateway/storedIscsiVolume:StoredIscsiVolume example arn:aws:storagegateway:us-east-1:123456789012:gateway/sgw-12345678/volume/vol-12345678
/// ```
pub mod stored_iscsi_volume {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StoredIscsiVolumeArgs {
        /// The unique identifier for the gateway local disk that is configured as a stored volume.
        #[builder(into)]
        pub disk_id: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the gateway.
        #[builder(into)]
        pub gateway_arn: pulumi_wasm_rust::Output<String>,
        /// `true` to use Amazon S3 server side encryption with your own AWS KMS key, or `false` to use a key managed by Amazon S3. Optional.
        #[builder(into, default)]
        pub kms_encrypted: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Amazon Resource Name (ARN) of the AWS KMS key used for Amazon S3 server side encryption. This value can only be set when `kms_encrypted` is `true`.
        #[builder(into, default)]
        pub kms_key: pulumi_wasm_rust::Output<Option<String>>,
        /// The network interface of the gateway on which to expose the iSCSI target. Only IPv4 addresses are accepted.
        #[builder(into)]
        pub network_interface_id: pulumi_wasm_rust::Output<String>,
        /// Specify this field as `true` if you want to preserve the data on the local disk. Otherwise, specifying this field as false creates an empty volume.
        #[builder(into)]
        pub preserve_existing_data: pulumi_wasm_rust::Output<bool>,
        /// The snapshot ID of the snapshot to restore as the new stored volumeE.g., `snap-1122aabb`.
        #[builder(into, default)]
        pub snapshot_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value mapping of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the iSCSI target used by initiators to connect to the target and as a suffix for the target ARN. The target name must be unique across all volumes of a gateway.
        #[builder(into)]
        pub target_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct StoredIscsiVolumeResult {
        /// Volume Amazon Resource Name (ARN), e.g., `arn:aws:storagegateway:us-east-1:123456789012:gateway/sgw-12345678/volume/vol-12345678`.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Whether mutual CHAP is enabled for the iSCSI target.
        pub chap_enabled: pulumi_wasm_rust::Output<bool>,
        /// The unique identifier for the gateway local disk that is configured as a stored volume.
        pub disk_id: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the gateway.
        pub gateway_arn: pulumi_wasm_rust::Output<String>,
        /// `true` to use Amazon S3 server side encryption with your own AWS KMS key, or `false` to use a key managed by Amazon S3. Optional.
        pub kms_encrypted: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Amazon Resource Name (ARN) of the AWS KMS key used for Amazon S3 server side encryption. This value can only be set when `kms_encrypted` is `true`.
        pub kms_key: pulumi_wasm_rust::Output<Option<String>>,
        /// Logical disk number.
        pub lun_number: pulumi_wasm_rust::Output<i32>,
        /// The network interface of the gateway on which to expose the iSCSI target. Only IPv4 addresses are accepted.
        pub network_interface_id: pulumi_wasm_rust::Output<String>,
        /// The port used to communicate with iSCSI targets.
        pub network_interface_port: pulumi_wasm_rust::Output<i32>,
        /// Specify this field as `true` if you want to preserve the data on the local disk. Otherwise, specifying this field as false creates an empty volume.
        pub preserve_existing_data: pulumi_wasm_rust::Output<bool>,
        /// The snapshot ID of the snapshot to restore as the new stored volumeE.g., `snap-1122aabb`.
        pub snapshot_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value mapping of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Target Amazon Resource Name (ARN), e.g., `arn:aws:storagegateway:us-east-1:123456789012:gateway/sgw-12345678/target/iqn.1997-05.com.amazon:TargetName`.
        pub target_arn: pulumi_wasm_rust::Output<String>,
        /// The name of the iSCSI target used by initiators to connect to the target and as a suffix for the target ARN. The target name must be unique across all volumes of a gateway.
        pub target_name: pulumi_wasm_rust::Output<String>,
        /// A value that indicates whether a storage volume is attached to, detached from, or is in the process of detaching from a gateway.
        pub volume_attachment_status: pulumi_wasm_rust::Output<String>,
        /// Volume ID, e.g., `vol-12345678`.
        pub volume_id: pulumi_wasm_rust::Output<String>,
        /// The size of the data stored on the volume in bytes.
        pub volume_size_in_bytes: pulumi_wasm_rust::Output<i32>,
        /// indicates the state of the storage volume.
        pub volume_status: pulumi_wasm_rust::Output<String>,
        /// indicates the type of the volume.
        pub volume_type: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: StoredIscsiVolumeArgs) -> StoredIscsiVolumeResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let disk_id_binding = args.disk_id.get_inner();
        let gateway_arn_binding = args.gateway_arn.get_inner();
        let kms_encrypted_binding = args.kms_encrypted.get_inner();
        let kms_key_binding = args.kms_key.get_inner();
        let network_interface_id_binding = args.network_interface_id.get_inner();
        let preserve_existing_data_binding = args.preserve_existing_data.get_inner();
        let snapshot_id_binding = args.snapshot_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let target_name_binding = args.target_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:storagegateway/storedIscsiVolume:StoredIscsiVolume".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "diskId".into(),
                    value: &disk_id_binding,
                },
                register_interface::ObjectField {
                    name: "gatewayArn".into(),
                    value: &gateway_arn_binding,
                },
                register_interface::ObjectField {
                    name: "kmsEncrypted".into(),
                    value: &kms_encrypted_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKey".into(),
                    value: &kms_key_binding,
                },
                register_interface::ObjectField {
                    name: "networkInterfaceId".into(),
                    value: &network_interface_id_binding,
                },
                register_interface::ObjectField {
                    name: "preserveExistingData".into(),
                    value: &preserve_existing_data_binding,
                },
                register_interface::ObjectField {
                    name: "snapshotId".into(),
                    value: &snapshot_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "targetName".into(),
                    value: &target_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "chapEnabled".into(),
                },
                register_interface::ResultField {
                    name: "diskId".into(),
                },
                register_interface::ResultField {
                    name: "gatewayArn".into(),
                },
                register_interface::ResultField {
                    name: "kmsEncrypted".into(),
                },
                register_interface::ResultField {
                    name: "kmsKey".into(),
                },
                register_interface::ResultField {
                    name: "lunNumber".into(),
                },
                register_interface::ResultField {
                    name: "networkInterfaceId".into(),
                },
                register_interface::ResultField {
                    name: "networkInterfacePort".into(),
                },
                register_interface::ResultField {
                    name: "preserveExistingData".into(),
                },
                register_interface::ResultField {
                    name: "snapshotId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "targetArn".into(),
                },
                register_interface::ResultField {
                    name: "targetName".into(),
                },
                register_interface::ResultField {
                    name: "volumeAttachmentStatus".into(),
                },
                register_interface::ResultField {
                    name: "volumeId".into(),
                },
                register_interface::ResultField {
                    name: "volumeSizeInBytes".into(),
                },
                register_interface::ResultField {
                    name: "volumeStatus".into(),
                },
                register_interface::ResultField {
                    name: "volumeType".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        StoredIscsiVolumeResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            chap_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("chapEnabled").unwrap(),
            ),
            disk_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("diskId").unwrap(),
            ),
            gateway_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gatewayArn").unwrap(),
            ),
            kms_encrypted: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsEncrypted").unwrap(),
            ),
            kms_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKey").unwrap(),
            ),
            lun_number: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lunNumber").unwrap(),
            ),
            network_interface_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkInterfaceId").unwrap(),
            ),
            network_interface_port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkInterfacePort").unwrap(),
            ),
            preserve_existing_data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("preserveExistingData").unwrap(),
            ),
            snapshot_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("snapshotId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            target_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetArn").unwrap(),
            ),
            target_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetName").unwrap(),
            ),
            volume_attachment_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("volumeAttachmentStatus").unwrap(),
            ),
            volume_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("volumeId").unwrap(),
            ),
            volume_size_in_bytes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("volumeSizeInBytes").unwrap(),
            ),
            volume_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("volumeStatus").unwrap(),
            ),
            volume_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("volumeType").unwrap(),
            ),
        }
    }
}
