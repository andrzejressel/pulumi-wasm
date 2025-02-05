/// An app's reCAPTCHA Enterprise configuration object.
///
///
/// To get more information about RecaptchaEnterpriseConfig, see:
///
/// * [API documentation](https://firebase.google.com/docs/reference/appcheck/rest/v1/projects.apps.recaptchaEnterpriseConfig)
/// * How-to Guides
///     * [Official Documentation](https://firebase.google.com/docs/app-check)
///
/// ## Example Usage
///
/// ### Firebase App Check Recaptcha Enterprise Config Basic
///
///
/// ```yaml
/// resources:
///   # Enables the reCAPTCHA Enterprise API
///   recaptchaEnterprise:
///     type: gcp:projects:Service
///     name: recaptcha_enterprise
///     properties:
///       project: my-project-name
///       service: recaptchaenterprise.googleapis.com
///       disableOnDestroy: false
///   default:
///     type: gcp:firebase:WebApp
///     properties:
///       project: my-project-name
///       displayName: Web App for reCAPTCHA Enterprise
///   # It takes a while for App Check to recognize the new app
///   # If your app already exists, you don't have to wait 30 seconds.
///   wait30s:
///     type: time:sleep
///     name: wait_30s
///     properties:
///       createDuration: 30s
///     options:
///       dependsOn:
///         - ${default}
///   defaultAppCheckRecaptchaEnterpriseConfig:
///     type: gcp:firebase:AppCheckRecaptchaEnterpriseConfig
///     name: default
///     properties:
///       project: my-project-name
///       appId: ${default.appId}
///       siteKey: 6LdpMXIpAAAAANkwWQPgEdjEhal7ugkH9RK9ytuw
///       tokenTtl: 7200s
///     options:
///       dependsOn:
///         - ${wait30s}
/// ```
///
/// ## Import
///
/// RecaptchaEnterpriseConfig can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/apps/{{app_id}}/recaptchaEnterpriseConfig`
///
/// * `{{project}}/{{app_id}}`
///
/// * `{{app_id}}`
///
/// When using the `pulumi import` command, RecaptchaEnterpriseConfig can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:firebase/appCheckRecaptchaEnterpriseConfig:AppCheckRecaptchaEnterpriseConfig default projects/{{project}}/apps/{{app_id}}/recaptchaEnterpriseConfig
/// ```
///
/// ```sh
/// $ pulumi import gcp:firebase/appCheckRecaptchaEnterpriseConfig:AppCheckRecaptchaEnterpriseConfig default {{project}}/{{app_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:firebase/appCheckRecaptchaEnterpriseConfig:AppCheckRecaptchaEnterpriseConfig default {{app_id}}
/// ```
///
pub mod app_check_recaptcha_enterprise_config {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AppCheckRecaptchaEnterpriseConfigArgs {
        /// The ID of an
        /// [Web App](https://firebase.google.com/docs/reference/firebase-management/rest/v1beta1/projects.webApps#WebApp.FIELDS.app_id).
        ///
        ///
        /// - - -
        #[builder(into)]
        pub app_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The score-based site key created in reCAPTCHA Enterprise used to invoke reCAPTCHA and generate the reCAPTCHA tokens for your application.
        /// **Important**: This is not the siteSecret (as it is in reCAPTCHA v3), but rather your score-based reCAPTCHA Enterprise site key.
        #[builder(into)]
        pub site_key: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the duration for which App Check tokens exchanged from reCAPTCHA Enterprise artifacts will be valid.
        /// If unset, a default value of 1 hour is assumed. Must be between 30 minutes and 7 days, inclusive.
        /// A duration in seconds with up to nine fractional digits, ending with 's'. Example: "3.5s".
        #[builder(into, default)]
        pub token_ttl: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AppCheckRecaptchaEnterpriseConfigResult {
        /// The ID of an
        /// [Web App](https://firebase.google.com/docs/reference/firebase-management/rest/v1beta1/projects.webApps#WebApp.FIELDS.app_id).
        ///
        ///
        /// - - -
        pub app_id: pulumi_wasm_rust::Output<String>,
        /// The relative resource name of the reCAPTCHA Enterprise configuration object
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The score-based site key created in reCAPTCHA Enterprise used to invoke reCAPTCHA and generate the reCAPTCHA tokens for your application.
        /// **Important**: This is not the siteSecret (as it is in reCAPTCHA v3), but rather your score-based reCAPTCHA Enterprise site key.
        pub site_key: pulumi_wasm_rust::Output<String>,
        /// Specifies the duration for which App Check tokens exchanged from reCAPTCHA Enterprise artifacts will be valid.
        /// If unset, a default value of 1 hour is assumed. Must be between 30 minutes and 7 days, inclusive.
        /// A duration in seconds with up to nine fractional digits, ending with 's'. Example: "3.5s".
        pub token_ttl: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AppCheckRecaptchaEnterpriseConfigArgs,
    ) -> AppCheckRecaptchaEnterpriseConfigResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let app_id_binding = args.app_id.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let site_key_binding = args.site_key.get_output(context).get_inner();
        let token_ttl_binding = args.token_ttl.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:firebase/appCheckRecaptchaEnterpriseConfig:AppCheckRecaptchaEnterpriseConfig"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "appId".into(),
                    value: &app_id_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "siteKey".into(),
                    value: &site_key_binding,
                },
                register_interface::ObjectField {
                    name: "tokenTtl".into(),
                    value: &token_ttl_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AppCheckRecaptchaEnterpriseConfigResult {
            app_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("appId")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            site_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("siteKey"),
            ),
            token_ttl: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tokenTtl"),
            ),
        }
    }
}
