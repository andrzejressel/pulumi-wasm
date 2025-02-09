#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_notification_channel {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNotificationChannelArgs {
        /// The display name for this notification channel.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Labels (corresponding to the
        /// NotificationChannelDescriptor schema) to filter the notification channels by.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The type of the notification channel.
        ///
        /// - - -
        ///
        /// Other optional fields include:
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// User-provided key-value labels to filter by.
        #[builder(into, default)]
        pub user_labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetNotificationChannelResult {
        /// An optional human-readable description of this notification channel.
        pub description: pulumi_gestalt_rust::Output<String>,
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether notifications are forwarded to the described channel.
        pub enabled: pulumi_gestalt_rust::Output<bool>,
        pub force_delete: pulumi_gestalt_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Configuration fields that define the channel and its behavior.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The full REST resource name for this channel. The syntax is:
        /// `projects/[PROJECT_ID]/notificationChannels/[CHANNEL_ID]`.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub sensitive_labels: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::monitoring::GetNotificationChannelSensitiveLabel,
            >,
        >,
        pub type_: pulumi_gestalt_rust::Output<Option<String>>,
        /// User-supplied key/value data that does not need to conform to the corresponding NotificationChannelDescriptor's schema, unlike the labels field.
        pub user_labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Indicates whether this channel has been verified or not.
        pub verification_status: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetNotificationChannelArgs,
    ) -> GetNotificationChannelResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let display_name_binding = args.display_name.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let project_binding = args.project.get_output(context);
        let type__binding = args.type_.get_output(context);
        let user_labels_binding = args.user_labels.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:monitoring/getNotificationChannel:getNotificationChannel".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: labels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userLabels".into(),
                    value: user_labels_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetNotificationChannelResult {
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            enabled: o.get_field("enabled"),
            force_delete: o.get_field("forceDelete"),
            id: o.get_field("id"),
            labels: o.get_field("labels"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            sensitive_labels: o.get_field("sensitiveLabels"),
            type_: o.get_field("type"),
            user_labels: o.get_field("userLabels"),
            verification_status: o.get_field("verificationStatus"),
        }
    }
}
