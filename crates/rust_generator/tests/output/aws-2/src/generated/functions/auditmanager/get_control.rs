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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetControlArgs,
    ) -> GetControlResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let control_mapping_sources_binding = args
            .control_mapping_sources
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:auditmanager/getControl:getControl".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "controlMappingSources".into(),
                    value: &control_mapping_sources_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetControlResult {
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
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            testing_information: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("testingInformation"),
            ),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}
