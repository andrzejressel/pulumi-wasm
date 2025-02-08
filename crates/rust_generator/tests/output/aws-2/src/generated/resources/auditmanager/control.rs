/// Resource for managing an AWS Audit Manager Control.
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
///     let example = control::create(
///         "example",
///         ControlArgs::builder()
///             .control_mapping_sources(
///                 vec![
///                     ControlControlMappingSource::builder().sourceName("example")
///                     .sourceSetUpOption("Procedural_Controls_Mapping")
///                     .sourceType("MANUAL").build_struct(),
///                 ],
///             )
///             .name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import an Audit Manager Control using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:auditmanager/control:Control example abc123-de45
/// ```
#[allow(clippy::doc_lazy_continuation)]
pub mod control {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ControlArgs {
        /// Recommended actions to carry out if the control isn't fulfilled.
        #[builder(into, default)]
        pub action_plan_instructions: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Title of the action plan for remediating the control.
        #[builder(into, default)]
        pub action_plan_title: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Data mapping sources. See `control_mapping_sources` below.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub control_mapping_sources: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::auditmanager::ControlControlMappingSource>>,
        >,
        /// Description of the control.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the control.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the control. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Steps to follow to determine if the control is satisfied.
        #[builder(into, default)]
        pub testing_information: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ControlResult {
        /// Recommended actions to carry out if the control isn't fulfilled.
        pub action_plan_instructions: pulumi_gestalt_rust::Output<Option<String>>,
        /// Title of the action plan for remediating the control.
        pub action_plan_title: pulumi_gestalt_rust::Output<Option<String>>,
        /// Amazon Resource Name (ARN) of the control.
        /// * `control_mapping_sources.*.source_id` - Unique identifier for the source.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Data mapping sources. See `control_mapping_sources` below.
        ///
        /// The following arguments are optional:
        pub control_mapping_sources: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::auditmanager::ControlControlMappingSource>>,
        >,
        /// Description of the control.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the control.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the control. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Steps to follow to determine if the control is satisfied.
        pub testing_information: pulumi_gestalt_rust::Output<Option<String>>,
        /// Type of control, such as a custom control or a standard control.
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ControlArgs,
    ) -> ControlResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let action_plan_instructions_binding = args
            .action_plan_instructions
            .get_output(context)
            .get_inner();
        let action_plan_title_binding = args
            .action_plan_title
            .get_output(context)
            .get_inner();
        let control_mapping_sources_binding = args
            .control_mapping_sources
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let testing_information_binding = args
            .testing_information
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:auditmanager/control:Control".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "actionPlanInstructions".into(),
                    value: &action_plan_instructions_binding,
                },
                register_interface::ObjectField {
                    name: "actionPlanTitle".into(),
                    value: &action_plan_title_binding,
                },
                register_interface::ObjectField {
                    name: "controlMappingSources".into(),
                    value: &control_mapping_sources_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "testingInformation".into(),
                    value: &testing_information_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ControlResult {
            action_plan_instructions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("actionPlanInstructions"),
            ),
            action_plan_title: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("actionPlanTitle"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            control_mapping_sources: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("controlMappingSources"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            testing_information: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("testingInformation"),
            ),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}
