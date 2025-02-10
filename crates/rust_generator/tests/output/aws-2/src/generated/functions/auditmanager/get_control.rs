#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_control {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetControlArgs {
        #[builder(into, default)]
        pub control_mapping_sources: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::super::types::auditmanager::GetControlControlMappingSource,
                >,
            >,
        >,
        /// Name of the control.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Type of control. Valid values are `Custom` and `Standard`.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetControlResult {
        pub action_plan_instructions: pulumi_gestalt_rust::Output<String>,
        pub action_plan_title: pulumi_gestalt_rust::Output<String>,
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub control_mapping_sources: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::super::types::auditmanager::GetControlControlMappingSource,
                >,
            >,
        >,
        pub description: pulumi_gestalt_rust::Output<String>,
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        pub testing_information: pulumi_gestalt_rust::Output<String>,
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetControlArgs,
    ) -> GetControlResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let control_mapping_sources_binding = args
            .control_mapping_sources
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:auditmanager/getControl:getControl".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "controlMappingSources".into(),
                    value: control_mapping_sources_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetControlResult {
            action_plan_instructions: o.get_field("actionPlanInstructions"),
            action_plan_title: o.get_field("actionPlanTitle"),
            arn: o.get_field("arn"),
            control_mapping_sources: o.get_field("controlMappingSources"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
            testing_information: o.get_field("testingInformation"),
            type_: o.get_field("type"),
        }
    }
}
