#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_response_plan {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetResponsePlanArgs {
        /// The Amazon Resource Name (ARN) of the response plan.
        #[builder(into)]
        pub arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The tags applied to the response plan.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetResponsePlanResult {
        /// (Optional) The actions that the response plan starts at the beginning of an incident.
        pub actions: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ssmincidents::GetResponsePlanAction>,
        >,
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The Chatbot chat channel used for collaboration during an incident.
        pub chat_channels: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The long format of the response plan name. This field can contain spaces.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) for the contacts and escalation plans that the response plan engages during an incident.
        pub engagements: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub incident_templates: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::ssmincidents::GetResponsePlanIncidentTemplate,
            >,
        >,
        /// Information about third-party services integrated into the response plan. The following values are supported:
        pub integrations: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ssmincidents::GetResponsePlanIntegration>,
        >,
        /// The name of the PagerDuty configuration.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The tags applied to the response plan.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetResponsePlanArgs,
    ) -> GetResponsePlanResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let arn_binding = args.arn.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ssmincidents/getResponsePlan:getResponsePlan".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "arn".into(),
                    value: arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetResponsePlanResult {
            actions: o.get_field("actions"),
            arn: o.get_field("arn"),
            chat_channels: o.get_field("chatChannels"),
            display_name: o.get_field("displayName"),
            engagements: o.get_field("engagements"),
            id: o.get_field("id"),
            incident_templates: o.get_field("incidentTemplates"),
            integrations: o.get_field("integrations"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
        }
    }
}
