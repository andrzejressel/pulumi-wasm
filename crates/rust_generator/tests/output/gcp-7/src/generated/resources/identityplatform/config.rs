/// Identity Platform configuration for a Cloud project. Identity Platform is an
/// end-to-end authentication system for third-party users to access apps
/// and services.
///
/// This entity is created only once during intialization and cannot be deleted,
/// individual Identity Providers may be disabled instead.  This resource may only
/// be created in billing-enabled projects.
///
///
/// To get more information about Config, see:
///
/// * [API documentation](https://cloud.google.com/identity-platform/docs/reference/rest/v2/Config)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/identity-platform/docs)
///
///
///
/// ## Example Usage
///
/// ### Identity Platform Config Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:organizations:Project
///     properties:
///       projectId: my-project
///       name: my-project
///       orgId: '123456789'
///       billingAccount: 000000-0000000-0000000-000000
///       deletionPolicy: DELETE
///       labels:
///         firebase: enabled
///   identitytoolkit:
///     type: gcp:projects:Service
///     properties:
///       project: ${default.projectId}
///       service: identitytoolkit.googleapis.com
///   defaultConfig:
///     type: gcp:identityplatform:Config
///     name: default
///     properties:
///       project: ${default.projectId}
///       autodeleteAnonymousUsers: true
///       signIn:
///         allowDuplicateEmails: true
///         anonymous:
///           enabled: true
///         email:
///           enabled: true
///           passwordRequired: false
///         phoneNumber:
///           enabled: true
///           testPhoneNumbers:
///             '+11231231234': '000000'
///       smsRegionConfig:
///         allowlistOnly:
///           allowedRegions:
///             - US
///             - CA
///       blockingFunctions:
///         triggers:
///           - eventType: beforeSignIn
///             functionUri: https://us-east1-my-project.cloudfunctions.net/before-sign-in
///         forwardInboundCredentials:
///           refreshToken: true
///           accessToken: true
///           idToken: true
///       quota:
///         signUpQuotaConfig:
///           quota: 1000
///           startTime: 2014-10-02T15:01:23Z
///           quotaDuration: 7200s
///       authorizedDomains:
///         - localhost
///         - my-project.firebaseapp.com
///         - my-project.web.app
/// ```
///
/// ## Import
///
/// Config can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/config`
///
/// * `projects/{{project}}`
///
/// * `{{project}}`
///
/// When using the `pulumi import` command, Config can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:identityplatform/config:Config default projects/{{project}}/config
/// ```
///
/// ```sh
/// $ pulumi import gcp:identityplatform/config:Config default projects/{{project}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:identityplatform/config:Config default {{project}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConfigArgs {
        /// List of domains authorized for OAuth redirects.
        #[builder(into, default)]
        pub authorized_domains: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Whether anonymous users will be auto-deleted after a period of 30 days
        #[builder(into, default)]
        pub autodelete_anonymous_users: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Configuration related to blocking functions.
        /// Structure is documented below.
        #[builder(into, default)]
        pub blocking_functions: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::identityplatform::ConfigBlockingFunctions>,
        >,
        /// Options related to how clients making requests on behalf of a project should be configured.
        /// Structure is documented below.
        #[builder(into, default)]
        pub client: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::identityplatform::ConfigClient>,
        >,
        /// Options related to how clients making requests on behalf of a project should be configured.
        /// Structure is documented below.
        #[builder(into, default)]
        pub mfa: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::identityplatform::ConfigMfa>,
        >,
        /// Configuration related to monitoring project activity.
        /// Structure is documented below.
        #[builder(into, default)]
        pub monitoring: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::identityplatform::ConfigMonitoring>,
        >,
        /// Configuration related to multi-tenant functionality.
        /// Structure is documented below.
        #[builder(into, default)]
        pub multi_tenant: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::identityplatform::ConfigMultiTenant>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration related to quotas.
        /// Structure is documented below.
        #[builder(into, default)]
        pub quota: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::identityplatform::ConfigQuota>,
        >,
        /// Configuration related to local sign in methods.
        /// Structure is documented below.
        #[builder(into, default)]
        pub sign_in: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::identityplatform::ConfigSignIn>,
        >,
        /// Configures the regions where users are allowed to send verification SMS for the project or tenant. This is based on the calling code of the destination phone number.
        /// Structure is documented below.
        #[builder(into, default)]
        pub sms_region_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::identityplatform::ConfigSmsRegionConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct ConfigResult {
        /// List of domains authorized for OAuth redirects.
        pub authorized_domains: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Whether anonymous users will be auto-deleted after a period of 30 days
        pub autodelete_anonymous_users: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Configuration related to blocking functions.
        /// Structure is documented below.
        pub blocking_functions: pulumi_gestalt_rust::Output<
            Option<super::super::types::identityplatform::ConfigBlockingFunctions>,
        >,
        /// Options related to how clients making requests on behalf of a project should be configured.
        /// Structure is documented below.
        pub client: pulumi_gestalt_rust::Output<
            super::super::types::identityplatform::ConfigClient,
        >,
        /// Options related to how clients making requests on behalf of a project should be configured.
        /// Structure is documented below.
        pub mfa: pulumi_gestalt_rust::Output<
            super::super::types::identityplatform::ConfigMfa,
        >,
        /// Configuration related to monitoring project activity.
        /// Structure is documented below.
        pub monitoring: pulumi_gestalt_rust::Output<
            super::super::types::identityplatform::ConfigMonitoring,
        >,
        /// Configuration related to multi-tenant functionality.
        /// Structure is documented below.
        pub multi_tenant: pulumi_gestalt_rust::Output<
            Option<super::super::types::identityplatform::ConfigMultiTenant>,
        >,
        /// The name of the Config resource
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Configuration related to quotas.
        /// Structure is documented below.
        pub quota: pulumi_gestalt_rust::Output<
            Option<super::super::types::identityplatform::ConfigQuota>,
        >,
        /// Configuration related to local sign in methods.
        /// Structure is documented below.
        pub sign_in: pulumi_gestalt_rust::Output<
            super::super::types::identityplatform::ConfigSignIn,
        >,
        /// Configures the regions where users are allowed to send verification SMS for the project or tenant. This is based on the calling code of the destination phone number.
        /// Structure is documented below.
        pub sms_region_config: pulumi_gestalt_rust::Output<
            super::super::types::identityplatform::ConfigSmsRegionConfig,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ConfigArgs,
    ) -> ConfigResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let authorized_domains_binding = args.authorized_domains.get_output(context);
        let autodelete_anonymous_users_binding = args
            .autodelete_anonymous_users
            .get_output(context);
        let blocking_functions_binding = args.blocking_functions.get_output(context);
        let client_binding = args.client.get_output(context);
        let mfa_binding = args.mfa.get_output(context);
        let monitoring_binding = args.monitoring.get_output(context);
        let multi_tenant_binding = args.multi_tenant.get_output(context);
        let project_binding = args.project.get_output(context);
        let quota_binding = args.quota.get_output(context);
        let sign_in_binding = args.sign_in.get_output(context);
        let sms_region_config_binding = args.sms_region_config.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:identityplatform/config:Config".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authorizedDomains".into(),
                    value: authorized_domains_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autodeleteAnonymousUsers".into(),
                    value: autodelete_anonymous_users_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "blockingFunctions".into(),
                    value: blocking_functions_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "client".into(),
                    value: client_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mfa".into(),
                    value: mfa_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "monitoring".into(),
                    value: monitoring_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "multiTenant".into(),
                    value: multi_tenant_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "quota".into(),
                    value: quota_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "signIn".into(),
                    value: sign_in_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "smsRegionConfig".into(),
                    value: sms_region_config_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ConfigResult {
            authorized_domains: o.get_field("authorizedDomains"),
            autodelete_anonymous_users: o.get_field("autodeleteAnonymousUsers"),
            blocking_functions: o.get_field("blockingFunctions"),
            client: o.get_field("client"),
            mfa: o.get_field("mfa"),
            monitoring: o.get_field("monitoring"),
            multi_tenant: o.get_field("multiTenant"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            quota: o.get_field("quota"),
            sign_in: o.get_field("signIn"),
            sms_region_config: o.get_field("smsRegionConfig"),
        }
    }
}
