/// Resource for managing an AWS Audit Manager Control.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod control {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ControlArgs {
        /// Recommended actions to carry out if the control isn't fulfilled.
        #[builder(into, default)]
        pub action_plan_instructions: pulumi_wasm_rust::Output<Option<String>>,
        /// Title of the action plan for remediating the control.
        #[builder(into, default)]
        pub action_plan_title: pulumi_wasm_rust::Output<Option<String>>,
        /// Data mapping sources. See `control_mapping_sources` below.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub control_mapping_sources: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::auditmanager::ControlControlMappingSource>>,
        >,
        /// Description of the control.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the control.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the control. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Steps to follow to determine if the control is satisfied.
        #[builder(into, default)]
        pub testing_information: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ControlResult {
        /// Recommended actions to carry out if the control isn't fulfilled.
        pub action_plan_instructions: pulumi_wasm_rust::Output<Option<String>>,
        /// Title of the action plan for remediating the control.
        pub action_plan_title: pulumi_wasm_rust::Output<Option<String>>,
        /// Amazon Resource Name (ARN) of the control.
        /// * `control_mapping_sources.*.source_id` - Unique identifier for the source.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Data mapping sources. See `control_mapping_sources` below.
        ///
        /// The following arguments are optional:
        pub control_mapping_sources: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::auditmanager::ControlControlMappingSource>>,
        >,
        /// Description of the control.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the control.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the control. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Steps to follow to determine if the control is satisfied.
        pub testing_information: pulumi_wasm_rust::Output<Option<String>>,
        /// Type of control, such as a custom control or a standard control.
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ControlArgs) -> ControlResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let action_plan_instructions_binding = args.action_plan_instructions.get_inner();
        let action_plan_title_binding = args.action_plan_title.get_inner();
        let control_mapping_sources_binding = args.control_mapping_sources.get_inner();
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let testing_information_binding = args.testing_information.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:auditmanager/control:Control".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "actionPlanInstructions".into(),
                },
                register_interface::ResultField {
                    name: "actionPlanTitle".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "controlMappingSources".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "testingInformation".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ControlResult {
            action_plan_instructions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("actionPlanInstructions").unwrap(),
            ),
            action_plan_title: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("actionPlanTitle").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            control_mapping_sources: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("controlMappingSources").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            testing_information: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("testingInformation").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
