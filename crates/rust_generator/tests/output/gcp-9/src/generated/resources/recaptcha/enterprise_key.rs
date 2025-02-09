/// The RecaptchaEnterprise Key resource
///
/// ## Example Usage
///
/// ### Android_key
/// A basic test of recaptcha enterprise key that can be used by Android apps
/// ```yaml
/// resources:
///   primary:
///     type: gcp:recaptcha:EnterpriseKey
///     properties:
///       displayName: display-name-one
///       androidSettings:
///         allowAllPackageNames: true
///         allowedPackageNames: []
///       project: my-project-name
///       testingOptions:
///         testingScore: 0.8
///       labels:
///         label-one: value-one
/// ```
/// ### Ios_key
/// A basic test of recaptcha enterprise key that can be used by iOS apps
/// ```yaml
/// resources:
///   primary:
///     type: gcp:recaptcha:EnterpriseKey
///     properties:
///       displayName: display-name-one
///       iosSettings:
///         allowAllBundleIds: true
///         allowedBundleIds: []
///       project: my-project-name
///       testingOptions:
///         testingScore: 1
///       labels:
///         label-one: value-one
/// ```
/// ### Minimal_key
/// A minimal test of recaptcha enterprise key
/// ```yaml
/// resources:
///   primary:
///     type: gcp:recaptcha:EnterpriseKey
///     properties:
///       displayName: display-name-one
///       project: my-project-name
///       webSettings:
///         integrationType: SCORE
///         allowAllDomains: true
///       labels: {}
/// ```
/// ### Waf_key
/// A basic test of recaptcha enterprise key that includes WAF settings
/// ```yaml
/// resources:
///   primary:
///     type: gcp:recaptcha:EnterpriseKey
///     properties:
///       displayName: display-name-one
///       project: my-project-name
///       testingOptions:
///         testingChallenge: NOCAPTCHA
///         testingScore: 0.5
///       wafSettings:
///         wafFeature: CHALLENGE_PAGE
///         wafService: CA
///       webSettings:
///         integrationType: INVISIBLE
///         allowAllDomains: true
///         allowedDomains: []
///         challengeSecurityPreference: USABILITY
///       labels:
///         label-one: value-one
/// ```
/// ### Web_key
/// A basic test of recaptcha enterprise key that can be used by websites
/// ```yaml
/// resources:
///   primary:
///     type: gcp:recaptcha:EnterpriseKey
///     properties:
///       displayName: display-name-one
///       project: my-project-name
///       testingOptions:
///         testingChallenge: NOCAPTCHA
///         testingScore: 0.5
///       webSettings:
///         integrationType: CHECKBOX
///         allowAllDomains: true
///         allowedDomains: []
///         challengeSecurityPreference: USABILITY
///       labels:
///         label-one: value-one
/// ```
/// ### Web_score_key
/// A basic test of recaptcha enterprise key with score integration type that can be used by websites
/// ```yaml
/// resources:
///   primary:
///     type: gcp:recaptcha:EnterpriseKey
///     properties:
///       displayName: display-name-one
///       project: my-project-name
///       testingOptions:
///         testingScore: 0.5
///       webSettings:
///         integrationType: SCORE
///         allowAllDomains: true
///         allowAmpTraffic: false
///         allowedDomains: []
///       labels:
///         label-one: value-one
/// ```
///
/// ## Import
///
/// Key can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/keys/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Key can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:recaptcha/enterpriseKey:EnterpriseKey default projects/{{project}}/keys/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:recaptcha/enterpriseKey:EnterpriseKey default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:recaptcha/enterpriseKey:EnterpriseKey default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod enterprise_key {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnterpriseKeyArgs {
        /// Settings for keys that can be used by Android apps.
        #[builder(into, default)]
        pub android_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::recaptcha::EnterpriseKeyAndroidSettings>,
        >,
        /// Human-readable display name of this key. Modifiable by user.
        ///
        ///
        ///
        /// - - -
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Settings for keys that can be used by iOS apps.
        #[builder(into, default)]
        pub ios_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::recaptcha::EnterpriseKeyIosSettings>,
        >,
        /// See [Creating and managing labels](https://cloud.google.com/recaptcha-enterprise/docs/labels).
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The project for the resource
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Options for user acceptance testing.
        #[builder(into, default)]
        pub testing_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::recaptcha::EnterpriseKeyTestingOptions>,
        >,
        /// Settings specific to keys that can be used for WAF (Web Application Firewall).
        #[builder(into, default)]
        pub waf_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::recaptcha::EnterpriseKeyWafSettings>,
        >,
        /// Settings for keys that can be used by websites.
        #[builder(into, default)]
        pub web_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::recaptcha::EnterpriseKeyWebSettings>,
        >,
    }
    #[allow(dead_code)]
    pub struct EnterpriseKeyResult {
        /// Settings for keys that can be used by Android apps.
        pub android_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::recaptcha::EnterpriseKeyAndroidSettings>,
        >,
        /// The timestamp corresponding to the creation of this Key.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Human-readable display name of this key. Modifiable by user.
        ///
        ///
        ///
        /// - - -
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Settings for keys that can be used by iOS apps.
        pub ios_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::recaptcha::EnterpriseKeyIosSettings>,
        >,
        /// See [Creating and managing labels](https://cloud.google.com/recaptcha-enterprise/docs/labels).
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The resource id for the Key, which is the same as the Site Key itself.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The project for the resource
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Options for user acceptance testing.
        pub testing_options: pulumi_gestalt_rust::Output<
            Option<super::super::types::recaptcha::EnterpriseKeyTestingOptions>,
        >,
        /// Settings specific to keys that can be used for WAF (Web Application Firewall).
        pub waf_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::recaptcha::EnterpriseKeyWafSettings>,
        >,
        /// Settings for keys that can be used by websites.
        pub web_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::recaptcha::EnterpriseKeyWebSettings>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EnterpriseKeyArgs,
    ) -> EnterpriseKeyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let android_settings_binding = args.android_settings.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let ios_settings_binding = args.ios_settings.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let project_binding = args.project.get_output(context);
        let testing_options_binding = args.testing_options.get_output(context);
        let waf_settings_binding = args.waf_settings.get_output(context);
        let web_settings_binding = args.web_settings.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:recaptcha/enterpriseKey:EnterpriseKey".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "androidSettings".into(),
                    value: android_settings_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "iosSettings".into(),
                    value: ios_settings_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: labels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "testingOptions".into(),
                    value: testing_options_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "wafSettings".into(),
                    value: waf_settings_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "webSettings".into(),
                    value: web_settings_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        EnterpriseKeyResult {
            android_settings: o.get_field("androidSettings"),
            create_time: o.get_field("createTime"),
            display_name: o.get_field("displayName"),
            effective_labels: o.get_field("effectiveLabels"),
            ios_settings: o.get_field("iosSettings"),
            labels: o.get_field("labels"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            testing_options: o.get_field("testingOptions"),
            waf_settings: o.get_field("wafSettings"),
            web_settings: o.get_field("webSettings"),
        }
    }
}
