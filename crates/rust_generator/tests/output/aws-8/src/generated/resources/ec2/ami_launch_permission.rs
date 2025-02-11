/// Adds a launch permission to an Amazon Machine Image (AMI).
///
/// ## Example Usage
///
/// ### AWS Account ID
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = ami_launch_permission::create(
///         "example",
///         AmiLaunchPermissionArgs::builder()
///             .account_id("123456789012")
///             .image_id("ami-12345678")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Public Access
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = ami_launch_permission::create(
///         "example",
///         AmiLaunchPermissionArgs::builder()
///             .group("all")
///             .image_id("ami-12345678")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Organization Access
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ec2:AmiLaunchPermission
///     properties:
///       imageId: ami-12345678
///       organizationArn: ${current.arn}
/// variables:
///   current:
///     fn::invoke:
///       function: aws:organizations:getOrganization
///       arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import AMI Launch Permissions using `[ACCOUNT-ID|GROUP-NAME|ORGANIZATION-ARN|ORGANIZATIONAL-UNIT-ARN]/IMAGE-ID`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/amiLaunchPermission:AmiLaunchPermission example 123456789012/ami-12345678
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod ami_launch_permission {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AmiLaunchPermissionArgs {
        /// AWS account ID for the launch permission.
        #[builder(into, default)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the group for the launch permission. Valid values: `"all"`.
        #[builder(into, default)]
        pub group: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ID of the AMI.
        #[builder(into)]
        pub image_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ARN of an organization for the launch permission.
        #[builder(into, default)]
        pub organization_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ARN of an organizational unit for the launch permission.
        #[builder(into, default)]
        pub organizational_unit_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AmiLaunchPermissionResult {
        /// AWS account ID for the launch permission.
        pub account_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the group for the launch permission. Valid values: `"all"`.
        pub group: pulumi_gestalt_rust::Output<Option<String>>,
        /// ID of the AMI.
        pub image_id: pulumi_gestalt_rust::Output<String>,
        /// ARN of an organization for the launch permission.
        pub organization_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// ARN of an organizational unit for the launch permission.
        pub organizational_unit_arn: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AmiLaunchPermissionArgs,
    ) -> AmiLaunchPermissionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let group_binding = args.group.get_output(context);
        let image_id_binding = args.image_id.get_output(context);
        let organization_arn_binding = args.organization_arn.get_output(context);
        let organizational_unit_arn_binding = args
            .organizational_unit_arn
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/amiLaunchPermission:AmiLaunchPermission".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "group".into(),
                    value: &group_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "imageId".into(),
                    value: &image_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "organizationArn".into(),
                    value: &organization_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "organizationalUnitArn".into(),
                    value: &organizational_unit_arn_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AmiLaunchPermissionResult {
            account_id: o.get_field("accountId"),
            group: o.get_field("group"),
            image_id: o.get_field("imageId"),
            organization_arn: o.get_field("organizationArn"),
            organizational_unit_arn: o.get_field("organizationalUnitArn"),
        }
    }
}
