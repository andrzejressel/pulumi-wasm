/// Manages an AWS Storage Gateway SMB File Share.
///
/// ## Example Usage
///
/// ### Active Directory Authentication
///
/// > **NOTE:** The gateway must have already joined the Active Directory domain prior to SMB file share creationE.g., via "SMB Settings" in the AWS Storage Gateway console or `smb_active_directory_settings` in the `aws.storagegateway.Gateway` resource.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = smb_file_share::create(
///         "example",
///         SmbFileShareArgs::builder()
///             .authentication("ActiveDirectory")
///             .gateway_arn("${exampleAwsStoragegatewayGateway.arn}")
///             .location_arn("${exampleAwsS3Bucket.arn}")
///             .role_arn("${exampleAwsIamRole.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Guest Authentication
///
/// > **NOTE:** The gateway must have already had the SMB guest password set prior to SMB file share creationE.g., via "SMB Settings" in the AWS Storage Gateway console or `smb_guest_password` in the `aws.storagegateway.Gateway` resource.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = smb_file_share::create(
///         "example",
///         SmbFileShareArgs::builder()
///             .authentication("GuestAccess")
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
/// Using `pulumi import`, import `aws_storagegateway_smb_file_share` using the SMB File Share Amazon Resource Name (ARN). For example:
///
/// ```sh
/// $ pulumi import aws:storagegateway/smbFileShare:SmbFileShare example arn:aws:storagegateway:us-east-1:123456789012:share/share-12345678
/// ```
pub mod smb_file_share {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SmbFileShareArgs {
        /// The files and folders on this share will only be visible to users with read access. Default value is `false`.
        #[builder(into, default)]
        pub access_based_enumeration: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// A list of users in the Active Directory that have admin access to the file share. Only valid if `authentication` is set to `ActiveDirectory`.
        #[builder(into, default)]
        pub admin_user_lists: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// The Amazon Resource Name (ARN) of the CloudWatch Log Group used for the audit logs.
        #[builder(into, default)]
        pub audit_destination_arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The authentication method that users use to access the file share. Defaults to `ActiveDirectory`. Valid values: `ActiveDirectory`, `GuestAccess`.
        #[builder(into, default)]
        pub authentication: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The region of the S3 buck used by the file share. Required when specifying a `vpc_endpoint_dns_name`.
        #[builder(into, default)]
        pub bucket_region: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Refresh cache information. see `cache_attributes` Block for more details.
        ///
        /// **Note:** If you have previously included a `cache_attributes` block in your configuration, removing it will not reset the refresh cache value and the previous value will remain. You must explicitly set a new value to change it.
        #[builder(into, default)]
        pub cache_attributes: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::storagegateway::SmbFileShareCacheAttributes>,
        >,
        /// The case of an object name in an Amazon S3 bucket. For `ClientSpecified`, the client determines the case sensitivity. For `CaseSensitive`, the gateway determines the case sensitivity. The default value is `ClientSpecified`.
        #[builder(into, default)]
        pub case_sensitivity: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The default [storage class](https://docs.aws.amazon.com/storagegateway/latest/APIReference/API_CreateNFSFileShare.html#StorageGateway-CreateNFSFileShare-request-DefaultStorageClass) for objects put into an Amazon S3 bucket by the file gateway. Defaults to `S3_STANDARD`.
        #[builder(into, default)]
        pub default_storage_class: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the file share. Must be set if an S3 prefix name is set in `location_arn`.
        #[builder(into, default)]
        pub file_share_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Amazon Resource Name (ARN) of the file gateway.
        #[builder(into)]
        pub gateway_arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// Boolean value that enables guessing of the MIME type for uploaded objects based on file extensions. Defaults to `true`.
        #[builder(into, default)]
        pub guess_mime_type_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// A list of users in the Active Directory that are not allowed to access the file share. Only valid if `authentication` is set to `ActiveDirectory`.
        #[builder(into, default)]
        pub invalid_user_lists: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Boolean value if `true` to use Amazon S3 server side encryption with your own AWS KMS key, or `false` to use a key managed by Amazon S3. Defaults to `false`.
        #[builder(into, default)]
        pub kms_encrypted: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Amazon Resource Name (ARN) for KMS key used for Amazon S3 server side encryption. This value can only be set when `kms_encrypted` is true.
        #[builder(into, default)]
        pub kms_key_arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ARN of the backed storage used for storing file data.
        #[builder(into)]
        pub location_arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// The notification policy of the file share. For more information see the [AWS Documentation](https://docs.aws.amazon.com/storagegateway/latest/APIReference/API_CreateNFSFileShare.html#StorageGateway-CreateNFSFileShare-request-NotificationPolicy). Default value is `{}`.
        #[builder(into, default)]
        pub notification_policy: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Access Control List permission for S3 objects. Defaults to `private`.
        #[builder(into, default)]
        pub object_acl: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Boolean to indicate Opportunistic lock (oplock) status. Defaults to `true`.
        #[builder(into, default)]
        pub oplocks_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Boolean to indicate write status of file share. File share does not accept writes if `true`. Defaults to `false`.
        #[builder(into, default)]
        pub read_only: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Boolean who pays the cost of the request and the data download from the Amazon S3 bucket. Set this value to `true` if you want the requester to pay instead of the bucket owner. Defaults to `false`.
        #[builder(into, default)]
        pub requester_pays: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The ARN of the AWS Identity and Access Management (IAM) role that a file gateway assumes when it accesses the underlying storage.
        #[builder(into)]
        pub role_arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// Set this value to `true` to enable ACL (access control list) on the SMB fileshare. Set it to `false` to map file and directory permissions to the POSIX permissions. This setting applies only to `ActiveDirectory` authentication type.
        #[builder(into, default)]
        pub smb_acl_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A list of users in the Active Directory that are allowed to access the file share. If you need to specify an Active directory group, add '@' before the name of the group. It will be set on Allowed group in AWS console. Only valid if `authentication` is set to `ActiveDirectory`.
        #[builder(into, default)]
        pub valid_user_lists: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// The DNS name of the VPC endpoint for S3 private link.
        #[builder(into, default)]
        pub vpc_endpoint_dns_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SmbFileShareResult {
        /// The files and folders on this share will only be visible to users with read access. Default value is `false`.
        pub access_based_enumeration: pulumi_wasm_rust::Output<Option<bool>>,
        /// A list of users in the Active Directory that have admin access to the file share. Only valid if `authentication` is set to `ActiveDirectory`.
        pub admin_user_lists: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Amazon Resource Name (ARN) of the SMB File Share.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the CloudWatch Log Group used for the audit logs.
        pub audit_destination_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The authentication method that users use to access the file share. Defaults to `ActiveDirectory`. Valid values: `ActiveDirectory`, `GuestAccess`.
        pub authentication: pulumi_wasm_rust::Output<Option<String>>,
        /// The region of the S3 buck used by the file share. Required when specifying a `vpc_endpoint_dns_name`.
        pub bucket_region: pulumi_wasm_rust::Output<Option<String>>,
        /// Refresh cache information. see `cache_attributes` Block for more details.
        ///
        /// **Note:** If you have previously included a `cache_attributes` block in your configuration, removing it will not reset the refresh cache value and the previous value will remain. You must explicitly set a new value to change it.
        pub cache_attributes: pulumi_wasm_rust::Output<
            Option<super::super::types::storagegateway::SmbFileShareCacheAttributes>,
        >,
        /// The case of an object name in an Amazon S3 bucket. For `ClientSpecified`, the client determines the case sensitivity. For `CaseSensitive`, the gateway determines the case sensitivity. The default value is `ClientSpecified`.
        pub case_sensitivity: pulumi_wasm_rust::Output<Option<String>>,
        /// The default [storage class](https://docs.aws.amazon.com/storagegateway/latest/APIReference/API_CreateNFSFileShare.html#StorageGateway-CreateNFSFileShare-request-DefaultStorageClass) for objects put into an Amazon S3 bucket by the file gateway. Defaults to `S3_STANDARD`.
        pub default_storage_class: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the file share. Must be set if an S3 prefix name is set in `location_arn`.
        pub file_share_name: pulumi_wasm_rust::Output<String>,
        /// ID of the SMB File Share.
        pub fileshare_id: pulumi_wasm_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the file gateway.
        pub gateway_arn: pulumi_wasm_rust::Output<String>,
        /// Boolean value that enables guessing of the MIME type for uploaded objects based on file extensions. Defaults to `true`.
        pub guess_mime_type_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A list of users in the Active Directory that are not allowed to access the file share. Only valid if `authentication` is set to `ActiveDirectory`.
        pub invalid_user_lists: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Boolean value if `true` to use Amazon S3 server side encryption with your own AWS KMS key, or `false` to use a key managed by Amazon S3. Defaults to `false`.
        pub kms_encrypted: pulumi_wasm_rust::Output<Option<bool>>,
        /// Amazon Resource Name (ARN) for KMS key used for Amazon S3 server side encryption. This value can only be set when `kms_encrypted` is true.
        pub kms_key_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The ARN of the backed storage used for storing file data.
        pub location_arn: pulumi_wasm_rust::Output<String>,
        /// The notification policy of the file share. For more information see the [AWS Documentation](https://docs.aws.amazon.com/storagegateway/latest/APIReference/API_CreateNFSFileShare.html#StorageGateway-CreateNFSFileShare-request-NotificationPolicy). Default value is `{}`.
        pub notification_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// Access Control List permission for S3 objects. Defaults to `private`.
        pub object_acl: pulumi_wasm_rust::Output<Option<String>>,
        /// Boolean to indicate Opportunistic lock (oplock) status. Defaults to `true`.
        pub oplocks_enabled: pulumi_wasm_rust::Output<bool>,
        /// File share path used by the NFS client to identify the mount point.
        pub path: pulumi_wasm_rust::Output<String>,
        /// Boolean to indicate write status of file share. File share does not accept writes if `true`. Defaults to `false`.
        pub read_only: pulumi_wasm_rust::Output<Option<bool>>,
        /// Boolean who pays the cost of the request and the data download from the Amazon S3 bucket. Set this value to `true` if you want the requester to pay instead of the bucket owner. Defaults to `false`.
        pub requester_pays: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ARN of the AWS Identity and Access Management (IAM) role that a file gateway assumes when it accesses the underlying storage.
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// Set this value to `true` to enable ACL (access control list) on the SMB fileshare. Set it to `false` to map file and directory permissions to the POSIX permissions. This setting applies only to `ActiveDirectory` authentication type.
        pub smb_acl_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A list of users in the Active Directory that are allowed to access the file share. If you need to specify an Active directory group, add '@' before the name of the group. It will be set on Allowed group in AWS console. Only valid if `authentication` is set to `ActiveDirectory`.
        pub valid_user_lists: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The DNS name of the VPC endpoint for S3 private link.
        pub vpc_endpoint_dns_name: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: SmbFileShareArgs,
    ) -> SmbFileShareResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_based_enumeration_binding = args
            .access_based_enumeration
            .get_output(context)
            .get_inner();
        let admin_user_lists_binding = args
            .admin_user_lists
            .get_output(context)
            .get_inner();
        let audit_destination_arn_binding = args
            .audit_destination_arn
            .get_output(context)
            .get_inner();
        let authentication_binding = args.authentication.get_output(context).get_inner();
        let bucket_region_binding = args.bucket_region.get_output(context).get_inner();
        let cache_attributes_binding = args
            .cache_attributes
            .get_output(context)
            .get_inner();
        let case_sensitivity_binding = args
            .case_sensitivity
            .get_output(context)
            .get_inner();
        let default_storage_class_binding = args
            .default_storage_class
            .get_output(context)
            .get_inner();
        let file_share_name_binding = args
            .file_share_name
            .get_output(context)
            .get_inner();
        let gateway_arn_binding = args.gateway_arn.get_output(context).get_inner();
        let guess_mime_type_enabled_binding = args
            .guess_mime_type_enabled
            .get_output(context)
            .get_inner();
        let invalid_user_lists_binding = args
            .invalid_user_lists
            .get_output(context)
            .get_inner();
        let kms_encrypted_binding = args.kms_encrypted.get_output(context).get_inner();
        let kms_key_arn_binding = args.kms_key_arn.get_output(context).get_inner();
        let location_arn_binding = args.location_arn.get_output(context).get_inner();
        let notification_policy_binding = args
            .notification_policy
            .get_output(context)
            .get_inner();
        let object_acl_binding = args.object_acl.get_output(context).get_inner();
        let oplocks_enabled_binding = args
            .oplocks_enabled
            .get_output(context)
            .get_inner();
        let read_only_binding = args.read_only.get_output(context).get_inner();
        let requester_pays_binding = args.requester_pays.get_output(context).get_inner();
        let role_arn_binding = args.role_arn.get_output(context).get_inner();
        let smb_acl_enabled_binding = args
            .smb_acl_enabled
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let valid_user_lists_binding = args
            .valid_user_lists
            .get_output(context)
            .get_inner();
        let vpc_endpoint_dns_name_binding = args
            .vpc_endpoint_dns_name
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:storagegateway/smbFileShare:SmbFileShare".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessBasedEnumeration".into(),
                    value: &access_based_enumeration_binding,
                },
                register_interface::ObjectField {
                    name: "adminUserLists".into(),
                    value: &admin_user_lists_binding,
                },
                register_interface::ObjectField {
                    name: "auditDestinationArn".into(),
                    value: &audit_destination_arn_binding,
                },
                register_interface::ObjectField {
                    name: "authentication".into(),
                    value: &authentication_binding,
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
                    name: "caseSensitivity".into(),
                    value: &case_sensitivity_binding,
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
                    name: "invalidUserLists".into(),
                    value: &invalid_user_lists_binding,
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
                    name: "notificationPolicy".into(),
                    value: &notification_policy_binding,
                },
                register_interface::ObjectField {
                    name: "objectAcl".into(),
                    value: &object_acl_binding,
                },
                register_interface::ObjectField {
                    name: "oplocksEnabled".into(),
                    value: &oplocks_enabled_binding,
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
                    name: "smbAclEnabled".into(),
                    value: &smb_acl_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "validUserLists".into(),
                    value: &valid_user_lists_binding,
                },
                register_interface::ObjectField {
                    name: "vpcEndpointDnsName".into(),
                    value: &vpc_endpoint_dns_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SmbFileShareResult {
            access_based_enumeration: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accessBasedEnumeration"),
            ),
            admin_user_lists: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("adminUserLists"),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            audit_destination_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("auditDestinationArn"),
            ),
            authentication: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("authentication"),
            ),
            bucket_region: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("bucketRegion"),
            ),
            cache_attributes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cacheAttributes"),
            ),
            case_sensitivity: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("caseSensitivity"),
            ),
            default_storage_class: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("defaultStorageClass"),
            ),
            file_share_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("fileShareName"),
            ),
            fileshare_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("fileshareId"),
            ),
            gateway_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("gatewayArn"),
            ),
            guess_mime_type_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("guessMimeTypeEnabled"),
            ),
            invalid_user_lists: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("invalidUserLists"),
            ),
            kms_encrypted: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("kmsEncrypted"),
            ),
            kms_key_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("kmsKeyArn"),
            ),
            location_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("locationArn"),
            ),
            notification_policy: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("notificationPolicy"),
            ),
            object_acl: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("objectAcl"),
            ),
            oplocks_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("oplocksEnabled"),
            ),
            path: pulumi_wasm_rust::__private::into_domain(o.extract_field("path")),
            read_only: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("readOnly"),
            ),
            requester_pays: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("requesterPays"),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("roleArn"),
            ),
            smb_acl_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("smbAclEnabled"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            valid_user_lists: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("validUserLists"),
            ),
            vpc_endpoint_dns_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("vpcEndpointDnsName"),
            ),
        }
    }
}
