/// A ScanConfig resource contains the configurations to launch a scan.
///
/// To get more information about ScanConfig, see:
///
/// * [API documentation](https://cloud.google.com/security-scanner/docs/reference/rest/v1beta/projects.scanConfigs)
/// * How-to Guides
///     * [Using Cloud Security Scanner](https://cloud.google.com/security-scanner/docs/scanning)
///
///
///
/// ## Example Usage
///
/// ### Scan Config Basic
///
///
/// ```yaml
/// resources:
///   scannerStaticIp:
///     type: gcp:compute:Address
///     name: scanner_static_ip
///     properties:
///       name: scan-basic-static-ip
///   scan-config:
///     type: gcp:compute:SecurityScanConfig
///     properties:
///       displayName: scan-config
///       startingUrls:
///         - http://${scannerStaticIp.address}
///       targetPlatforms:
///         - COMPUTE
/// ```
///
/// ## Import
///
/// ScanConfig can be imported using any of these accepted formats:
///
/// * `{{project}}/{{name}}`
///
/// * `{{project}} {{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, ScanConfig can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/securityScanConfig:SecurityScanConfig default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/securityScanConfig:SecurityScanConfig default "{{project}} {{name}}"
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/securityScanConfig:SecurityScanConfig default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod security_scan_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SecurityScanConfigArgs {
        /// The authentication configuration.
        /// If specified, service will use the authentication configuration during scanning.
        /// Structure is documented below.
        #[builder(into, default)]
        pub authentication: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::SecurityScanConfigAuthentication>,
        >,
        /// The blacklist URL patterns as described in
        /// https://cloud.google.com/security-scanner/docs/excluded-urls
        #[builder(into, default)]
        pub blacklist_patterns: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The user provider display name of the ScanConfig.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Controls export of scan configurations and results to Cloud Security Command Center.
        /// Default value is `ENABLED`.
        /// Possible values are: `ENABLED`, `DISABLED`.
        #[builder(into, default)]
        pub export_to_security_command_center: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The maximum QPS during scanning. A valid value ranges from 5 to 20 inclusively.
        /// Defaults to 15.
        #[builder(into, default)]
        pub max_qps: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The schedule of the ScanConfig
        /// Structure is documented below.
        #[builder(into, default)]
        pub schedule: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::SecurityScanConfigSchedule>,
        >,
        /// The starting URLs from which the scanner finds site pages.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub starting_urls: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// Set of Cloud Platforms targeted by the scan. If empty, APP_ENGINE will be used as a default.
        /// Each value may be one of: `APP_ENGINE`, `COMPUTE`.
        #[builder(into, default)]
        pub target_platforms: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Type of the user agents used for scanning
        /// Default value is `CHROME_LINUX`.
        /// Possible values are: `USER_AGENT_UNSPECIFIED`, `CHROME_LINUX`, `CHROME_ANDROID`, `SAFARI_IPHONE`.
        #[builder(into, default)]
        pub user_agent: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SecurityScanConfigResult {
        /// The authentication configuration.
        /// If specified, service will use the authentication configuration during scanning.
        /// Structure is documented below.
        pub authentication: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::SecurityScanConfigAuthentication>,
        >,
        /// The blacklist URL patterns as described in
        /// https://cloud.google.com/security-scanner/docs/excluded-urls
        pub blacklist_patterns: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The user provider display name of the ScanConfig.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// Controls export of scan configurations and results to Cloud Security Command Center.
        /// Default value is `ENABLED`.
        /// Possible values are: `ENABLED`, `DISABLED`.
        pub export_to_security_command_center: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// The maximum QPS during scanning. A valid value ranges from 5 to 20 inclusively.
        /// Defaults to 15.
        pub max_qps: pulumi_gestalt_rust::Output<Option<i32>>,
        /// A server defined name for this index. Format:
        /// `projects/{{project}}/scanConfigs/{{server_generated_id}}`
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The schedule of the ScanConfig
        /// Structure is documented below.
        pub schedule: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::SecurityScanConfigSchedule>,
        >,
        /// The starting URLs from which the scanner finds site pages.
        ///
        ///
        /// - - -
        pub starting_urls: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Set of Cloud Platforms targeted by the scan. If empty, APP_ENGINE will be used as a default.
        /// Each value may be one of: `APP_ENGINE`, `COMPUTE`.
        pub target_platforms: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Type of the user agents used for scanning
        /// Default value is `CHROME_LINUX`.
        /// Possible values are: `USER_AGENT_UNSPECIFIED`, `CHROME_LINUX`, `CHROME_ANDROID`, `SAFARI_IPHONE`.
        pub user_agent: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SecurityScanConfigArgs,
    ) -> SecurityScanConfigResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let authentication_binding = args.authentication.get_output(context);
        let blacklist_patterns_binding = args.blacklist_patterns.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let export_to_security_command_center_binding = args
            .export_to_security_command_center
            .get_output(context);
        let max_qps_binding = args.max_qps.get_output(context);
        let project_binding = args.project.get_output(context);
        let schedule_binding = args.schedule.get_output(context);
        let starting_urls_binding = args.starting_urls.get_output(context);
        let target_platforms_binding = args.target_platforms.get_output(context);
        let user_agent_binding = args.user_agent.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/securityScanConfig:SecurityScanConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authentication".into(),
                    value: &authentication_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "blacklistPatterns".into(),
                    value: &blacklist_patterns_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "exportToSecurityCommandCenter".into(),
                    value: &export_to_security_command_center_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maxQps".into(),
                    value: &max_qps_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "schedule".into(),
                    value: &schedule_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "startingUrls".into(),
                    value: &starting_urls_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetPlatforms".into(),
                    value: &target_platforms_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userAgent".into(),
                    value: &user_agent_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SecurityScanConfigResult {
            authentication: o.get_field("authentication"),
            blacklist_patterns: o.get_field("blacklistPatterns"),
            display_name: o.get_field("displayName"),
            export_to_security_command_center: o
                .get_field("exportToSecurityCommandCenter"),
            max_qps: o.get_field("maxQps"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            schedule: o.get_field("schedule"),
            starting_urls: o.get_field("startingUrls"),
            target_platforms: o.get_field("targetPlatforms"),
            user_agent: o.get_field("userAgent"),
        }
    }
}
