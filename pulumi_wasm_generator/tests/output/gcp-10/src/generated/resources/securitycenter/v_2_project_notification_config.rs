/// A Cloud Security Command Center (Cloud SCC) notification configs. A
/// notification config is a Cloud SCC resource that contains the
/// configuration to send notifications for create/update events of
/// findings, assets and etc.
/// > **Note:** In order to use Cloud SCC resources, your organization must be enrolled
/// in [SCC Standard/Premium](https://cloud.google.com/security-command-center/docs/quickstart-security-command-center).
/// Without doing so, you may run into errors during resource creation.
///
///
/// To get more information about ProjectNotificationConfig, see:
///
/// * [API documentation](https://cloud.google.com/security-command-center/docs/reference/rest/v2/projects.locations.notificationConfigs)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/security-command-center/docs)
///
/// ## Example Usage
///
/// ### Scc V2 Project Notification Config Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let customNotificationConfig = v_2_project_notification_config::create(
///         "customNotificationConfig",
///         V2ProjectNotificationConfigArgs::builder()
///             .config_id("my-config")
///             .description(
///                 "My custom Cloud Security Command Center Finding Notification Configuration",
///             )
///             .location("global")
///             .project("my-project-name")
///             .pubsub_topic("${sccV2ProjectNotification.id}")
///             .streaming_config(
///                 V2ProjectNotificationConfigStreamingConfig::builder()
///                     .filter("category = \"OPEN_FIREWALL\" AND state = \"ACTIVE\"")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let sccV2ProjectNotification = topic::create(
///         "sccV2ProjectNotification",
///         TopicArgs::builder().name("my-topic").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ProjectNotificationConfig can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/notificationConfigs/{{config_id}}`
///
/// * `{{project}}/{{location}}/{{config_id}}`
///
/// * `{{location}}/{{config_id}}`
///
/// When using the `pulumi import` command, ProjectNotificationConfig can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:securitycenter/v2ProjectNotificationConfig:V2ProjectNotificationConfig default projects/{{project}}/locations/{{location}}/notificationConfigs/{{config_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:securitycenter/v2ProjectNotificationConfig:V2ProjectNotificationConfig default {{project}}/{{location}}/{{config_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:securitycenter/v2ProjectNotificationConfig:V2ProjectNotificationConfig default {{location}}/{{config_id}}
/// ```
///
pub mod v_2_project_notification_config {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct V2ProjectNotificationConfigArgs {
        /// This must be unique within the project.
        #[builder(into)]
        pub config_id: pulumi_wasm_rust::Output<String>,
        /// The description of the notification config (max of 1024 characters).
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Location ID of the parent organization. Only global is supported at the moment.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The Pub/Sub topic to send notifications to. Its format is "projects/[project_id]/topics/[topic]".
        #[builder(into, default)]
        pub pubsub_topic: pulumi_wasm_rust::Output<Option<String>>,
        /// The config for triggering streaming-based notifications.
        /// Structure is documented below.
        #[builder(into)]
        pub streaming_config: pulumi_wasm_rust::Output<
            super::super::types::securitycenter::V2ProjectNotificationConfigStreamingConfig,
        >,
    }
    #[allow(dead_code)]
    pub struct V2ProjectNotificationConfigResult {
        /// This must be unique within the project.
        pub config_id: pulumi_wasm_rust::Output<String>,
        /// The description of the notification config (max of 1024 characters).
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Location ID of the parent organization. Only global is supported at the moment.
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The resource name of this notification config, in the format
        /// `projects/{{projectId}}/locations/{{location}}/notificationConfigs/{{config_id}}`.
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<String>,
        /// The Pub/Sub topic to send notifications to. Its format is "projects/[project_id]/topics/[topic]".
        pub pubsub_topic: pulumi_wasm_rust::Output<Option<String>>,
        /// The service account that needs "pubsub.topics.publish" permission to
        /// publish to the Pub/Sub topic.
        pub service_account: pulumi_wasm_rust::Output<String>,
        /// The config for triggering streaming-based notifications.
        /// Structure is documented below.
        pub streaming_config: pulumi_wasm_rust::Output<
            super::super::types::securitycenter::V2ProjectNotificationConfigStreamingConfig,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: V2ProjectNotificationConfigArgs,
    ) -> V2ProjectNotificationConfigResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let config_id_binding = args.config_id.get_inner();
        let description_binding = args.description.get_inner();
        let location_binding = args.location.get_inner();
        let project_binding = args.project.get_inner();
        let pubsub_topic_binding = args.pubsub_topic.get_inner();
        let streaming_config_binding = args.streaming_config.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:securitycenter/v2ProjectNotificationConfig:V2ProjectNotificationConfig"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "configId".into(),
                    value: &config_id_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "pubsubTopic".into(),
                    value: &pubsub_topic_binding,
                },
                register_interface::ObjectField {
                    name: "streamingConfig".into(),
                    value: &streaming_config_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "configId".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pubsubTopic".into(),
                },
                register_interface::ResultField {
                    name: "serviceAccount".into(),
                },
                register_interface::ResultField {
                    name: "streamingConfig".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        V2ProjectNotificationConfigResult {
            config_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configId").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pubsub_topic: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pubsubTopic").unwrap(),
            ),
            service_account: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceAccount").unwrap(),
            ),
            streaming_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("streamingConfig").unwrap(),
            ),
        }
    }
}
