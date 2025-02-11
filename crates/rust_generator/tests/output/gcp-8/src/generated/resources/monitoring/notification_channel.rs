/// A NotificationChannel is a medium through which an alert is delivered
/// when a policy violation is detected. Examples of channels include email, SMS,
/// and third-party messaging applications. Fields containing sensitive information
/// like authentication tokens or contact info are only partially populated on retrieval.
///
/// Notification Channels are designed to be flexible and are made up of a supported `type`
/// and labels to configure that channel. Each `type` has specific labels that need to be
/// present for that channel to be correctly configured. The labels that are required to be
/// present for one channel `type` are often different than those required for another.
/// Due to these loose constraints it's often best to set up a channel through the UI
/// and import it to the provider when setting up a brand new channel type to determine which
/// labels are required.
///
/// A list of supported channels per project the `list` endpoint can be
/// accessed programmatically or through the api explorer at  https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.notificationChannelDescriptors/list .
/// This provides the channel type and all of the required labels that must be passed.
///
///
/// To get more information about NotificationChannel, see:
///
/// * [API documentation](https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.notificationChannels)
/// * How-to Guides
///     * [Monitoring API Documentation](https://cloud.google.com/monitoring/api/v3/)
///     * [Notification Options](https://cloud.google.com/monitoring/support/notification-options)
///
///
///
/// ## Example Usage
///
/// ### Notification Channel Basic
///
///
/// ```yaml
/// resources:
///   basic:
///     type: gcp:monitoring:NotificationChannel
///     properties:
///       displayName: Test Notification Channel
///       type: email
///       labels:
///         email_address: fake_email@blahblah.com
///       forceDelete: false
/// ```
/// ### Notification Channel Sensitive
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:monitoring:NotificationChannel
///     properties:
///       displayName: Test Slack Channel
///       type: slack
///       labels:
///         channel_name: '#foobar'
///       sensitiveLabels:
///         authToken: one
/// ```
///
/// ## Import
///
/// NotificationChannel can be imported using any of these accepted formats:
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, NotificationChannel can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:monitoring/notificationChannel:NotificationChannel default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod notification_channel {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NotificationChannelArgs {
        /// An optional human-readable description of this notification channel. This description may provide additional details, beyond the display name, for the channel. This may not exceed 1024 Unicode characters.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An optional human-readable name for this notification channel. It is recommended that you specify a non-empty and unique name in order to make it easier to identify the channels in your project, though this is not enforced. The display name is limited to 512 Unicode characters.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether notifications are forwarded to the described channel. This makes it possible to disable delivery of notifications to a particular channel without removing the channel from all alerting policies that reference the channel. This is a more convenient approach when the change is temporary and you want to receive notifications from the same set of alerting policies on the channel at some point in the future.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// If true, the notification channel will be deleted regardless
        /// of its use in alert policies (the policies will be updated
        /// to remove the channel). If false, channels that are still
        /// referenced by an existing alerting policy will fail to be
        /// deleted in a delete operation.
        #[builder(into, default)]
        pub force_delete: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Configuration fields that define the channel and its behavior. The
        /// permissible and required labels are specified in the
        /// NotificationChannelDescriptor corresponding to the type field.
        /// Labels with sensitive data are obfuscated by the API and therefore the provider cannot
        /// determine if there are upstream changes to these fields. They can also be configured via
        /// the sensitive_labels block, but cannot be configured in both places.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Different notification type behaviors are configured primarily using the the `labels` field on this
        /// resource. This block contains the labels which contain secrets or passwords so that they can be marked
        /// sensitive and hidden from plan output. The name of the field, eg: password, will be the key
        /// in the `labels` map in the api request.
        /// Credentials may not be specified in both locations and will cause an error. Changing from one location
        /// to a different credential configuration in the config will require an apply to update state.
        /// Structure is documented below.
        #[builder(into, default)]
        pub sensitive_labels: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::monitoring::NotificationChannelSensitiveLabels>,
        >,
        /// The type of the notification channel. This field matches the value of the NotificationChannelDescriptor.type field. See https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.notificationChannelDescriptors/list to get the list of valid values such as "email", "slack", etc...
        ///
        ///
        /// - - -
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
        /// User-supplied key/value data that does not need to conform to the corresponding NotificationChannelDescriptor's schema, unlike the labels field. This field is intended to be used for organizing and identifying the NotificationChannel objects.The field can contain up to 64 entries. Each key and value is limited to 63 Unicode characters or 128 bytes, whichever is smaller. Labels and values can contain only lowercase letters, numerals, underscores, and dashes. Keys must begin with a letter.
        #[builder(into, default)]
        pub user_labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct NotificationChannelResult {
        /// An optional human-readable description of this notification channel. This description may provide additional details, beyond the display name, for the channel. This may not exceed 1024 Unicode characters.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// An optional human-readable name for this notification channel. It is recommended that you specify a non-empty and unique name in order to make it easier to identify the channels in your project, though this is not enforced. The display name is limited to 512 Unicode characters.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether notifications are forwarded to the described channel. This makes it possible to disable delivery of notifications to a particular channel without removing the channel from all alerting policies that reference the channel. This is a more convenient approach when the change is temporary and you want to receive notifications from the same set of alerting policies on the channel at some point in the future.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// If true, the notification channel will be deleted regardless
        /// of its use in alert policies (the policies will be updated
        /// to remove the channel). If false, channels that are still
        /// referenced by an existing alerting policy will fail to be
        /// deleted in a delete operation.
        pub force_delete: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Configuration fields that define the channel and its behavior. The
        /// permissible and required labels are specified in the
        /// NotificationChannelDescriptor corresponding to the type field.
        /// Labels with sensitive data are obfuscated by the API and therefore the provider cannot
        /// determine if there are upstream changes to these fields. They can also be configured via
        /// the sensitive_labels block, but cannot be configured in both places.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The full REST resource name for this channel. The syntax is:
        /// projects/[PROJECT_ID]/notificationChannels/[CHANNEL_ID]
        /// The [CHANNEL_ID] is automatically assigned by the server on creation.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Different notification type behaviors are configured primarily using the the `labels` field on this
        /// resource. This block contains the labels which contain secrets or passwords so that they can be marked
        /// sensitive and hidden from plan output. The name of the field, eg: password, will be the key
        /// in the `labels` map in the api request.
        /// Credentials may not be specified in both locations and will cause an error. Changing from one location
        /// to a different credential configuration in the config will require an apply to update state.
        /// Structure is documented below.
        pub sensitive_labels: pulumi_gestalt_rust::Output<
            Option<super::super::types::monitoring::NotificationChannelSensitiveLabels>,
        >,
        /// The type of the notification channel. This field matches the value of the NotificationChannelDescriptor.type field. See https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.notificationChannelDescriptors/list to get the list of valid values such as "email", "slack", etc...
        ///
        ///
        /// - - -
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// User-supplied key/value data that does not need to conform to the corresponding NotificationChannelDescriptor's schema, unlike the labels field. This field is intended to be used for organizing and identifying the NotificationChannel objects.The field can contain up to 64 entries. Each key and value is limited to 63 Unicode characters or 128 bytes, whichever is smaller. Labels and values can contain only lowercase letters, numerals, underscores, and dashes. Keys must begin with a letter.
        pub user_labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Indicates whether this channel has been verified or not. On a ListNotificationChannels or GetNotificationChannel operation, this field is expected to be populated.If the value is UNVERIFIED, then it indicates that the channel is non-functioning (it both requires verification and lacks verification); otherwise, it is assumed that the channel works.If the channel is neither VERIFIED nor UNVERIFIED, it implies that the channel is of a type that does not require verification or that this specific channel has been exempted from verification because it was created prior to verification being required for channels of this type.This field cannot be modified using a standard UpdateNotificationChannel operation. To change the value of this field, you must call VerifyNotificationChannel.
        pub verification_status: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NotificationChannelArgs,
    ) -> NotificationChannelResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let force_delete_binding = args.force_delete.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let project_binding = args.project.get_output(context);
        let sensitive_labels_binding = args.sensitive_labels.get_output(context);
        let type__binding = args.type_.get_output(context);
        let user_labels_binding = args.user_labels.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:monitoring/notificationChannel:NotificationChannel".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "forceDelete".into(),
                    value: &force_delete_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sensitiveLabels".into(),
                    value: &sensitive_labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userLabels".into(),
                    value: &user_labels_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        NotificationChannelResult {
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            enabled: o.get_field("enabled"),
            force_delete: o.get_field("forceDelete"),
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
