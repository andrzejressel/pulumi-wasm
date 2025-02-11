#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_notification_channel {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNotificationChannelArgs {
        /// Filter configurations for the Amazon SNS notification topic. See the `filters` attribute reference below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::super::types::devopsguru::GetNotificationChannelFilter>,
            >,
        >,
        /// Unique identifier for the notification channel.
        #[builder(into)]
        pub id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// SNS noficiation channel configurations. See the `sns` attribute reference below.
        #[builder(into, default)]
        pub sns: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::devopsguru::GetNotificationChannelSn>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetNotificationChannelResult {
        /// Filter configurations for the Amazon SNS notification topic. See the `filters` attribute reference below.
        pub filters: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::super::types::devopsguru::GetNotificationChannelFilter>,
            >,
        >,
        pub id: pulumi_gestalt_rust::Output<String>,
        /// SNS noficiation channel configurations. See the `sns` attribute reference below.
        pub sns: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::devopsguru::GetNotificationChannelSn>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetNotificationChannelArgs,
    ) -> GetNotificationChannelResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filters_binding = args.filters.get_output(context);
        let id_binding = args.id.get_output(context);
        let sns_binding = args.sns.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:devopsguru/getNotificationChannel:getNotificationChannel".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: &filters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "id".into(),
                    value: &id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sns".into(),
                    value: &sns_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetNotificationChannelResult {
            filters: o.get_field("filters"),
            id: o.get_field("id"),
            sns: o.get_field("sns"),
        }
    }
}
