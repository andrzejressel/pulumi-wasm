/// An app's reCAPTCHA V3 configuration object.
///
///
/// To get more information about RecaptchaV3Config, see:
///
/// * [API documentation](https://firebase.google.com/docs/reference/appcheck/rest/v1/projects.apps.recaptchaV3Config)
/// * How-to Guides
///     * [Official Documentation](https://firebase.google.com/docs/app-check)
///
///
///
/// ## Example Usage
///
/// ### Firebase App Check Recaptcha V3 Config Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:firebase:WebApp
///     properties:
///       project: my-project-name
///       displayName: Web App for reCAPTCHA V3
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
///   defaultAppCheckRecaptchaV3Config:
///     type: gcp:firebase:AppCheckRecaptchaV3Config
///     name: default
///     properties:
///       project: my-project-name
///       appId: ${default.appId}
///       siteSecret: 6Lf9YnQpAAAAAC3-MHmdAllTbPwTZxpUw5d34YzX
///       tokenTtl: 7200s
///     options:
///       dependsOn:
///         - ${wait30s}
/// ```
///
/// ## Import
///
/// RecaptchaV3Config can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/apps/{{app_id}}/recaptchaV3Config`
///
/// * `{{project}}/{{app_id}}`
///
/// * `{{app_id}}`
///
/// When using the `pulumi import` command, RecaptchaV3Config can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:firebase/appCheckRecaptchaV3Config:AppCheckRecaptchaV3Config default projects/{{project}}/apps/{{app_id}}/recaptchaV3Config
/// ```
///
/// ```sh
/// $ pulumi import gcp:firebase/appCheckRecaptchaV3Config:AppCheckRecaptchaV3Config default {{project}}/{{app_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:firebase/appCheckRecaptchaV3Config:AppCheckRecaptchaV3Config default {{app_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod app_check_recaptcha_v_3_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AppCheckRecaptchaV3ConfigArgs {
        /// The ID of an
        /// [Web App](https://firebase.google.com/docs/reference/firebase-management/rest/v1beta1/projects.webApps#WebApp.FIELDS.app_id).
        ///
        ///
        /// - - -
        #[builder(into)]
        pub app_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The site secret used to identify your service for reCAPTCHA v3 verification.
        /// For security reasons, this field will never be populated in any response.
        /// **Note**: This property is sensitive and will not be displayed in the plan.
        #[builder(into)]
        pub site_secret: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the duration for which App Check tokens exchanged from reCAPTCHA V3 artifacts will be valid.
        /// If unset, a default value of 1 hour is assumed. Must be between 30 minutes and 7 days, inclusive.
        /// A duration in seconds with up to nine fractional digits, ending with 's'. Example: "3.5s".
        #[builder(into, default)]
        pub token_ttl: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AppCheckRecaptchaV3ConfigResult {
        /// The ID of an
        /// [Web App](https://firebase.google.com/docs/reference/firebase-management/rest/v1beta1/projects.webApps#WebApp.FIELDS.app_id).
        ///
        ///
        /// - - -
        pub app_id: pulumi_gestalt_rust::Output<String>,
        /// The relative resource name of the reCAPTCHA V3 configuration object
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The site secret used to identify your service for reCAPTCHA v3 verification.
        /// For security reasons, this field will never be populated in any response.
        /// **Note**: This property is sensitive and will not be displayed in the plan.
        pub site_secret: pulumi_gestalt_rust::Output<String>,
        /// Whether the siteSecret was previously set. Since we will never return the siteSecret field, this field is the only way to find out whether it was previously set.
        pub site_secret_set: pulumi_gestalt_rust::Output<bool>,
        /// Specifies the duration for which App Check tokens exchanged from reCAPTCHA V3 artifacts will be valid.
        /// If unset, a default value of 1 hour is assumed. Must be between 30 minutes and 7 days, inclusive.
        /// A duration in seconds with up to nine fractional digits, ending with 's'. Example: "3.5s".
        pub token_ttl: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AppCheckRecaptchaV3ConfigArgs,
    ) -> AppCheckRecaptchaV3ConfigResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let app_id_binding = args.app_id.get_output(context);
        let project_binding = args.project.get_output(context);
        let site_secret_binding = args.site_secret.get_output(context);
        let token_ttl_binding = args.token_ttl.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:firebase/appCheckRecaptchaV3Config:AppCheckRecaptchaV3Config"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "appId".into(),
                    value: app_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "siteSecret".into(),
                    value: site_secret_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tokenTtl".into(),
                    value: token_ttl_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AppCheckRecaptchaV3ConfigResult {
            app_id: o.get_field("appId"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            site_secret: o.get_field("siteSecret"),
            site_secret_set: o.get_field("siteSecretSet"),
            token_ttl: o.get_field("tokenTtl"),
        }
    }
}
