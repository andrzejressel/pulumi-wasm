#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_contact_flow {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetContactFlowArgs {
        /// Returns information on a specific Contact Flow by contact flow id
        #[builder(into, default)]
        pub contact_flow_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Reference to the hosting Amazon Connect Instance
        #[builder(into)]
        pub instance_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Returns information on a specific Contact Flow by name
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Tags to assign to the Contact Flow.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Type of Contact Flow.
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetContactFlowResult {
        /// ARN of the Contact Flow.
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub contact_flow_id: pulumi_gestalt_rust::Output<String>,
        /// Logic of the Contact Flow.
        pub content: pulumi_gestalt_rust::Output<String>,
        /// Description of the Contact Flow.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub instance_id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Tags to assign to the Contact Flow.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Type of Contact Flow.
        pub type_: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetContactFlowArgs,
    ) -> GetContactFlowResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let contact_flow_id_binding = args.contact_flow_id.get_output(context);
        let instance_id_binding = args.instance_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:connect/getContactFlow:getContactFlow".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contactFlowId".into(),
                    value: &contact_flow_id_binding.drop_type(),
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
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetContactFlowResult {
            arn: o.get_field("arn"),
            contact_flow_id: o.get_field("contactFlowId"),
            content: o.get_field("content"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            instance_id: o.get_field("instanceId"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
            type_: o.get_field("type"),
        }
    }
}
