/// A Cloud Security Command Center (Cloud SCC) notification configs. A
/// notification config is a Cloud SCC resource that contains the
/// configuration to send notifications for create/update events of
/// findings, assets and etc.
/// > **Note:** In order to use Cloud SCC resources, your organization must be enrolled
/// in [SCC Standard/Premium](https://cloud.google.com/security-command-center/docs/quickstart-security-command-center).
/// Without doing so, you may run into errors during resource creation.
///
///
/// To get more information about OrganizationNotificationConfig, see:
///
/// * [API documentation](https://cloud.google.com/security-command-center/docs/reference/rest/v2/organizations.locations.notificationConfigs)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/security-command-center/docs)
///
/// ## Example Usage
///
/// ### Scc V2 Organization Notification Config Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let customOrganizationNotificationConfig = v_2_organization_notification_config::create(
///         "customOrganizationNotificationConfig",
///         V2OrganizationNotificationConfigArgs::builder()
///             .config_id("my-config")
///             .description(
///                 "My custom Cloud Security Command Center Finding Organization Notification Configuration",
///             )
///             .location("global")
///             .organization("123456789")
///             .pubsub_topic("${sccV2OrganizationNotificationConfig.id}")
///             .streaming_config(
///                 V2OrganizationNotificationConfigStreamingConfig::builder()
///                     .filter("category = \"OPEN_FIREWALL\" AND state = \"ACTIVE\"")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let sccV2OrganizationNotificationConfig = topic::create(
///         "sccV2OrganizationNotificationConfig",
///         TopicArgs::builder().name("my-topic").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// OrganizationNotificationConfig can be imported using any of these accepted formats:
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, OrganizationNotificationConfig can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:securitycenter/v2OrganizationNotificationConfig:V2OrganizationNotificationConfig default {{name}}
/// ```
///
pub mod v_2_organization_notification_config {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct V2OrganizationNotificationConfigArgs {
        /// This must be unique within the organization.
        #[builder(into)]
        pub config_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The description of the notification config (max of 1024 characters).
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// location Id is provided by organization. If not provided, Use global as default.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The organization whose Cloud Security Command Center the Notification
        /// Config lives in.
        #[builder(into)]
        pub organization: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Pub/Sub topic to send notifications to. Its format is
        /// "projects/[project_id]/topics/[topic]".
        #[builder(into)]
        pub pubsub_topic: pulumi_wasm_rust::InputOrOutput<String>,
        /// The config for triggering streaming-based notifications.
        /// Structure is documented below.
        #[builder(into)]
        pub streaming_config: pulumi_wasm_rust::InputOrOutput<
            super::super::types::securitycenter::V2OrganizationNotificationConfigStreamingConfig,
        >,
    }
    #[allow(dead_code)]
    pub struct V2OrganizationNotificationConfigResult {
        /// This must be unique within the organization.
        pub config_id: pulumi_wasm_rust::Output<String>,
        /// The description of the notification config (max of 1024 characters).
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// location Id is provided by organization. If not provided, Use global as default.
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The resource name of this notification config, in the format
        /// `organizations/{{organization}}/notificationConfigs/{{config_id}}`.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The organization whose Cloud Security Command Center the Notification
        /// Config lives in.
        pub organization: pulumi_wasm_rust::Output<String>,
        /// The Pub/Sub topic to send notifications to. Its format is
        /// "projects/[project_id]/topics/[topic]".
        pub pubsub_topic: pulumi_wasm_rust::Output<String>,
        /// The service account that needs "pubsub.topics.publish" permission to
        /// publish to the Pub/Sub topic.
        pub service_account: pulumi_wasm_rust::Output<String>,
        /// The config for triggering streaming-based notifications.
        /// Structure is documented below.
        pub streaming_config: pulumi_wasm_rust::Output<
            super::super::types::securitycenter::V2OrganizationNotificationConfigStreamingConfig,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: V2OrganizationNotificationConfigArgs,
    ) -> V2OrganizationNotificationConfigResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let config_id_binding = args.config_id.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let organization_binding = args.organization.get_output(context).get_inner();
        let pubsub_topic_binding = args.pubsub_topic.get_output(context).get_inner();
        let streaming_config_binding = args
            .streaming_config
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:securitycenter/v2OrganizationNotificationConfig:V2OrganizationNotificationConfig"
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
                    name: "organization".into(),
                    value: &organization_binding,
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        V2OrganizationNotificationConfigResult {
            config_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("configId"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            organization: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("organization"),
            ),
            pubsub_topic: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pubsubTopic"),
            ),
            service_account: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serviceAccount"),
            ),
            streaming_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("streamingConfig"),
            ),
        }
    }
}
