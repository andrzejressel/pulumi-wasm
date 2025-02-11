/// Resource for managing an AWS Resource Groups Resource.
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
///     let example = dedicated_host::create(
///         "example",
///         DedicatedHostArgs::builder()
///             .auto_placement("on")
///             .availability_zone("us-east-1a")
///             .host_recovery("off")
///             .instance_family("t3")
///             .build_struct(),
///     );
///     let exampleGroup = group::create(
///         "exampleGroup",
///         GroupArgs::builder().name("example").build_struct(),
///     );
///     let exampleResource = resource::create(
///         "exampleResource",
///         ResourceArgs::builder()
///             .group_arn("${exampleGroup.arn}")
///             .resource_arn("${example.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import an AWS Resource Groups Resource using `group_arn` and `resource_arn`, separated by a comma (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:resourcegroups/resource:Resource example arn:aws:resource-groups:us-west-2:012345678901:group/example,arn:aws:lambda:us-west-2:012345678901:function:example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod resource {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourceArgs {
        /// Name or ARN of the resource group to add resources to.
        #[builder(into)]
        pub group_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ARN of the resource to be added to the group.
        #[builder(into)]
        pub resource_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ResourceResult {
        /// Name or ARN of the resource group to add resources to.
        pub group_arn: pulumi_gestalt_rust::Output<String>,
        /// ARN of the resource to be added to the group.
        pub resource_arn: pulumi_gestalt_rust::Output<String>,
        /// The resource type of a resource, such as `AWS::EC2::Instance`.
        pub resource_type: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ResourceArgs,
    ) -> ResourceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let group_arn_binding = args.group_arn.get_output(context);
        let resource_arn_binding = args.resource_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:resourcegroups/resource:Resource".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "groupArn".into(),
                    value: &group_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceArn".into(),
                    value: &resource_arn_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ResourceResult {
            group_arn: o.get_field("groupArn"),
            resource_arn: o.get_field("resourceArn"),
            resource_type: o.get_field("resourceType"),
        }
    }
}
