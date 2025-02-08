/// Provides a resource to manage an S3 Access Grant.
/// Each access grant has its own ID and gives an IAM user or role or a directory user, or group (the grantee) access to a registered location. You determine the level of access, such as `READ` or `READWRITE`.
/// Before you can create a grant, you must have an S3 Access Grants instance in the same Region as the S3 data.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod access_grant {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccessGrantArgs {
        /// See Location Configuration below for more details.
        #[builder(into, default)]
        pub access_grants_location_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::s3control::AccessGrantAccessGrantsLocationConfiguration,
            >,
        >,
        /// The ID of the S3 Access Grants location to with the access grant is giving access.
        #[builder(into)]
        pub access_grants_location_id: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// See Grantee below for more details.
        #[builder(into, default)]
        pub grantee: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::s3control::AccessGrantGrantee>,
        >,
        /// The access grant's level of access. Valid values: `READ`, `WRITE`, `READWRITE`.
        #[builder(into)]
        pub permission: pulumi_gestalt_rust::InputOrOutput<String>,
        /// If you are creating an access grant that grants access to only one object, set this to `Object`. Valid values: `Object`.
        #[builder(into, default)]
        pub s3_prefix_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AccessGrantResult {
        /// Amazon Resource Name (ARN) of the S3 Access Grant.
        pub access_grant_arn: pulumi_gestalt_rust::Output<String>,
        /// Unique ID of the S3 Access Grant.
        pub access_grant_id: pulumi_gestalt_rust::Output<String>,
        /// See Location Configuration below for more details.
        pub access_grants_location_configuration: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::s3control::AccessGrantAccessGrantsLocationConfiguration,
            >,
        >,
        /// The ID of the S3 Access Grants location to with the access grant is giving access.
        pub access_grants_location_id: pulumi_gestalt_rust::Output<String>,
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// The access grant's scope.
        pub grant_scope: pulumi_gestalt_rust::Output<String>,
        /// See Grantee below for more details.
        pub grantee: pulumi_gestalt_rust::Output<
            Option<super::super::types::s3control::AccessGrantGrantee>,
        >,
        /// The access grant's level of access. Valid values: `READ`, `WRITE`, `READWRITE`.
        pub permission: pulumi_gestalt_rust::Output<String>,
        /// If you are creating an access grant that grants access to only one object, set this to `Object`. Valid values: `Object`.
        pub s3_prefix_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: AccessGrantArgs,
    ) -> AccessGrantResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let access_grants_location_configuration_binding = args
            .access_grants_location_configuration
            .get_output(context)
            .get_inner();
        let access_grants_location_id_binding = args
            .access_grants_location_id
            .get_output(context)
            .get_inner();
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let grantee_binding = args.grantee.get_output(context).get_inner();
        let permission_binding = args.permission.get_output(context).get_inner();
        let s3_prefix_type_binding = args.s3_prefix_type.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        AccessGrantResult {
            access_grant_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accessGrantArn"),
            ),
            access_grant_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accessGrantId"),
            ),
            access_grants_location_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accessGrantsLocationConfiguration"),
            ),
            access_grants_location_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accessGrantsLocationId"),
            ),
            account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            grant_scope: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("grantScope"),
            ),
            grantee: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("grantee"),
            ),
            permission: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("permission"),
            ),
            s3_prefix_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("s3PrefixType"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
