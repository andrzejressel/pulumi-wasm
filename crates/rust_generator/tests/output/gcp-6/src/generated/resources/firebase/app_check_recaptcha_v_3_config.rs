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
#[allow(clippy::doc_lazy_continuation)]
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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: AppCheckRecaptchaV3ConfigArgs,
    ) -> AppCheckRecaptchaV3ConfigResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let app_id_binding = args.app_id.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let site_secret_binding = args.site_secret.get_output(context).get_inner();
        let token_ttl_binding = args.token_ttl.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:firebase/appCheckRecaptchaV3Config:AppCheckRecaptchaV3Config"
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
                    name: "siteSecret".into(),
                    value: &site_secret_binding,
                },
                register_interface::ObjectField {
                    name: "tokenTtl".into(),
                    value: &token_ttl_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AppCheckRecaptchaV3ConfigResult {
            app_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("appId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            site_secret: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("siteSecret"),
            ),
            site_secret_set: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("siteSecretSet"),
            ),
            token_ttl: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tokenTtl"),
            ),
        }
    }
}
