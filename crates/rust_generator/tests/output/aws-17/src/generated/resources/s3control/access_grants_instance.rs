/// Provides a resource to manage an S3 Access Grants instance, which serves as a logical grouping for access grants.
/// You can have one S3 Access Grants instance per Region in your account.
///
/// ## Example Usage
///
/// ### Basic Usage
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
/// }
/// ```
///
/// ### AWS IAM Identity Center
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = access_grants_instance::create(
///         "example",
///         AccessGrantsInstanceArgs::builder()
///             .identity_center_arn("arn:aws:sso:::instance/ssoins-890759e9c7bfdc1d")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import S3 Access Grants instances using the `account_id`. For example:
///
/// ```sh
/// $ pulumi import aws:s3control/accessGrantsInstance:AccessGrantsInstance example 123456789012
/// ```
#[allow(clippy::doc_lazy_continuation)]
pub mod access_grants_instance {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccessGrantsInstanceArgs {
        #[builder(into, default)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ARN of the AWS IAM Identity Center instance associated with the S3 Access Grants instance.
        #[builder(into, default)]
        pub identity_center_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AccessGrantsInstanceResult {
        /// Amazon Resource Name (ARN) of the S3 Access Grants instance.
        pub access_grants_instance_arn: pulumi_gestalt_rust::Output<String>,
        /// Unique ID of the S3 Access Grants instance.
        pub access_grants_instance_id: pulumi_gestalt_rust::Output<String>,
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the AWS IAM Identity Center instance application; a subresource of the original Identity Center instance.
        pub identity_center_application_arn: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the AWS IAM Identity Center instance associated with the S3 Access Grants instance.
        pub identity_center_arn: pulumi_gestalt_rust::Output<Option<String>>,
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
        args: AccessGrantsInstanceArgs,
    ) -> AccessGrantsInstanceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let identity_center_arn_binding = args
            .identity_center_arn
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:s3control/accessGrantsInstance:AccessGrantsInstance".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "identityCenterArn".into(),
                    value: &identity_center_arn_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AccessGrantsInstanceResult {
            access_grants_instance_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accessGrantsInstanceArn"),
            ),
            access_grants_instance_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accessGrantsInstanceId"),
            ),
            account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            identity_center_application_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identityCenterApplicationArn"),
            ),
            identity_center_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identityCenterArn"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
