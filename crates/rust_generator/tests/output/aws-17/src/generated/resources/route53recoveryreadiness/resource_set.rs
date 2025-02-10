/// Provides an AWS Route 53 Recovery Readiness Resource Set.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_set::create(
///         "example",
///         ResourceSetArgs::builder()
///             .resource_set_name("${[\"my-cw-alarm-set\"]}")
///             .resource_set_type("AWS::CloudWatch::Alarm")
///             .resources(
///                 vec![
///                     ResourceSetResource::builder()
///                     .resourceArn("${exampleAwsCloudwatchMetricAlarm.arn}")
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Route53 Recovery Readiness resource set name using the resource set name. For example:
///
/// ```sh
/// $ pulumi import aws:route53recoveryreadiness/resourceSet:ResourceSet my-cw-alarm-set example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod resource_set {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourceSetArgs {
        /// Unique name describing the resource set.
        #[builder(into)]
        pub resource_set_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Type of the resources in the resource set.
        #[builder(into)]
        pub resource_set_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// List of resources to add to this resource set. See below.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub resources: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::route53recoveryreadiness::ResourceSetResource>,
        >,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ResourceSetResult {
        /// ARN of the resource set
        /// * `resources.#.component_id` - Unique identified for DNS Target Resources, use for readiness checks.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Unique name describing the resource set.
        pub resource_set_name: pulumi_gestalt_rust::Output<String>,
        /// Type of the resources in the resource set.
        pub resource_set_type: pulumi_gestalt_rust::Output<String>,
        /// List of resources to add to this resource set. See below.
        ///
        /// The following arguments are optional:
        pub resources: pulumi_gestalt_rust::Output<
            Vec<super::super::types::route53recoveryreadiness::ResourceSetResource>,
        >,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ResourceSetArgs,
    ) -> ResourceSetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let resource_set_name_binding = args.resource_set_name.get_output(context);
        let resource_set_type_binding = args.resource_set_type.get_output(context);
        let resources_binding = args.resources.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:route53recoveryreadiness/resourceSet:ResourceSet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceSetName".into(),
                    value: resource_set_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceSetType".into(),
                    value: resource_set_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resources".into(),
                    value: resources_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ResourceSetResult {
            arn: o.get_field("arn"),
            resource_set_name: o.get_field("resourceSetName"),
            resource_set_type: o.get_field("resourceSetType"),
            resources: o.get_field("resources"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
