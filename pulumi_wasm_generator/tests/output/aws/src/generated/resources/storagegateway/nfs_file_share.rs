/// Manages an AWS Storage Gateway NFS File Share.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod nfs_file_share {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NfsFileShareArgs {
        /// The Amazon Resource Name (ARN) of the storage used for audit logs.
        #[builder(into, default)]
        pub audit_destination_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The region of the S3 bucket used by the file share. Required when specifying `vpc_endpoint_dns_name`.
        #[builder(into, default)]
        pub bucket_region: pulumi_wasm_rust::Output<Option<String>>,
        /// Refresh cache information. see Cache Attributes for more details.
        #[builder(into, default)]
        pub cache_attributes: pulumi_wasm_rust::Output<
            Option<super::super::types::storagegateway::NfsFileShareCacheAttributes>,
        >,
        /// The list of clients that are allowed to access the file gateway. The list must contain either valid IP addresses or valid CIDR blocks. Set to `["0.0.0.0/0"]` to not limit access. Minimum 1 item. Maximum 100 items.
        #[builder(into)]
        pub client_lists: pulumi_wasm_rust::Output<Vec<String>>,
        /// The default [storage class](https://docs.aws.amazon.com/storagegateway/latest/APIReference/API_CreateNFSFileShare.html#StorageGateway-CreateNFSFileShare-request-DefaultStorageClass) for objects put into an Amazon S3 bucket by the file gateway. Defaults to `S3_STANDARD`.
        #[builder(into, default)]
        pub default_storage_class: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the file share. Must be set if an S3 prefix name is set in `location_arn`.
        #[builder(into, default)]
        pub file_share_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Amazon Resource Name (ARN) of the file gateway.
        #[builder(into)]
        pub gateway_arn: pulumi_wasm_rust::Output<String>,
        /// Boolean value that enables guessing of the MIME type for uploaded objects based on file extensions. Defaults to `true`.
        #[builder(into, default)]
        pub guess_mime_type_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Boolean value if `true` to use Amazon S3 server side encryption with your own AWS KMS key, or `false` to use a key managed by Amazon S3. Defaults to `false`.
        #[builder(into, default)]
        pub kms_encrypted: pulumi_wasm_rust::Output<Option<bool>>,
        /// Amazon Resource Name (ARN) for KMS key used for Amazon S3 server side encryption. This value can only be set when `kms_encrypted` is true.
        #[builder(into, default)]
        pub kms_key_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The ARN of the backed storage used for storing file data.
        #[builder(into)]
        pub location_arn: pulumi_wasm_rust::Output<String>,
        /// Nested argument with file share default values. More information below. see NFS File Share Defaults for more details.
        #[builder(into, default)]
        pub nfs_file_share_defaults: pulumi_wasm_rust::Output<
            Option<super::super::types::storagegateway::NfsFileShareNfsFileShareDefaults>,
        >,
        /// The notification policy of the file share. For more information see the [AWS Documentation](https://docs.aws.amazon.com/storagegateway/latest/APIReference/API_CreateNFSFileShare.html#StorageGateway-CreateNFSFileShare-request-NotificationPolicy). Default value is `{}`.
        #[builder(into, default)]
        pub notification_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// Access Control List permission for S3 objects. Defaults to `private`.
        #[builder(into, default)]
        pub object_acl: pulumi_wasm_rust::Output<Option<String>>,
        /// Boolean to indicate write status of file share. File share does not accept writes if `true`. Defaults to `false`.
        #[builder(into, default)]
        pub read_only: pulumi_wasm_rust::Output<Option<bool>>,
        /// Boolean who pays the cost of the request and the data download from the Amazon S3 bucket. Set this value to `true` if you want the requester to pay instead of the bucket owner. Defaults to `false`.
        #[builder(into, default)]
        pub requester_pays: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ARN of the AWS Identity and Access Management (IAM) role that a file gateway assumes when it accesses the underlying storage.
        #[builder(into)]
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// Maps a user to anonymous user. Defaults to `RootSquash`. Valid values: `RootSquash` (only root is mapped to anonymous user), `NoSquash` (no one is mapped to anonymous user), `AllSquash` (everyone is mapped to anonymous user)
        #[builder(into, default)]
        pub squash: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The DNS name of the VPC endpoint for S3 PrivateLink.
        #[builder(into, default)]
        pub vpc_endpoint_dns_name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct NfsFileShareResult {
        /// Amazon Resource Name (ARN) of the NFS File Share.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the storage used for audit logs.
        pub audit_destination_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The region of the S3 bucket used by the file share. Required when specifying `vpc_endpoint_dns_name`.
        pub bucket_region: pulumi_wasm_rust::Output<Option<String>>,
        /// Refresh cache information. see Cache Attributes for more details.
        pub cache_attributes: pulumi_wasm_rust::Output<
            Option<super::super::types::storagegateway::NfsFileShareCacheAttributes>,
        >,
        /// The list of clients that are allowed to access the file gateway. The list must contain either valid IP addresses or valid CIDR blocks. Set to `["0.0.0.0/0"]` to not limit access. Minimum 1 item. Maximum 100 items.
        pub client_lists: pulumi_wasm_rust::Output<Vec<String>>,
        /// The default [storage class](https://docs.aws.amazon.com/storagegateway/latest/APIReference/API_CreateNFSFileShare.html#StorageGateway-CreateNFSFileShare-request-DefaultStorageClass) for objects put into an Amazon S3 bucket by the file gateway. Defaults to `S3_STANDARD`.
        pub default_storage_class: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the file share. Must be set if an S3 prefix name is set in `location_arn`.
        pub file_share_name: pulumi_wasm_rust::Output<String>,
        /// ID of the NFS File Share.
        pub fileshare_id: pulumi_wasm_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the file gateway.
        pub gateway_arn: pulumi_wasm_rust::Output<String>,
        /// Boolean value that enables guessing of the MIME type for uploaded objects based on file extensions. Defaults to `true`.
        pub guess_mime_type_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Boolean value if `true` to use Amazon S3 server side encryption with your own AWS KMS key, or `false` to use a key managed by Amazon S3. Defaults to `false`.
        pub kms_encrypted: pulumi_wasm_rust::Output<Option<bool>>,
        /// Amazon Resource Name (ARN) for KMS key used for Amazon S3 server side encryption. This value can only be set when `kms_encrypted` is true.
        pub kms_key_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The ARN of the backed storage used for storing file data.
        pub location_arn: pulumi_wasm_rust::Output<String>,
        /// Nested argument with file share default values. More information below. see NFS File Share Defaults for more details.
        pub nfs_file_share_defaults: pulumi_wasm_rust::Output<
            Option<super::super::types::storagegateway::NfsFileShareNfsFileShareDefaults>,
        >,
        /// The notification policy of the file share. For more information see the [AWS Documentation](https://docs.aws.amazon.com/storagegateway/latest/APIReference/API_CreateNFSFileShare.html#StorageGateway-CreateNFSFileShare-request-NotificationPolicy). Default value is `{}`.
        pub notification_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// Access Control List permission for S3 objects. Defaults to `private`.
        pub object_acl: pulumi_wasm_rust::Output<Option<String>>,
        /// File share path used by the NFS client to identify the mount point.
        pub path: pulumi_wasm_rust::Output<String>,
        /// Boolean to indicate write status of file share. File share does not accept writes if `true`. Defaults to `false`.
        pub read_only: pulumi_wasm_rust::Output<Option<bool>>,
        /// Boolean who pays the cost of the request and the data download from the Amazon S3 bucket. Set this value to `true` if you want the requester to pay instead of the bucket owner. Defaults to `false`.
        pub requester_pays: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ARN of the AWS Identity and Access Management (IAM) role that a file gateway assumes when it accesses the underlying storage.
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// Maps a user to anonymous user. Defaults to `RootSquash`. Valid values: `RootSquash` (only root is mapped to anonymous user), `NoSquash` (no one is mapped to anonymous user), `AllSquash` (everyone is mapped to anonymous user)
        pub squash: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The DNS name of the VPC endpoint for S3 PrivateLink.
        pub vpc_endpoint_dns_name: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: NfsFileShareArgs) -> NfsFileShareResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let audit_destination_arn_binding = args.audit_destination_arn.get_inner();
        let bucket_region_binding = args.bucket_region.get_inner();
        let cache_attributes_binding = args.cache_attributes.get_inner();
        let client_lists_binding = args.client_lists.get_inner();
        let default_storage_class_binding = args.default_storage_class.get_inner();
        let file_share_name_binding = args.file_share_name.get_inner();
        let gateway_arn_binding = args.gateway_arn.get_inner();
        let guess_mime_type_enabled_binding = args.guess_mime_type_enabled.get_inner();
        let kms_encrypted_binding = args.kms_encrypted.get_inner();
        let kms_key_arn_binding = args.kms_key_arn.get_inner();
        let location_arn_binding = args.location_arn.get_inner();
        let nfs_file_share_defaults_binding = args.nfs_file_share_defaults.get_inner();
        let notification_policy_binding = args.notification_policy.get_inner();
        let object_acl_binding = args.object_acl.get_inner();
        let read_only_binding = args.read_only.get_inner();
        let requester_pays_binding = args.requester_pays.get_inner();
        let role_arn_binding = args.role_arn.get_inner();
        let squash_binding = args.squash.get_inner();
        let tags_binding = args.tags.get_inner();
        let vpc_endpoint_dns_name_binding = args.vpc_endpoint_dns_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:storagegateway/nfsFileShare:NfsFileShare".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "auditDestinationArn".into(),
                },
                register_interface::ResultField {
                    name: "bucketRegion".into(),
                },
                register_interface::ResultField {
                    name: "cacheAttributes".into(),
                },
                register_interface::ResultField {
                    name: "clientLists".into(),
                },
                register_interface::ResultField {
                    name: "defaultStorageClass".into(),
                },
                register_interface::ResultField {
                    name: "fileShareName".into(),
                },
                register_interface::ResultField {
                    name: "fileshareId".into(),
                },
                register_interface::ResultField {
                    name: "gatewayArn".into(),
                },
                register_interface::ResultField {
                    name: "guessMimeTypeEnabled".into(),
                },
                register_interface::ResultField {
                    name: "kmsEncrypted".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyArn".into(),
                },
                register_interface::ResultField {
                    name: "locationArn".into(),
                },
                register_interface::ResultField {
                    name: "nfsFileShareDefaults".into(),
                },
                register_interface::ResultField {
                    name: "notificationPolicy".into(),
                },
                register_interface::ResultField {
                    name: "objectAcl".into(),
                },
                register_interface::ResultField {
                    name: "path".into(),
                },
                register_interface::ResultField {
                    name: "readOnly".into(),
                },
                register_interface::ResultField {
                    name: "requesterPays".into(),
                },
                register_interface::ResultField {
                    name: "roleArn".into(),
                },
                register_interface::ResultField {
                    name: "squash".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "vpcEndpointDnsName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NfsFileShareResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            audit_destination_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("auditDestinationArn").unwrap(),
            ),
            bucket_region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bucketRegion").unwrap(),
            ),
            cache_attributes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cacheAttributes").unwrap(),
            ),
            client_lists: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientLists").unwrap(),
            ),
            default_storage_class: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultStorageClass").unwrap(),
            ),
            file_share_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fileShareName").unwrap(),
            ),
            fileshare_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fileshareId").unwrap(),
            ),
            gateway_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gatewayArn").unwrap(),
            ),
            guess_mime_type_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("guessMimeTypeEnabled").unwrap(),
            ),
            kms_encrypted: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsEncrypted").unwrap(),
            ),
            kms_key_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyArn").unwrap(),
            ),
            location_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("locationArn").unwrap(),
            ),
            nfs_file_share_defaults: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nfsFileShareDefaults").unwrap(),
            ),
            notification_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notificationPolicy").unwrap(),
            ),
            object_acl: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("objectAcl").unwrap(),
            ),
            path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("path").unwrap(),
            ),
            read_only: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("readOnly").unwrap(),
            ),
            requester_pays: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("requesterPays").unwrap(),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleArn").unwrap(),
            ),
            squash: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("squash").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            vpc_endpoint_dns_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcEndpointDnsName").unwrap(),
            ),
        }
    }
}
