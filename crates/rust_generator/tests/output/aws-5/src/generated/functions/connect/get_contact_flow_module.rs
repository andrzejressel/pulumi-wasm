#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_contact_flow_module {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetContactFlowModuleArgs {
        /// Returns information on a specific Contact Flow Module by contact flow module id
        #[builder(into, default)]
        pub contact_flow_module_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Reference to the hosting Amazon Connect Instance
        #[builder(into)]
        pub instance_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Returns information on a specific Contact Flow Module by name
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags to assign to the Contact Flow Module.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetContactFlowModuleResult {
        /// ARN of the Contact Flow Module.
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub contact_flow_module_id: pulumi_gestalt_rust::Output<String>,
        /// Logic of the Contact Flow Module.
        pub content: pulumi_gestalt_rust::Output<String>,
        /// Description of the Contact Flow Module.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub instance_id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Type of Contact Flow Module Module. Values are either `ACTIVE` or `ARCHIVED`.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Status of the Contact Flow Module Module. Values are either `PUBLISHED` or `SAVED`.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Map of tags to assign to the Contact Flow Module.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetContactFlowModuleArgs,
    ) -> GetContactFlowModuleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let contact_flow_module_id_binding = args
            .contact_flow_module_id
            .get_output(context);
        let instance_id_binding = args.instance_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:connect/getContactFlowModule:getContactFlowModule".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contactFlowModuleId".into(),
                    value: &contact_flow_module_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceId".into(),
                    value: &instance_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetContactFlowModuleResult {
            arn: o.get_field("arn"),
            contact_flow_module_id: o.get_field("contactFlowModuleId"),
            content: o.get_field("content"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            instance_id: o.get_field("instanceId"),
            name: o.get_field("name"),
            state: o.get_field("state"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
        }
    }
}
