/// Provides a resource to manage an S3 Access Grant.
/// Each access grant has its own ID and gives an IAM user or role or a directory user, or group (the grantee) access to a registered location. You determine the level of access, such as `READ` or `READWRITE`.
/// Before you can create a grant, you must have an S3 Access Grants instance in the same Region as the S3 data.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = access_grants_instance::create(
///         "example",
///         AccessGrantsInstanceArgs::builder().build_struct(),
///     );
///     let exampleAccessGrant = access_grant::create(
///         "exampleAccessGrant",
///         AccessGrantArgs::builder()
///             .access_grants_location_configuration(
///                 AccessGrantAccessGrantsLocationConfiguration::builder()
///                     .s3SubPrefix("prefixB*")
///                     .build_struct(),
///             )
///             .access_grants_location_id(
///                 "${exampleAccessGrantsLocation.accessGrantsLocationId}",
///             )
///             .grantee(
///                 AccessGrantGrantee::builder()
///                     .granteeIdentifier("${exampleAwsIamUser.arn}")
///                     .granteeType("IAM")
///                     .build_struct(),
///             )
///             .permission("READ")
///             .build_struct(),
///     );
///     let exampleAccessGrantsLocation = access_grants_location::create(
///         "exampleAccessGrantsLocation",
///         AccessGrantsLocationArgs::builder()
///             .iam_role_arn("${exampleAwsIamRole.arn}")
///             .location_scope("s3://${exampleAwsS3Bucket.bucket}/prefixA*")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import S3 Access Grants using the `account_id` and `access_grant_id`, separated by a comma (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:s3control/accessGrant:AccessGrant example 123456789012,04549c5e-2f3c-4a07-824d-2cafe720aa22
/// ```
pub mod access_grant {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccessGrantArgs {
        /// See Location Configuration below for more details.
        #[builder(into, default)]
        pub access_grants_location_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::s3control::AccessGrantAccessGrantsLocationConfiguration,
            >,
        >,
        /// The ID of the S3 Access Grants location to with the access grant is giving access.
        #[builder(into)]
        pub access_grants_location_id: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// See Grantee below for more details.
        #[builder(into, default)]
        pub grantee: pulumi_wasm_rust::Output<
            Option<super::super::types::s3control::AccessGrantGrantee>,
        >,
        /// The access grant's level of access. Valid values: `READ`, `WRITE`, `READWRITE`.
        #[builder(into)]
        pub permission: pulumi_wasm_rust::Output<String>,
        /// If you are creating an access grant that grants access to only one object, set this to `Object`. Valid values: `Object`.
        #[builder(into, default)]
        pub s3_prefix_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AccessGrantResult {
        /// Amazon Resource Name (ARN) of the S3 Access Grant.
        pub access_grant_arn: pulumi_wasm_rust::Output<String>,
        /// Unique ID of the S3 Access Grant.
        pub access_grant_id: pulumi_wasm_rust::Output<String>,
        /// See Location Configuration below for more details.
        pub access_grants_location_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::s3control::AccessGrantAccessGrantsLocationConfiguration,
            >,
        >,
        /// The ID of the S3 Access Grants location to with the access grant is giving access.
        pub access_grants_location_id: pulumi_wasm_rust::Output<String>,
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// The access grant's scope.
        pub grant_scope: pulumi_wasm_rust::Output<String>,
        /// See Grantee below for more details.
        pub grantee: pulumi_wasm_rust::Output<
            Option<super::super::types::s3control::AccessGrantGrantee>,
        >,
        /// The access grant's level of access. Valid values: `READ`, `WRITE`, `READWRITE`.
        pub permission: pulumi_wasm_rust::Output<String>,
        /// If you are creating an access grant that grants access to only one object, set this to `Object`. Valid values: `Object`.
        pub s3_prefix_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AccessGrantArgs) -> AccessGrantResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_grants_location_configuration_binding = args
            .access_grants_location_configuration
            .get_inner();
        let access_grants_location_id_binding = args
            .access_grants_location_id
            .get_inner();
        let account_id_binding = args.account_id.get_inner();
        let grantee_binding = args.grantee.get_inner();
        let permission_binding = args.permission.get_inner();
        let s3_prefix_type_binding = args.s3_prefix_type.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:s3control/accessGrant:AccessGrant".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessGrantsLocationConfiguration".into(),
                    value: &access_grants_location_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "accessGrantsLocationId".into(),
                    value: &access_grants_location_id_binding,
                },
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "grantee".into(),
                    value: &grantee_binding,
                },
                register_interface::ObjectField {
                    name: "permission".into(),
                    value: &permission_binding,
                },
                register_interface::ObjectField {
                    name: "s3PrefixType".into(),
                    value: &s3_prefix_type_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessGrantArn".into(),
                },
                register_interface::ResultField {
                    name: "accessGrantId".into(),
                },
                register_interface::ResultField {
                    name: "accessGrantsLocationConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "accessGrantsLocationId".into(),
                },
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "grantScope".into(),
                },
                register_interface::ResultField {
                    name: "grantee".into(),
                },
                register_interface::ResultField {
                    name: "permission".into(),
                },
                register_interface::ResultField {
                    name: "s3PrefixType".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AccessGrantResult {
            access_grant_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessGrantArn").unwrap(),
            ),
            access_grant_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessGrantId").unwrap(),
            ),
            access_grants_location_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessGrantsLocationConfiguration").unwrap(),
            ),
            access_grants_location_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessGrantsLocationId").unwrap(),
            ),
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            grant_scope: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("grantScope").unwrap(),
            ),
            grantee: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("grantee").unwrap(),
            ),
            permission: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("permission").unwrap(),
            ),
            s3_prefix_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("s3PrefixType").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
