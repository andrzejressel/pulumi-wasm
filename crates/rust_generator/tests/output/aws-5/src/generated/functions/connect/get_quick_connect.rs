#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_quick_connect {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetQuickConnectArgs {
        /// Reference to the hosting Amazon Connect Instance
        #[builder(into)]
        pub instance_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Returns information on a specific Quick Connect by name
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Returns information on a specific Quick Connect by Quick Connect id
        #[builder(into, default)]
        pub quick_connect_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags to assign to the Quick Connect.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetQuickConnectResult {
        /// ARN of the Quick Connect.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Description of the Quick Connect.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub instance_id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A block that defines the configuration information for the Quick Connect: `quick_connect_type` and one of `phone_config`, `queue_config`, `user_config` . The Quick Connect Config block is documented below.
        pub quick_connect_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::connect::GetQuickConnectQuickConnectConfig>,
        >,
        /// Identifier for the Quick Connect.
        pub quick_connect_id: pulumi_gestalt_rust::Output<String>,
        /// Map of tags to assign to the Quick Connect.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetQuickConnectArgs,
    ) -> GetQuickConnectResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let instance_id_binding = args.instance_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let quick_connect_id_binding = args.quick_connect_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:connect/getQuickConnect:getQuickConnect".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceId".into(),
                    value: instance_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "quickConnectId".into(),
                    value: quick_connect_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetQuickConnectResult {
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            instance_id: o.get_field("instanceId"),
            name: o.get_field("name"),
            quick_connect_configs: o.get_field("quickConnectConfigs"),
            quick_connect_id: o.get_field("quickConnectId"),
            tags: o.get_field("tags"),
        }
    }
}
