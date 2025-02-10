/// Manages an AWS Storage Gateway cached iSCSI volume.
///
/// > **NOTE:** The gateway must have cache added (e.g., via the `aws.storagegateway.Cache` resource) before creating volumes otherwise the Storage Gateway API will return an error.
///
/// > **NOTE:** The gateway must have an upload buffer added (e.g., via the `aws.storagegateway.UploadBuffer` resource) before the volume is operational to clients, however the Storage Gateway API will allow volume creation without error in that case and return volume status as `UPLOAD BUFFER NOT CONFIGURED`.
///
/// ## Example Usage
///
/// > **NOTE:** These examples are referencing the `aws.storagegateway.Cache` resource `gateway_arn` attribute to ensure this provider properly adds cache before creating the volume. If you are not using this method, you may need to declare an expicit dependency (e.g. via `depends_on = [aws_storagegateway_cache.example]`) to ensure proper ordering.
///
/// ### Create Empty Cached iSCSI Volume
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = caches_iscsi_volume::create(
///         "example",
///         CachesIscsiVolumeArgs::builder()
///             .gateway_arn("${exampleAwsStoragegatewayCache.gatewayArn}")
///             .network_interface_id("${exampleAwsInstance.privateIp}")
///             .target_name("example")
///             .volume_size_in_bytes(5368709120)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Create Cached iSCSI Volume From Snapshot
///
///
/// ### Create Cached iSCSI Volume From Source Volume
///
/// ```yaml
/// resources:
///   example:
///     type: aws:storagegateway:CachesIscsiVolume
///     properties:
///       gatewayArn: ${exampleAwsStoragegatewayCache.gatewayArn}
///       networkInterfaceId: ${exampleAwsInstance.privateIp}
///       sourceVolumeArn: ${existing.arn}
///       targetName: example
///       volumeSizeInBytes: ${existing.volumeSizeInBytes}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_storagegateway_cached_iscsi_volume` using the volume Amazon Resource Name (ARN). For example:
///
/// ```sh
/// $ pulumi import aws:storagegateway/cachesIscsiVolume:CachesIscsiVolume example arn:aws:storagegateway:us-east-1:123456789012:gateway/sgw-12345678/volume/vol-12345678
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod caches_iscsi_volume {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CachesIscsiVolumeArgs {
        /// The Amazon Resource Name (ARN) of the gateway.
        #[builder(into)]
        pub gateway_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Set to `true` to use Amazon S3 server side encryption with your own AWS KMS key, or `false` to use a key managed by Amazon S3.
        #[builder(into, default)]
        pub kms_encrypted: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The Amazon Resource Name (ARN) of the AWS KMS key used for Amazon S3 server side encryption. Is required when `kms_encrypted` is set.
        #[builder(into, default)]
        pub kms_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The network interface of the gateway on which to expose the iSCSI target. Only IPv4 addresses are accepted.
        #[builder(into)]
        pub network_interface_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The snapshot ID of the snapshot to restore as the new cached volumeE.g., `snap-1122aabb`.
        #[builder(into, default)]
        pub snapshot_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ARN for an existing volume. Specifying this ARN makes the new volume into an exact copy of the specified existing volume's latest recovery point. The `volume_size_in_bytes` value for this new volume must be equal to or larger than the size of the existing volume, in bytes.
        #[builder(into, default)]
        pub source_volume_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the iSCSI target used by initiators to connect to the target and as a suffix for the target ARN. The target name must be unique across all volumes of a gateway.
        #[builder(into)]
        pub target_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The size of the volume in bytes.
        #[builder(into)]
        pub volume_size_in_bytes: pulumi_gestalt_rust::InputOrOutput<i32>,
    }
    #[allow(dead_code)]
    pub struct CachesIscsiVolumeResult {
        /// Volume Amazon Resource Name (ARN), e.g., `arn:aws:storagegateway:us-east-1:123456789012:gateway/sgw-12345678/volume/vol-12345678`.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Whether mutual CHAP is enabled for the iSCSI target.
        pub chap_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The Amazon Resource Name (ARN) of the gateway.
        pub gateway_arn: pulumi_gestalt_rust::Output<String>,
        /// Set to `true` to use Amazon S3 server side encryption with your own AWS KMS key, or `false` to use a key managed by Amazon S3.
        pub kms_encrypted: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The Amazon Resource Name (ARN) of the AWS KMS key used for Amazon S3 server side encryption. Is required when `kms_encrypted` is set.
        pub kms_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// Logical disk number.
        pub lun_number: pulumi_gestalt_rust::Output<i32>,
        /// The network interface of the gateway on which to expose the iSCSI target. Only IPv4 addresses are accepted.
        pub network_interface_id: pulumi_gestalt_rust::Output<String>,
        /// The port used to communicate with iSCSI targets.
        pub network_interface_port: pulumi_gestalt_rust::Output<i32>,
        /// The snapshot ID of the snapshot to restore as the new cached volumeE.g., `snap-1122aabb`.
        pub snapshot_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ARN for an existing volume. Specifying this ARN makes the new volume into an exact copy of the specified existing volume's latest recovery point. The `volume_size_in_bytes` value for this new volume must be equal to or larger than the size of the existing volume, in bytes.
        pub source_volume_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
        /// Volume Amazon Resource Name (ARN), e.g., `arn:aws:storagegateway:us-east-1:123456789012:gateway/sgw-12345678/volume/vol-12345678`.
        pub volume_arn: pulumi_gestalt_rust::Output<String>,
        /// Volume ID, e.g., `vol-12345678`.
        pub volume_id: pulumi_gestalt_rust::Output<String>,
        /// The size of the volume in bytes.
        pub volume_size_in_bytes: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CachesIscsiVolumeArgs,
    ) -> CachesIscsiVolumeResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let gateway_arn_binding = args.gateway_arn.get_output(context);
        let kms_encrypted_binding = args.kms_encrypted.get_output(context);
        let kms_key_binding = args.kms_key.get_output(context);
        let network_interface_id_binding = args.network_interface_id.get_output(context);
        let snapshot_id_binding = args.snapshot_id.get_output(context);
        let source_volume_arn_binding = args.source_volume_arn.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let target_name_binding = args.target_name.get_output(context);
        let volume_size_in_bytes_binding = args.volume_size_in_bytes.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:storagegateway/cachesIscsiVolume:CachesIscsiVolume".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gatewayArn".into(),
                    value: gateway_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsEncrypted".into(),
                    value: kms_encrypted_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsKey".into(),
                    value: kms_key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkInterfaceId".into(),
                    value: network_interface_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "snapshotId".into(),
                    value: snapshot_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceVolumeArn".into(),
                    value: source_volume_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetName".into(),
                    value: target_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "volumeSizeInBytes".into(),
                    value: volume_size_in_bytes_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        CachesIscsiVolumeResult {
            arn: o.get_field("arn"),
            chap_enabled: o.get_field("chapEnabled"),
            gateway_arn: o.get_field("gatewayArn"),
            kms_encrypted: o.get_field("kmsEncrypted"),
            kms_key: o.get_field("kmsKey"),
            lun_number: o.get_field("lunNumber"),
            network_interface_id: o.get_field("networkInterfaceId"),
            network_interface_port: o.get_field("networkInterfacePort"),
            snapshot_id: o.get_field("snapshotId"),
            source_volume_arn: o.get_field("sourceVolumeArn"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            target_arn: o.get_field("targetArn"),
            target_name: o.get_field("targetName"),
            volume_arn: o.get_field("volumeArn"),
            volume_id: o.get_field("volumeId"),
            volume_size_in_bytes: o.get_field("volumeSizeInBytes"),
        }
    }
}
