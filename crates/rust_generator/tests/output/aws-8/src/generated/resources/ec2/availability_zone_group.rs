/// Manages an EC2 Availability Zone Group, such as updating its opt-in status.
///
/// > **NOTE:** This is an advanced resource. The provider will automatically assume management of the EC2 Availability Zone Group without import and perform no actions on removal from configuration.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = availability_zone_group::create(
///         "example",
///         AvailabilityZoneGroupArgs::builder()
///             .group_name("us-west-2-lax-1")
///             .opt_in_status("opted-in")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import EC2 Availability Zone Groups using the group name. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/availabilityZoneGroup:AvailabilityZoneGroup example us-west-2-lax-1
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod availability_zone_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AvailabilityZoneGroupArgs {
        /// Name of the Availability Zone Group.
        #[builder(into)]
        pub group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Indicates whether to enable or disable Availability Zone Group. Valid values: `opted-in` or `not-opted-in`.
        #[builder(into)]
        pub opt_in_status: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AvailabilityZoneGroupResult {
        /// Name of the Availability Zone Group.
        pub group_name: pulumi_gestalt_rust::Output<String>,
        /// Indicates whether to enable or disable Availability Zone Group. Valid values: `opted-in` or `not-opted-in`.
        pub opt_in_status: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AvailabilityZoneGroupArgs,
    ) -> AvailabilityZoneGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let group_name_binding = args.group_name.get_output(context);
        let opt_in_status_binding = args.opt_in_status.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/availabilityZoneGroup:AvailabilityZoneGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "groupName".into(),
                    value: &group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "optInStatus".into(),
                    value: &opt_in_status_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AvailabilityZoneGroupResult {
            group_name: o.get_field("groupName"),
            opt_in_status: o.get_field("optInStatus"),
        }
    }
}
