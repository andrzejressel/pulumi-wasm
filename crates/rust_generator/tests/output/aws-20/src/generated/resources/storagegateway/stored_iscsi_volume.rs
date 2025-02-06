/// Manages an AWS Storage Gateway stored iSCSI volume.
///
/// > **NOTE:** The gateway must have a working storage added (e.g., via the `aws.storagegateway.WorkingStorage` resource) before the volume is operational to clients, however the Storage Gateway API will allow volume creation without error in that case and return volume status as `WORKING STORAGE NOT CONFIGURED`.
///
/// ## Example Usage
///
/// ### Create Empty Stored iSCSI Volume
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StoredIscsiVolumeArgs {
        /// The unique identifier for the gateway local disk that is configured as a stored volume.
        #[builder(into)]
        pub disk_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Amazon Resource Name (ARN) of the gateway.
        #[builder(into)]
        pub gateway_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// `true` to use Amazon S3 server side encryption with your own AWS KMS key, or `false` to use a key managed by Amazon S3. Optional.
        #[builder(into, default)]
        pub kms_encrypted: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The Amazon Resource Name (ARN) of the AWS KMS key used for Amazon S3 server side encryption. This value can only be set when `kms_encrypted` is `true`.
        #[builder(into, default)]
        pub kms_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The network interface of the gateway on which to expose the iSCSI target. Only IPv4 addresses are accepted.
        #[builder(into)]
        pub network_interface_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specify this field as `true` if you want to preserve the data on the local disk. Otherwise, specifying this field as false creates an empty volume.
        #[builder(into)]
        pub preserve_existing_data: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// The snapshot ID of the snapshot to restore as the new stored volumeE.g., `snap-1122aabb`.
        #[builder(into, default)]
        pub snapshot_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value mapping of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the iSCSI target used by initiators to connect to the target and as a suffix for the target ARN. The target name must be unique across all volumes of a gateway.
        #[builder(into)]
        pub target_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct StoredIscsiVolumeResult {
        /// Volume Amazon Resource Name (ARN), e.g., `arn:aws:storagegateway:us-east-1:123456789012:gateway/sgw-12345678/volume/vol-12345678`.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Whether mutual CHAP is enabled for the iSCSI target.
        pub chap_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The unique identifier for the gateway local disk that is configured as a stored volume.
        pub disk_id: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the gateway.
        pub gateway_arn: pulumi_gestalt_rust::Output<String>,
        /// `true` to use Amazon S3 server side encryption with your own AWS KMS key, or `false` to use a key managed by Amazon S3. Optional.
        pub kms_encrypted: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The Amazon Resource Name (ARN) of the AWS KMS key used for Amazon S3 server side encryption. This value can only be set when `kms_encrypted` is `true`.
        pub kms_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// Logical disk number.
        pub lun_number: pulumi_gestalt_rust::Output<i32>,
        /// The network interface of the gateway on which to expose the iSCSI target. Only IPv4 addresses are accepted.
        pub network_interface_id: pulumi_gestalt_rust::Output<String>,
        /// The port used to communicate with iSCSI targets.
        pub network_interface_port: pulumi_gestalt_rust::Output<i32>,
        /// Specify this field as `true` if you want to preserve the data on the local disk. Otherwise, specifying this field as false creates an empty volume.
        pub preserve_existing_data: pulumi_gestalt_rust::Output<bool>,
        /// The snapshot ID of the snapshot to restore as the new stored volumeE.g., `snap-1122aabb`.
        pub snapshot_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Key-value mapping of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Target Amazon Resource Name (ARN), e.g., `arn:aws:storagegateway:us-east-1:123456789012:gateway/sgw-12345678/target/iqn.1997-05.com.amazon:TargetName`.
        pub target_arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the iSCSI target used by initiators to connect to the target and as a suffix for the target ARN. The target name must be unique across all volumes of a gateway.
        pub target_name: pulumi_gestalt_rust::Output<String>,
        /// A value that indicates whether a storage volume is attached to, detached from, or is in the process of detaching from a gateway.
        pub volume_attachment_status: pulumi_gestalt_rust::Output<String>,
        /// Volume ID, e.g., `vol-12345678`.
        pub volume_id: pulumi_gestalt_rust::Output<String>,
        /// The size of the data stored on the volume in bytes.
        pub volume_size_in_bytes: pulumi_gestalt_rust::Output<i32>,
        /// indicates the state of the storage volume.
        pub volume_status: pulumi_gestalt_rust::Output<String>,
        /// indicates the type of the volume.
        pub volume_type: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: StoredIscsiVolumeArgs,
    ) -> StoredIscsiVolumeResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let disk_id_binding = args.disk_id.get_output(context).get_inner();
        let gateway_arn_binding = args.gateway_arn.get_output(context).get_inner();
        let kms_encrypted_binding = args.kms_encrypted.get_output(context).get_inner();
        let kms_key_binding = args.kms_key.get_output(context).get_inner();
        let network_interface_id_binding = args
            .network_interface_id
            .get_output(context)
            .get_inner();
        let preserve_existing_data_binding = args
            .preserve_existing_data
            .get_output(context)
            .get_inner();
        let snapshot_id_binding = args.snapshot_id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let target_name_binding = args.target_name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:storagegateway/storedIscsiVolume:StoredIscsiVolume".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        StoredIscsiVolumeResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            chap_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("chapEnabled"),
            ),
            disk_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("diskId"),
            ),
            gateway_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("gatewayArn"),
            ),
            kms_encrypted: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsEncrypted"),
            ),
            kms_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsKey"),
            ),
            lun_number: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lunNumber"),
            ),
            network_interface_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkInterfaceId"),
            ),
            network_interface_port: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkInterfacePort"),
            ),
            preserve_existing_data: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("preserveExistingData"),
            ),
            snapshot_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("snapshotId"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            target_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("targetArn"),
            ),
            target_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("targetName"),
            ),
            volume_attachment_status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("volumeAttachmentStatus"),
            ),
            volume_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("volumeId"),
            ),
            volume_size_in_bytes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("volumeSizeInBytes"),
            ),
            volume_status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("volumeStatus"),
            ),
            volume_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("volumeType"),
            ),
        }
    }
}
