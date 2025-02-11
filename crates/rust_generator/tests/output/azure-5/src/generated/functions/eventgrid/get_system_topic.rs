#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_system_topic {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSystemTopicArgs {
        /// The name of the EventGrid System Topic resource.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the resource group in which the EventGrid System Topic exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetSystemTopicResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// An `identity` block as defined below, which contains the Managed Service Identity information for this Event Grid System Topic.
        pub identities: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::eventgrid::GetSystemTopicIdentity>,
        >,
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The Metric ARM Resource ID of the Event Grid System Topic.
        pub metric_arm_resource_id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Event Grid System Topic ARM Source.
        pub source_arm_resource_id: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which are assigned to the Event Grid System Topic.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The Topic Type of the Event Grid System Topic.
        pub topic_type: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetSystemTopicArgs,
    ) -> GetSystemTopicResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:eventgrid/getSystemTopic:getSystemTopic".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetSystemTopicResult {
            id: o.get_field("id"),
            identities: o.get_field("identities"),
            location: o.get_field("location"),
            metric_arm_resource_id: o.get_field("metricArmResourceId"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            source_arm_resource_id: o.get_field("sourceArmResourceId"),
            tags: o.get_field("tags"),
            topic_type: o.get_field("topicType"),
        }
    }
}
