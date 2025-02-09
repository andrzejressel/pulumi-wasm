/// Manages an AWS Storage Gateway NFS File Share.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = nfs_file_share::create(
///         "example",
///         NfsFileShareArgs::builder()
///             .client_lists(vec!["0.0.0.0/0",])
///             .gateway_arn("${exampleAwsStoragegatewayGateway.arn}")
///             .location_arn("${exampleAwsS3Bucket.arn}")
///             .role_arn("${exampleAwsIamRole.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_storagegateway_nfs_file_share` using the NFS File Share Amazon Resource Name (ARN). For example:
///
/// ```sh
/// $ pulumi import aws:storagegateway/nfsFileShare:NfsFileShare example arn:aws:storagegateway:us-east-1:123456789012:share/share-12345678
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod nfs_file_share {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NfsFileShareArgs {
        /// The Amazon Resource Name (ARN) of the storage used for audit logs.
        #[builder(into, default)]
        pub audit_destination_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The region of the S3 bucket used by the file share. Required when specifying `vpc_endpoint_dns_name`.
        #[builder(into, default)]
        pub bucket_region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Refresh cache information. see Cache Attributes for more details.
        #[builder(into, default)]
        pub cache_attributes: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::storagegateway::NfsFileShareCacheAttributes>,
        >,
        /// The list of clients that are allowed to access the file gateway. The list must contain either valid IP addresses or valid CIDR blocks. Set to `["0.0.0.0/0"]` to not limit access. Minimum 1 item. Maximum 100 items.
        #[builder(into)]
        pub client_lists: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The default [storage class](https://docs.aws.amazon.com/storagegateway/latest/APIReference/API_CreateNFSFileShare.html#StorageGateway-CreateNFSFileShare-request-DefaultStorageClass) for objects put into an Amazon S3 bucket by the file gateway. Defaults to `S3_STANDARD`.
        #[builder(into, default)]
        pub default_storage_class: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the file share. Must be set if an S3 prefix name is set in `location_arn`.
        #[builder(into, default)]
        pub file_share_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Amazon Resource Name (ARN) of the file gateway.
        #[builder(into)]
        pub gateway_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Boolean value that enables guessing of the MIME type for uploaded objects based on file extensions. Defaults to `true`.
        #[builder(into, default)]
        pub guess_mime_type_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Boolean value if `true` to use Amazon S3 server side encryption with your own AWS KMS key, or `false` to use a key managed by Amazon S3. Defaults to `false`.
        #[builder(into, default)]
        pub kms_encrypted: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Amazon Resource Name (ARN) for KMS key used for Amazon S3 server side encryption. This value can only be set when `kms_encrypted` is true.
        #[builder(into, default)]
        pub kms_key_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ARN of the backed storage used for storing file data.
        #[builder(into)]
        pub location_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Nested argument with file share default values. More information below. see NFS File Share Defaults for more details.
        #[builder(into, default)]
        pub nfs_file_share_defaults: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::storagegateway::NfsFileShareNfsFileShareDefaults>,
        >,
        /// The notification policy of the file share. For more information see the [AWS Documentation](https://docs.aws.amazon.com/storagegateway/latest/APIReference/API_CreateNFSFileShare.html#StorageGateway-CreateNFSFileShare-request-NotificationPolicy). Default value is `{}`.
        #[builder(into, default)]
        pub notification_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Access Control List permission for S3 objects. Defaults to `private`.
        #[builder(into, default)]
        pub object_acl: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Boolean to indicate write status of file share. File share does not accept writes if `true`. Defaults to `false`.
        #[builder(into, default)]
        pub read_only: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Boolean who pays the cost of the request and the data download from the Amazon S3 bucket. Set this value to `true` if you want the requester to pay instead of the bucket owner. Defaults to `false`.
        #[builder(into, default)]
        pub requester_pays: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The ARN of the AWS Identity and Access Management (IAM) role that a file gateway assumes when it accesses the underlying storage.
        #[builder(into)]
        pub role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Maps a user to anonymous user. Defaults to `RootSquash`. Valid values: `RootSquash` (only root is mapped to anonymous user), `NoSquash` (no one is mapped to anonymous user), `AllSquash` (everyone is mapped to anonymous user)
        #[builder(into, default)]
        pub squash: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The DNS name of the VPC endpoint for S3 PrivateLink.
        #[builder(into, default)]
        pub vpc_endpoint_dns_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct NfsFileShareResult {
        /// Amazon Resource Name (ARN) of the NFS File Share.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the storage used for audit logs.
        pub audit_destination_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// The region of the S3 bucket used by the file share. Required when specifying `vpc_endpoint_dns_name`.
        pub bucket_region: pulumi_gestalt_rust::Output<Option<String>>,
        /// Refresh cache information. see Cache Attributes for more details.
        pub cache_attributes: pulumi_gestalt_rust::Output<
            Option<super::super::types::storagegateway::NfsFileShareCacheAttributes>,
        >,
        /// The list of clients that are allowed to access the file gateway. The list must contain either valid IP addresses or valid CIDR blocks. Set to `["0.0.0.0/0"]` to not limit access. Minimum 1 item. Maximum 100 items.
        pub client_lists: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The default [storage class](https://docs.aws.amazon.com/storagegateway/latest/APIReference/API_CreateNFSFileShare.html#StorageGateway-CreateNFSFileShare-request-DefaultStorageClass) for objects put into an Amazon S3 bucket by the file gateway. Defaults to `S3_STANDARD`.
        pub default_storage_class: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the file share. Must be set if an S3 prefix name is set in `location_arn`.
        pub file_share_name: pulumi_gestalt_rust::Output<String>,
        /// ID of the NFS File Share.
        pub fileshare_id: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the file gateway.
        pub gateway_arn: pulumi_gestalt_rust::Output<String>,
        /// Boolean value that enables guessing of the MIME type for uploaded objects based on file extensions. Defaults to `true`.
        pub guess_mime_type_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Boolean value if `true` to use Amazon S3 server side encryption with your own AWS KMS key, or `false` to use a key managed by Amazon S3. Defaults to `false`.
        pub kms_encrypted: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Amazon Resource Name (ARN) for KMS key used for Amazon S3 server side encryption. This value can only be set when `kms_encrypted` is true.
        pub kms_key_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ARN of the backed storage used for storing file data.
        pub location_arn: pulumi_gestalt_rust::Output<String>,
        /// Nested argument with file share default values. More information below. see NFS File Share Defaults for more details.
        pub nfs_file_share_defaults: pulumi_gestalt_rust::Output<
            Option<super::super::types::storagegateway::NfsFileShareNfsFileShareDefaults>,
        >,
        /// The notification policy of the file share. For more information see the [AWS Documentation](https://docs.aws.amazon.com/storagegateway/latest/APIReference/API_CreateNFSFileShare.html#StorageGateway-CreateNFSFileShare-request-NotificationPolicy). Default value is `{}`.
        pub notification_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// Access Control List permission for S3 objects. Defaults to `private`.
        pub object_acl: pulumi_gestalt_rust::Output<Option<String>>,
        /// File share path used by the NFS client to identify the mount point.
        pub path: pulumi_gestalt_rust::Output<String>,
        /// Boolean to indicate write status of file share. File share does not accept writes if `true`. Defaults to `false`.
        pub read_only: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Boolean who pays the cost of the request and the data download from the Amazon S3 bucket. Set this value to `true` if you want the requester to pay instead of the bucket owner. Defaults to `false`.
        pub requester_pays: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The ARN of the AWS Identity and Access Management (IAM) role that a file gateway assumes when it accesses the underlying storage.
        pub role_arn: pulumi_gestalt_rust::Output<String>,
        /// Maps a user to anonymous user. Defaults to `RootSquash`. Valid values: `RootSquash` (only root is mapped to anonymous user), `NoSquash` (no one is mapped to anonymous user), `AllSquash` (everyone is mapped to anonymous user)
        pub squash: pulumi_gestalt_rust::Output<Option<String>>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The DNS name of the VPC endpoint for S3 PrivateLink.
        pub vpc_endpoint_dns_name: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: NfsFileShareArgs,
    ) -> NfsFileShareResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let audit_destination_arn_binding_1 = args
            .audit_destination_arn
            .get_output(context);
        let audit_destination_arn_binding = audit_destination_arn_binding_1.get_inner();
        let bucket_region_binding_1 = args.bucket_region.get_output(context);
        let bucket_region_binding = bucket_region_binding_1.get_inner();
        let cache_attributes_binding_1 = args.cache_attributes.get_output(context);
        let cache_attributes_binding = cache_attributes_binding_1.get_inner();
        let client_lists_binding_1 = args.client_lists.get_output(context);
        let client_lists_binding = client_lists_binding_1.get_inner();
        let default_storage_class_binding_1 = args
            .default_storage_class
            .get_output(context);
        let default_storage_class_binding = default_storage_class_binding_1.get_inner();
        let file_share_name_binding_1 = args.file_share_name.get_output(context);
        let file_share_name_binding = file_share_name_binding_1.get_inner();
        let gateway_arn_binding_1 = args.gateway_arn.get_output(context);
        let gateway_arn_binding = gateway_arn_binding_1.get_inner();
        let guess_mime_type_enabled_binding_1 = args
            .guess_mime_type_enabled
            .get_output(context);
        let guess_mime_type_enabled_binding = guess_mime_type_enabled_binding_1
            .get_inner();
        let kms_encrypted_binding_1 = args.kms_encrypted.get_output(context);
        let kms_encrypted_binding = kms_encrypted_binding_1.get_inner();
        let kms_key_arn_binding_1 = args.kms_key_arn.get_output(context);
        let kms_key_arn_binding = kms_key_arn_binding_1.get_inner();
        let location_arn_binding_1 = args.location_arn.get_output(context);
        let location_arn_binding = location_arn_binding_1.get_inner();
        let nfs_file_share_defaults_binding_1 = args
            .nfs_file_share_defaults
            .get_output(context);
        let nfs_file_share_defaults_binding = nfs_file_share_defaults_binding_1
            .get_inner();
        let notification_policy_binding_1 = args.notification_policy.get_output(context);
        let notification_policy_binding = notification_policy_binding_1.get_inner();
        let object_acl_binding_1 = args.object_acl.get_output(context);
        let object_acl_binding = object_acl_binding_1.get_inner();
        let read_only_binding_1 = args.read_only.get_output(context);
        let read_only_binding = read_only_binding_1.get_inner();
        let requester_pays_binding_1 = args.requester_pays.get_output(context);
        let requester_pays_binding = requester_pays_binding_1.get_inner();
        let role_arn_binding_1 = args.role_arn.get_output(context);
        let role_arn_binding = role_arn_binding_1.get_inner();
        let squash_binding_1 = args.squash.get_output(context);
        let squash_binding = squash_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let vpc_endpoint_dns_name_binding_1 = args
            .vpc_endpoint_dns_name
            .get_output(context);
        let vpc_endpoint_dns_name_binding = vpc_endpoint_dns_name_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:storagegateway/nfsFileShare:NfsFileShare".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "auditDestinationArn".into(),
                    value: &audit_destination_arn_binding,
                },
                register_interface::ObjectField {
                    name: "bucketRegion".into(),
                    value: &bucket_region_binding,
                },
                register_interface::ObjectField {
                    name: "cacheAttributes".into(),
                    value: &cache_attributes_binding,
                },
                register_interface::ObjectField {
                    name: "clientLists".into(),
                    value: &client_lists_binding,
                },
                register_interface::ObjectField {
                    name: "defaultStorageClass".into(),
                    value: &default_storage_class_binding,
                },
                register_interface::ObjectField {
                    name: "fileShareName".into(),
                    value: &file_share_name_binding,
                },
                register_interface::ObjectField {
                    name: "gatewayArn".into(),
                    value: &gateway_arn_binding,
                },
                register_interface::ObjectField {
                    name: "guessMimeTypeEnabled".into(),
                    value: &guess_mime_type_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "kmsEncrypted".into(),
                    value: &kms_encrypted_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyArn".into(),
                    value: &kms_key_arn_binding,
                },
                register_interface::ObjectField {
                    name: "locationArn".into(),
                    value: &location_arn_binding,
                },
                register_interface::ObjectField {
                    name: "nfsFileShareDefaults".into(),
                    value: &nfs_file_share_defaults_binding,
                },
                register_interface::ObjectField {
                    name: "notificationPolicy".into(),
                    value: &notification_policy_binding,
                },
                register_interface::ObjectField {
                    name: "objectAcl".into(),
                    value: &object_acl_binding,
                },
                register_interface::ObjectField {
                    name: "readOnly".into(),
                    value: &read_only_binding,
                },
                register_interface::ObjectField {
                    name: "requesterPays".into(),
                    value: &requester_pays_binding,
                },
                register_interface::ObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "squash".into(),
                    value: &squash_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "vpcEndpointDnsName".into(),
                    value: &vpc_endpoint_dns_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        NfsFileShareResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            audit_destination_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("auditDestinationArn"),
            ),
            bucket_region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bucketRegion"),
            ),
            cache_attributes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cacheAttributes"),
            ),
            client_lists: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clientLists"),
            ),
            default_storage_class: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultStorageClass"),
            ),
            file_share_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fileShareName"),
            ),
            fileshare_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fileshareId"),
            ),
            gateway_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("gatewayArn"),
            ),
            guess_mime_type_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("guessMimeTypeEnabled"),
            ),
            kms_encrypted: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsEncrypted"),
            ),
            kms_key_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsKeyArn"),
            ),
            location_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("locationArn"),
            ),
            nfs_file_share_defaults: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nfsFileShareDefaults"),
            ),
            notification_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("notificationPolicy"),
            ),
            object_acl: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("objectAcl"),
            ),
            path: pulumi_gestalt_rust::__private::into_domain(o.extract_field("path")),
            read_only: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("readOnly"),
            ),
            requester_pays: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("requesterPays"),
            ),
            role_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("roleArn"),
            ),
            squash: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("squash"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            vpc_endpoint_dns_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vpcEndpointDnsName"),
            ),
        }
    }
}
