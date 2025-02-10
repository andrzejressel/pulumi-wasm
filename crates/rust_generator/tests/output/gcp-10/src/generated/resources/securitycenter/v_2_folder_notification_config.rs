/// A Cloud Security Command Center (Cloud SCC) notification configs. A
/// notification config is a Cloud SCC resource that contains the
/// configuration to send notifications for create/update events of
/// findings, assets and etc.
/// > **Note:** In order to use Cloud SCC resources, your organization must be enrolled
/// in [SCC Standard/Premium](https://cloud.google.com/security-command-center/docs/quickstart-security-command-center).
/// Without doing so, you may run into errors during resource creation.
///
///
/// To get more information about FolderNotificationConfig, see:
///
/// * [API documentation](https://cloud.google.com/security-command-center/docs/reference/rest/v2/folders.locations.notificationConfigs)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/security-command-center/docs)
///
/// ## Example Usage
///
/// ### Scc V2 Folder Notification Config Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let customNotificationConfig = v_2_folder_notification_config::create(
///         "customNotificationConfig",
///         V2FolderNotificationConfigArgs::builder()
///             .config_id("my-config")
///             .description(
///                 "My custom Cloud Security Command Center Finding Notification Configuration",
///             )
///             .folder("${folder.folderId}")
///             .location("global")
///             .pubsub_topic("${sccV2FolderNotificationConfig.id}")
///             .streaming_config(
///                 V2FolderNotificationConfigStreamingConfig::builder()
///                     .filter("category = \"OPEN_FIREWALL\" AND state = \"ACTIVE\"")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let folder = folder::create(
///         "folder",
///         FolderArgs::builder()
///             .display_name("folder-name")
///             .parent("organizations/123456789")
///             .build_struct(),
///     );
///     let sccV2FolderNotificationConfig = topic::create(
///         "sccV2FolderNotificationConfig",
///         TopicArgs::builder().name("my-topic").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// FolderNotificationConfig can be imported using any of these accepted formats:
///
/// * `folders/{{folder}}/locations/{{location}}/notificationConfigs/{{config_id}}`
///
/// * `{{folder}}/{{location}}/{{config_id}}`
///
/// When using the `pulumi import` command, FolderNotificationConfig can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:securitycenter/v2FolderNotificationConfig:V2FolderNotificationConfig default folders/{{folder}}/locations/{{location}}/notificationConfigs/{{config_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:securitycenter/v2FolderNotificationConfig:V2FolderNotificationConfig default {{folder}}/{{location}}/{{config_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod v_2_folder_notification_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct V2FolderNotificationConfigArgs {
        /// This must be unique within the organization.
        #[builder(into)]
        pub config_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The description of the notification config (max of 1024 characters).
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Numerical ID of the parent folder.
        #[builder(into)]
        pub folder: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Location ID of the parent organization. If not provided, 'global' will be used as the default location.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Pub/Sub topic to send notifications to. Its format is
        /// "projects/[project_id]/topics/[topic]".
        #[builder(into)]
        pub pubsub_topic: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The config for triggering streaming-based notifications.
        /// Structure is documented below.
        #[builder(into)]
        pub streaming_config: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::securitycenter::V2FolderNotificationConfigStreamingConfig,
        >,
    }
    #[allow(dead_code)]
    pub struct V2FolderNotificationConfigResult {
        /// This must be unique within the organization.
        pub config_id: pulumi_gestalt_rust::Output<String>,
        /// The description of the notification config (max of 1024 characters).
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Numerical ID of the parent folder.
        pub folder: pulumi_gestalt_rust::Output<String>,
        /// Location ID of the parent organization. If not provided, 'global' will be used as the default location.
        pub location: pulumi_gestalt_rust::Output<Option<String>>,
        /// The resource name of this notification config, in the format
        /// `folders/{{folder}}/locations/{{location}}/notificationConfigs/{{config_id}}`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Pub/Sub topic to send notifications to. Its format is
        /// "projects/[project_id]/topics/[topic]".
        pub pubsub_topic: pulumi_gestalt_rust::Output<String>,
        /// The service account that needs "pubsub.topics.publish" permission to
        /// publish to the Pub/Sub topic.
        pub service_account: pulumi_gestalt_rust::Output<String>,
        /// The config for triggering streaming-based notifications.
        /// Structure is documented below.
        pub streaming_config: pulumi_gestalt_rust::Output<
            super::super::types::securitycenter::V2FolderNotificationConfigStreamingConfig,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: V2FolderNotificationConfigArgs,
    ) -> V2FolderNotificationConfigResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let config_id_binding = args.config_id.get_output(context);
        let description_binding = args.description.get_output(context);
        let folder_binding = args.folder.get_output(context);
        let location_binding = args.location.get_output(context);
        let pubsub_topic_binding = args.pubsub_topic.get_output(context);
        let streaming_config_binding = args.streaming_config.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:securitycenter/v2FolderNotificationConfig:V2FolderNotificationConfig"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configId".into(),
                    value: config_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "folder".into(),
                    value: folder_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pubsubTopic".into(),
                    value: pubsub_topic_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "streamingConfig".into(),
                    value: streaming_config_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        V2FolderNotificationConfigResult {
            config_id: o.get_field("configId"),
            description: o.get_field("description"),
            folder: o.get_field("folder"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            pubsub_topic: o.get_field("pubsubTopic"),
            service_account: o.get_field("serviceAccount"),
            streaming_config: o.get_field("streamingConfig"),
        }
    }
}
