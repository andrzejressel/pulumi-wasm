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
pub mod stack {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StackArgs {
        /// Set of configuration blocks defining the interface VPC endpoints. Users of the stack can connect to AppStream 2.0 only through the specified endpoints.
        /// See `access_endpoints` below.
        #[builder(into, default)]
        pub access_endpoints: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::appstream::StackAccessEndpoint>>,
        >,
        /// Settings for application settings persistence.
        /// See `application_settings` below.
        #[builder(into, default)]
        pub application_settings: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::appstream::StackApplicationSettings>,
        >,
        /// Description for the AppStream stack.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Stack name to display.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Domains where AppStream 2.0 streaming sessions can be embedded in an iframe. You must approve the domains that you want to host embedded AppStream 2.0 streaming sessions.
        #[builder(into, default)]
        pub embed_host_domains: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// URL that users are redirected to after they click the Send Feedback link. If no URL is specified, no Send Feedback link is displayed. .
        #[builder(into, default)]
        pub feedback_url: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Unique name for the AppStream stack.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// URL that users are redirected to after their streaming session ends.
        #[builder(into, default)]
        pub redirect_url: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Configuration block for the storage connectors to enable.
        /// See `storage_connectors` below.
        #[builder(into, default)]
        pub storage_connectors: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::appstream::StackStorageConnector>>,
        >,
        /// The streaming protocol you want your stack to prefer. This can be UDP or TCP. Currently, UDP is only supported in the Windows native client.
        /// See `streaming_experience_settings` below.
        #[builder(into, default)]
        pub streaming_experience_settings: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::appstream::StackStreamingExperienceSettings>,
        >,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration block for the actions that are enabled or disabled for users during their streaming sessions. If not provided, these settings are configured automatically by AWS. If provided, the configuration should include a block for each configurable action.
        /// See `user_settings` below.
        #[builder(into, default)]
        pub user_settings: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::appstream::StackUserSetting>>,
        >,
    }
    #[allow(dead_code)]
    pub struct StackResult {
        /// Set of configuration blocks defining the interface VPC endpoints. Users of the stack can connect to AppStream 2.0 only through the specified endpoints.
        /// See `access_endpoints` below.
        pub access_endpoints: pulumi_wasm_rust::Output<
            Vec<super::super::types::appstream::StackAccessEndpoint>,
        >,
        /// Settings for application settings persistence.
        /// See `application_settings` below.
        pub application_settings: pulumi_wasm_rust::Output<
            super::super::types::appstream::StackApplicationSettings,
        >,
        /// ARN of the appstream stack.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Date and time, in UTC and extended RFC 3339 format, when the stack was created.
        pub created_time: pulumi_wasm_rust::Output<String>,
        /// Description for the AppStream stack.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Stack name to display.
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Domains where AppStream 2.0 streaming sessions can be embedded in an iframe. You must approve the domains that you want to host embedded AppStream 2.0 streaming sessions.
        pub embed_host_domains: pulumi_wasm_rust::Output<Vec<String>>,
        /// URL that users are redirected to after they click the Send Feedback link. If no URL is specified, no Send Feedback link is displayed. .
        pub feedback_url: pulumi_wasm_rust::Output<String>,
        /// Unique name for the AppStream stack.
        ///
        /// The following arguments are optional:
        pub name: pulumi_wasm_rust::Output<String>,
        /// URL that users are redirected to after their streaming session ends.
        pub redirect_url: pulumi_wasm_rust::Output<String>,
        /// Configuration block for the storage connectors to enable.
        /// See `storage_connectors` below.
        pub storage_connectors: pulumi_wasm_rust::Output<
            Vec<super::super::types::appstream::StackStorageConnector>,
        >,
        /// The streaming protocol you want your stack to prefer. This can be UDP or TCP. Currently, UDP is only supported in the Windows native client.
        /// See `streaming_experience_settings` below.
        pub streaming_experience_settings: pulumi_wasm_rust::Output<
            super::super::types::appstream::StackStreamingExperienceSettings,
        >,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Configuration block for the actions that are enabled or disabled for users during their streaming sessions. If not provided, these settings are configured automatically by AWS. If provided, the configuration should include a block for each configurable action.
        /// See `user_settings` below.
        pub user_settings: pulumi_wasm_rust::Output<
            Vec<super::super::types::appstream::StackUserSetting>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: StackArgs,
    ) -> StackResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_endpoints_binding = args
            .access_endpoints
            .get_output(context)
            .get_inner();
        let application_settings_binding = args
            .application_settings
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let embed_host_domains_binding = args
            .embed_host_domains
            .get_output(context)
            .get_inner();
        let feedback_url_binding = args.feedback_url.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let redirect_url_binding = args.redirect_url.get_output(context).get_inner();
        let storage_connectors_binding = args
            .storage_connectors
            .get_output(context)
            .get_inner();
        let streaming_experience_settings_binding = args
            .streaming_experience_settings
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let user_settings_binding = args.user_settings.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:appstream/stack:Stack".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessEndpoints".into(),
                    value: &access_endpoints_binding,
                },
                register_interface::ObjectField {
                    name: "applicationSettings".into(),
                    value: &application_settings_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "embedHostDomains".into(),
                    value: &embed_host_domains_binding,
                },
                register_interface::ObjectField {
                    name: "feedbackUrl".into(),
                    value: &feedback_url_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "redirectUrl".into(),
                    value: &redirect_url_binding,
                },
                register_interface::ObjectField {
                    name: "storageConnectors".into(),
                    value: &storage_connectors_binding,
                },
                register_interface::ObjectField {
                    name: "streamingExperienceSettings".into(),
                    value: &streaming_experience_settings_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "userSettings".into(),
                    value: &user_settings_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessEndpoints".into(),
                },
                register_interface::ResultField {
                    name: "applicationSettings".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "createdTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "embedHostDomains".into(),
                },
                register_interface::ResultField {
                    name: "feedbackUrl".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "redirectUrl".into(),
                },
                register_interface::ResultField {
                    name: "storageConnectors".into(),
                },
                register_interface::ResultField {
                    name: "streamingExperienceSettings".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "userSettings".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        StackResult {
            access_endpoints: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessEndpoints").unwrap(),
            ),
            application_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationSettings").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            created_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            embed_host_domains: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("embedHostDomains").unwrap(),
            ),
            feedback_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("feedbackUrl").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            redirect_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("redirectUrl").unwrap(),
            ),
            storage_connectors: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageConnectors").unwrap(),
            ),
            streaming_experience_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("streamingExperienceSettings").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            user_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userSettings").unwrap(),
            ),
        }
    }
}
