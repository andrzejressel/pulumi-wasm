/// Provides an AppStream stack.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:appstream:Stack
///     properties:
///       name: stack name
///       description: stack description
///       displayName: stack display name
///       feedbackUrl: http://your-domain/feedback
///       redirectUrl: http://your-domain/redirect
///       storageConnectors:
///         - connectorType: HOMEFOLDERS
///       userSettings:
///         - action: AUTO_TIME_ZONE_REDIRECTION
///           permission: DISABLED
///         - action: CLIPBOARD_COPY_FROM_LOCAL_DEVICE
///           permission: ENABLED
///         - action: CLIPBOARD_COPY_TO_LOCAL_DEVICE
///           permission: ENABLED
///         - action: DOMAIN_PASSWORD_SIGNIN
///           permission: ENABLED
///         - action: DOMAIN_SMART_CARD_SIGNIN
///           permission: DISABLED
///         - action: FILE_DOWNLOAD
///           permission: ENABLED
///         - action: FILE_UPLOAD
///           permission: ENABLED
///         - action: PRINTING_TO_LOCAL_DEVICE
///           permission: ENABLED
///       applicationSettings:
///         enabled: true
///         settingsGroup: SettingsGroup
///       tags:
///         TagName: TagValue
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_appstream_stack` using the id. For example:
///
/// ```sh
/// $ pulumi import aws:appstream/stack:Stack example stackID
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod stack {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StackArgs {
        /// Set of configuration blocks defining the interface VPC endpoints. Users of the stack can connect to AppStream 2.0 only through the specified endpoints.
        /// See `access_endpoints` below.
        #[builder(into, default)]
        pub access_endpoints: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::appstream::StackAccessEndpoint>>,
        >,
        /// Settings for application settings persistence.
        /// See `application_settings` below.
        #[builder(into, default)]
        pub application_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appstream::StackApplicationSettings>,
        >,
        /// Description for the AppStream stack.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Stack name to display.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Domains where AppStream 2.0 streaming sessions can be embedded in an iframe. You must approve the domains that you want to host embedded AppStream 2.0 streaming sessions.
        #[builder(into, default)]
        pub embed_host_domains: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// URL that users are redirected to after they click the Send Feedback link. If no URL is specified, no Send Feedback link is displayed. .
        #[builder(into, default)]
        pub feedback_url: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Unique name for the AppStream stack.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// URL that users are redirected to after their streaming session ends.
        #[builder(into, default)]
        pub redirect_url: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block for the storage connectors to enable.
        /// See `storage_connectors` below.
        #[builder(into, default)]
        pub storage_connectors: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::appstream::StackStorageConnector>>,
        >,
        /// The streaming protocol you want your stack to prefer. This can be UDP or TCP. Currently, UDP is only supported in the Windows native client.
        /// See `streaming_experience_settings` below.
        #[builder(into, default)]
        pub streaming_experience_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appstream::StackStreamingExperienceSettings>,
        >,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration block for the actions that are enabled or disabled for users during their streaming sessions. If not provided, these settings are configured automatically by AWS. If provided, the configuration should include a block for each configurable action.
        /// See `user_settings` below.
        #[builder(into, default)]
        pub user_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::appstream::StackUserSetting>>,
        >,
    }
    #[allow(dead_code)]
    pub struct StackResult {
        /// Set of configuration blocks defining the interface VPC endpoints. Users of the stack can connect to AppStream 2.0 only through the specified endpoints.
        /// See `access_endpoints` below.
        pub access_endpoints: pulumi_gestalt_rust::Output<
            Vec<super::super::types::appstream::StackAccessEndpoint>,
        >,
        /// Settings for application settings persistence.
        /// See `application_settings` below.
        pub application_settings: pulumi_gestalt_rust::Output<
            super::super::types::appstream::StackApplicationSettings,
        >,
        /// ARN of the appstream stack.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Date and time, in UTC and extended RFC 3339 format, when the stack was created.
        pub created_time: pulumi_gestalt_rust::Output<String>,
        /// Description for the AppStream stack.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Stack name to display.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Domains where AppStream 2.0 streaming sessions can be embedded in an iframe. You must approve the domains that you want to host embedded AppStream 2.0 streaming sessions.
        pub embed_host_domains: pulumi_gestalt_rust::Output<Vec<String>>,
        /// URL that users are redirected to after they click the Send Feedback link. If no URL is specified, no Send Feedback link is displayed. .
        pub feedback_url: pulumi_gestalt_rust::Output<String>,
        /// Unique name for the AppStream stack.
        ///
        /// The following arguments are optional:
        pub name: pulumi_gestalt_rust::Output<String>,
        /// URL that users are redirected to after their streaming session ends.
        pub redirect_url: pulumi_gestalt_rust::Output<String>,
        /// Configuration block for the storage connectors to enable.
        /// See `storage_connectors` below.
        pub storage_connectors: pulumi_gestalt_rust::Output<
            Vec<super::super::types::appstream::StackStorageConnector>,
        >,
        /// The streaming protocol you want your stack to prefer. This can be UDP or TCP. Currently, UDP is only supported in the Windows native client.
        /// See `streaming_experience_settings` below.
        pub streaming_experience_settings: pulumi_gestalt_rust::Output<
            super::super::types::appstream::StackStreamingExperienceSettings,
        >,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Configuration block for the actions that are enabled or disabled for users during their streaming sessions. If not provided, these settings are configured automatically by AWS. If provided, the configuration should include a block for each configurable action.
        /// See `user_settings` below.
        pub user_settings: pulumi_gestalt_rust::Output<
            Vec<super::super::types::appstream::StackUserSetting>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: StackArgs,
    ) -> StackResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let access_endpoints_binding = args.access_endpoints.get_output(context);
        let application_settings_binding = args.application_settings.get_output(context);
        let description_binding = args.description.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let embed_host_domains_binding = args.embed_host_domains.get_output(context);
        let feedback_url_binding = args.feedback_url.get_output(context);
        let name_binding = args.name.get_output(context);
        let redirect_url_binding = args.redirect_url.get_output(context);
        let storage_connectors_binding = args.storage_connectors.get_output(context);
        let streaming_experience_settings_binding = args
            .streaming_experience_settings
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let user_settings_binding = args.user_settings.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:appstream/stack:Stack".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accessEndpoints".into(),
                    value: &access_endpoints_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationSettings".into(),
                    value: &application_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "embedHostDomains".into(),
                    value: &embed_host_domains_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "feedbackUrl".into(),
                    value: &feedback_url_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "redirectUrl".into(),
                    value: &redirect_url_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageConnectors".into(),
                    value: &storage_connectors_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "streamingExperienceSettings".into(),
                    value: &streaming_experience_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userSettings".into(),
                    value: &user_settings_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        StackResult {
            access_endpoints: o.get_field("accessEndpoints"),
            application_settings: o.get_field("applicationSettings"),
            arn: o.get_field("arn"),
            created_time: o.get_field("createdTime"),
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            embed_host_domains: o.get_field("embedHostDomains"),
            feedback_url: o.get_field("feedbackUrl"),
            name: o.get_field("name"),
            redirect_url: o.get_field("redirectUrl"),
            storage_connectors: o.get_field("storageConnectors"),
            streaming_experience_settings: o.get_field("streamingExperienceSettings"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            user_settings: o.get_field("userSettings"),
        }
    }
}
