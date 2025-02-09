/// Manages an AWS Storage Gateway SMB File Share.
///
/// ## Example Usage
///
/// ### Active Directory Authentication
///
/// > **NOTE:** The gateway must have already joined the Active Directory domain prior to SMB file share creationE.g., via "SMB Settings" in the AWS Storage Gateway console or `smb_active_directory_settings` in the `aws.storagegateway.Gateway` resource.
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod smb_file_share {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SmbFileShareArgs {
        /// The files and folders on this share will only be visible to users with read access. Default value is `false`.
        #[builder(into, default)]
        pub access_based_enumeration: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A list of users in the Active Directory that have admin access to the file share. Only valid if `authentication` is set to `ActiveDirectory`.
        #[builder(into, default)]
        pub admin_user_lists: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The Amazon Resource Name (ARN) of the CloudWatch Log Group used for the audit logs.
        #[builder(into, default)]
        pub audit_destination_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The authentication method that users use to access the file share. Defaults to `ActiveDirectory`. Valid values: `ActiveDirectory`, `GuestAccess`.
        #[builder(into, default)]
        pub authentication: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The region of the S3 buck used by the file share. Required when specifying a `vpc_endpoint_dns_name`.
        #[builder(into, default)]
        pub bucket_region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Refresh cache information. see `cache_attributes` Block for more details.
        ///
        /// **Note:** If you have previously included a `cache_attributes` block in your configuration, removing it will not reset the refresh cache value and the previous value will remain. You must explicitly set a new value to change it.
        #[builder(into, default)]
        pub cache_attributes: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::storagegateway::SmbFileShareCacheAttributes>,
        >,
        /// The case of an object name in an Amazon S3 bucket. For `ClientSpecified`, the client determines the case sensitivity. For `CaseSensitive`, the gateway determines the case sensitivity. The default value is `ClientSpecified`.
        #[builder(into, default)]
        pub case_sensitivity: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
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
        /// A list of users in the Active Directory that are not allowed to access the file share. Only valid if `authentication` is set to `ActiveDirectory`.
        #[builder(into, default)]
        pub invalid_user_lists: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Boolean value if `true` to use Amazon S3 server side encryption with your own AWS KMS key, or `false` to use a key managed by Amazon S3. Defaults to `false`.
        #[builder(into, default)]
        pub kms_encrypted: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Amazon Resource Name (ARN) for KMS key used for Amazon S3 server side encryption. This value can only be set when `kms_encrypted` is true.
        #[builder(into, default)]
        pub kms_key_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ARN of the backed storage used for storing file data.
        #[builder(into)]
        pub location_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The notification policy of the file share. For more information see the [AWS Documentation](https://docs.aws.amazon.com/storagegateway/latest/APIReference/API_CreateNFSFileShare.html#StorageGateway-CreateNFSFileShare-request-NotificationPolicy). Default value is `{}`.
        #[builder(into, default)]
        pub notification_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Access Control List permission for S3 objects. Defaults to `private`.
        #[builder(into, default)]
        pub object_acl: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Boolean to indicate Opportunistic lock (oplock) status. Defaults to `true`.
        #[builder(into, default)]
        pub oplocks_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Boolean to indicate write status of file share. File share does not accept writes if `true`. Defaults to `false`.
        #[builder(into, default)]
        pub read_only: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Boolean who pays the cost of the request and the data download from the Amazon S3 bucket. Set this value to `true` if you want the requester to pay instead of the bucket owner. Defaults to `false`.
        #[builder(into, default)]
        pub requester_pays: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The ARN of the AWS Identity and Access Management (IAM) role that a file gateway assumes when it accesses the underlying storage.
        #[builder(into)]
        pub role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Set this value to `true` to enable ACL (access control list) on the SMB fileshare. Set it to `false` to map file and directory permissions to the POSIX permissions. This setting applies only to `ActiveDirectory` authentication type.
        #[builder(into, default)]
        pub smb_acl_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A list of users in the Active Directory that are allowed to access the file share. If you need to specify an Active directory group, add '@' before the name of the group. It will be set on Allowed group in AWS console. Only valid if `authentication` is set to `ActiveDirectory`.
        #[builder(into, default)]
        pub valid_user_lists: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The DNS name of the VPC endpoint for S3 private link.
        #[builder(into, default)]
        pub vpc_endpoint_dns_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SmbFileShareResult {
        /// The files and folders on this share will only be visible to users with read access. Default value is `false`.
        pub access_based_enumeration: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A list of users in the Active Directory that have admin access to the file share. Only valid if `authentication` is set to `ActiveDirectory`.
        pub admin_user_lists: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Amazon Resource Name (ARN) of the SMB File Share.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the CloudWatch Log Group used for the audit logs.
        pub audit_destination_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// The authentication method that users use to access the file share. Defaults to `ActiveDirectory`. Valid values: `ActiveDirectory`, `GuestAccess`.
        pub authentication: pulumi_gestalt_rust::Output<Option<String>>,
        /// The region of the S3 buck used by the file share. Required when specifying a `vpc_endpoint_dns_name`.
        pub bucket_region: pulumi_gestalt_rust::Output<Option<String>>,
        /// Refresh cache information. see `cache_attributes` Block for more details.
        ///
        /// **Note:** If you have previously included a `cache_attributes` block in your configuration, removing it will not reset the refresh cache value and the previous value will remain. You must explicitly set a new value to change it.
        pub cache_attributes: pulumi_gestalt_rust::Output<
            Option<super::super::types::storagegateway::SmbFileShareCacheAttributes>,
        >,
        /// The case of an object name in an Amazon S3 bucket. For `ClientSpecified`, the client determines the case sensitivity. For `CaseSensitive`, the gateway determines the case sensitivity. The default value is `ClientSpecified`.
        pub case_sensitivity: pulumi_gestalt_rust::Output<Option<String>>,
        /// The default [storage class](https://docs.aws.amazon.com/storagegateway/latest/APIReference/API_CreateNFSFileShare.html#StorageGateway-CreateNFSFileShare-request-DefaultStorageClass) for objects put into an Amazon S3 bucket by the file gateway. Defaults to `S3_STANDARD`.
        pub default_storage_class: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the file share. Must be set if an S3 prefix name is set in `location_arn`.
        pub file_share_name: pulumi_gestalt_rust::Output<String>,
        /// ID of the SMB File Share.
        pub fileshare_id: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the file gateway.
        pub gateway_arn: pulumi_gestalt_rust::Output<String>,
        /// Boolean value that enables guessing of the MIME type for uploaded objects based on file extensions. Defaults to `true`.
        pub guess_mime_type_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A list of users in the Active Directory that are not allowed to access the file share. Only valid if `authentication` is set to `ActiveDirectory`.
        pub invalid_user_lists: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Boolean value if `true` to use Amazon S3 server side encryption with your own AWS KMS key, or `false` to use a key managed by Amazon S3. Defaults to `false`.
        pub kms_encrypted: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Amazon Resource Name (ARN) for KMS key used for Amazon S3 server side encryption. This value can only be set when `kms_encrypted` is true.
        pub kms_key_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ARN of the backed storage used for storing file data.
        pub location_arn: pulumi_gestalt_rust::Output<String>,
        /// The notification policy of the file share. For more information see the [AWS Documentation](https://docs.aws.amazon.com/storagegateway/latest/APIReference/API_CreateNFSFileShare.html#StorageGateway-CreateNFSFileShare-request-NotificationPolicy). Default value is `{}`.
        pub notification_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// Access Control List permission for S3 objects. Defaults to `private`.
        pub object_acl: pulumi_gestalt_rust::Output<Option<String>>,
        /// Boolean to indicate Opportunistic lock (oplock) status. Defaults to `true`.
        pub oplocks_enabled: pulumi_gestalt_rust::Output<bool>,
        /// File share path used by the NFS client to identify the mount point.
        pub path: pulumi_gestalt_rust::Output<String>,
        /// Boolean to indicate write status of file share. File share does not accept writes if `true`. Defaults to `false`.
        pub read_only: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Boolean who pays the cost of the request and the data download from the Amazon S3 bucket. Set this value to `true` if you want the requester to pay instead of the bucket owner. Defaults to `false`.
        pub requester_pays: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The ARN of the AWS Identity and Access Management (IAM) role that a file gateway assumes when it accesses the underlying storage.
        pub role_arn: pulumi_gestalt_rust::Output<String>,
        /// Set this value to `true` to enable ACL (access control list) on the SMB fileshare. Set it to `false` to map file and directory permissions to the POSIX permissions. This setting applies only to `ActiveDirectory` authentication type.
        pub smb_acl_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A list of users in the Active Directory that are allowed to access the file share. If you need to specify an Active directory group, add '@' before the name of the group. It will be set on Allowed group in AWS console. Only valid if `authentication` is set to `ActiveDirectory`.
        pub valid_user_lists: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The DNS name of the VPC endpoint for S3 private link.
        pub vpc_endpoint_dns_name: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SmbFileShareArgs,
    ) -> SmbFileShareResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let access_based_enumeration_binding = args
            .access_based_enumeration
            .get_output(context);
        let admin_user_lists_binding = args.admin_user_lists.get_output(context);
        let audit_destination_arn_binding = args
            .audit_destination_arn
            .get_output(context);
        let authentication_binding = args.authentication.get_output(context);
        let bucket_region_binding = args.bucket_region.get_output(context);
        let cache_attributes_binding = args.cache_attributes.get_output(context);
        let case_sensitivity_binding = args.case_sensitivity.get_output(context);
        let default_storage_class_binding = args
            .default_storage_class
            .get_output(context);
        let file_share_name_binding = args.file_share_name.get_output(context);
        let gateway_arn_binding = args.gateway_arn.get_output(context);
        let guess_mime_type_enabled_binding = args
            .guess_mime_type_enabled
            .get_output(context);
        let invalid_user_lists_binding = args.invalid_user_lists.get_output(context);
        let kms_encrypted_binding = args.kms_encrypted.get_output(context);
        let kms_key_arn_binding = args.kms_key_arn.get_output(context);
        let location_arn_binding = args.location_arn.get_output(context);
        let notification_policy_binding = args.notification_policy.get_output(context);
        let object_acl_binding = args.object_acl.get_output(context);
        let oplocks_enabled_binding = args.oplocks_enabled.get_output(context);
        let read_only_binding = args.read_only.get_output(context);
        let requester_pays_binding = args.requester_pays.get_output(context);
        let role_arn_binding = args.role_arn.get_output(context);
        let smb_acl_enabled_binding = args.smb_acl_enabled.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let valid_user_lists_binding = args.valid_user_lists.get_output(context);
        let vpc_endpoint_dns_name_binding = args
            .vpc_endpoint_dns_name
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:storagegateway/smbFileShare:SmbFileShare".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accessBasedEnumeration".into(),
                    value: access_based_enumeration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "adminUserLists".into(),
                    value: admin_user_lists_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "auditDestinationArn".into(),
                    value: audit_destination_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authentication".into(),
                    value: authentication_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bucketRegion".into(),
                    value: bucket_region_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cacheAttributes".into(),
                    value: cache_attributes_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "caseSensitivity".into(),
                    value: case_sensitivity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultStorageClass".into(),
                    value: default_storage_class_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fileShareName".into(),
                    value: file_share_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gatewayArn".into(),
                    value: gateway_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "guessMimeTypeEnabled".into(),
                    value: guess_mime_type_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "invalidUserLists".into(),
                    value: invalid_user_lists_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsEncrypted".into(),
                    value: kms_encrypted_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsKeyArn".into(),
                    value: kms_key_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "locationArn".into(),
                    value: location_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "notificationPolicy".into(),
                    value: notification_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "objectAcl".into(),
                    value: object_acl_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "oplocksEnabled".into(),
                    value: oplocks_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "readOnly".into(),
                    value: read_only_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "requesterPays".into(),
                    value: requester_pays_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleArn".into(),
                    value: role_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "smbAclEnabled".into(),
                    value: smb_acl_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "validUserLists".into(),
                    value: valid_user_lists_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcEndpointDnsName".into(),
                    value: vpc_endpoint_dns_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SmbFileShareResult {
            access_based_enumeration: o.get_field("accessBasedEnumeration"),
            admin_user_lists: o.get_field("adminUserLists"),
            arn: o.get_field("arn"),
            audit_destination_arn: o.get_field("auditDestinationArn"),
            authentication: o.get_field("authentication"),
            bucket_region: o.get_field("bucketRegion"),
            cache_attributes: o.get_field("cacheAttributes"),
            case_sensitivity: o.get_field("caseSensitivity"),
            default_storage_class: o.get_field("defaultStorageClass"),
            file_share_name: o.get_field("fileShareName"),
            fileshare_id: o.get_field("fileshareId"),
            gateway_arn: o.get_field("gatewayArn"),
            guess_mime_type_enabled: o.get_field("guessMimeTypeEnabled"),
            invalid_user_lists: o.get_field("invalidUserLists"),
            kms_encrypted: o.get_field("kmsEncrypted"),
            kms_key_arn: o.get_field("kmsKeyArn"),
            location_arn: o.get_field("locationArn"),
            notification_policy: o.get_field("notificationPolicy"),
            object_acl: o.get_field("objectAcl"),
            oplocks_enabled: o.get_field("oplocksEnabled"),
            path: o.get_field("path"),
            read_only: o.get_field("readOnly"),
            requester_pays: o.get_field("requesterPays"),
            role_arn: o.get_field("roleArn"),
            smb_acl_enabled: o.get_field("smbAclEnabled"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            valid_user_lists: o.get_field("validUserLists"),
            vpc_endpoint_dns_name: o.get_field("vpcEndpointDnsName"),
        }
    }
}
