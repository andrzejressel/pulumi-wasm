/// Provides a resource to manage an S3 Access Grants location.
/// A location is an S3 resource (bucket or prefix) in a permission grant that the grantee can access.
/// The S3 data must be in the same Region as your S3 Access Grants instance.
/// When you register a location, you must include the IAM role that has permission to manage the S3 location that you are registering.
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
///     let exampleAccessGrantsLocation = access_grants_location::create(
///         "exampleAccessGrantsLocation",
///         AccessGrantsLocationArgs::builder()
///             .iam_role_arn("${exampleAwsIamRole.arn}")
///             .location_scope("s3://")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import S3 Access Grants locations using the `account_id` and `access_grants_location_id`, separated by a comma (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:s3control/accessGrantsLocation:AccessGrantsLocation example 123456789012,default
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod access_grants_location {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccessGrantsLocationArgs {
        #[builder(into, default)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ARN of the IAM role that S3 Access Grants should use when fulfilling runtime access
        /// requests to the location.
        #[builder(into)]
        pub iam_role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The default S3 URI `s3://` or the URI to a custom location, a specific bucket or prefix.
        #[builder(into)]
        pub location_scope: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AccessGrantsLocationResult {
        /// Amazon Resource Name (ARN) of the S3 Access Grants location.
        pub access_grants_location_arn: pulumi_gestalt_rust::Output<String>,
        /// Unique ID of the S3 Access Grants location.
        pub access_grants_location_id: pulumi_gestalt_rust::Output<String>,
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the IAM role that S3 Access Grants should use when fulfilling runtime access
        /// requests to the location.
        pub iam_role_arn: pulumi_gestalt_rust::Output<String>,
        /// The default S3 URI `s3://` or the URI to a custom location, a specific bucket or prefix.
        pub location_scope: pulumi_gestalt_rust::Output<String>,
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
        args: AccessGrantsLocationArgs,
    ) -> AccessGrantsLocationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let iam_role_arn_binding = args.iam_role_arn.get_output(context).get_inner();
        let location_scope_binding = args.location_scope.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:s3control/accessGrantsLocation:AccessGrantsLocation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "iamRoleArn".into(),
                    value: &iam_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "locationScope".into(),
                    value: &location_scope_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AccessGrantsLocationResult {
            access_grants_location_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accessGrantsLocationArn"),
            ),
            access_grants_location_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accessGrantsLocationId"),
            ),
            account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            iam_role_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("iamRoleArn"),
            ),
            location_scope: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("locationScope"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
