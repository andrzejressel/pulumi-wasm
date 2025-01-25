/// IAP settings - manage IAP settings
///
///
/// To get more information about Settings, see:
///
/// * [API documentation](https://cloud.google.com/iap/docs/reference/rest/v1/IapSettings)
/// * How-to Guides
///     * [Customizing IAP](https://cloud.google.com/iap/docs/customizing)
///
///
///
/// ## Example Usage
///
/// ### Iap Settings Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:RegionBackendService
///     properties:
///       name: iap-settings-tf
///       region: us-central1
///       healthChecks: ${defaultHealthCheck.id}
///       connectionDrainingTimeoutSec: 10
///       sessionAffinity: CLIENT_IP
///   defaultHealthCheck:
///     type: gcp:compute:HealthCheck
///     name: default
///     properties:
///       name: iap-bs-health-check
///       checkIntervalSec: 1
///       timeoutSec: 1
///       tcpHealthCheck:
///         port: '80'
///   iapSettings:
///     type: gcp:iap:Settings
///     name: iap_settings
///     properties:
///       name: projects/${project.number}/iap_web/compute-us-central1/services/${default.name}
///       accessSettings:
///         identitySources:
///           - WORKFORCE_IDENTITY_FEDERATION
///         allowedDomainsSettings:
///           domains:
///             - test.abc.com
///           enable: true
///         corsSettings:
///           allowHttpOptions: true
///         reauthSettings:
///           method: SECURE_KEY
///           maxAge: 305s
///           policyType: MINIMUM
///         gcipSettings:
///           loginPageUri: https://test.com/?apiKey=abc
///         oauthSettings:
///           loginHint: test
///         workforceIdentitySettings:
///           workforcePools: wif-pool
///           oauth2:
///             clientId: test-client-id
///             clientSecret: test-client-secret
///       applicationSettings:
///         cookieDomain: test.abc.com
///         csmSettings:
///           rctokenAud: test-aud-set
///         accessDeniedPageSettings:
///           accessDeniedPageUri: test-uri
///           generateTroubleshootingUri: true
///           remediationTokenGenerationEnabled: false
///         attributePropagationSettings:
///           outputCredentials:
///             - HEADER
///           expression: attributes.saml_attributes.filter(attribute, attribute.name in ["test1", "test2"])
///           enable: false
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// Settings can be imported using any of these accepted formats:
///
/// * `{{name}}/iapSettings`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Settings can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:iap/settings:Settings default {{name}}/iapSettings
/// ```
///
/// ```sh
/// $ pulumi import gcp:iap/settings:Settings default {{name}}
/// ```
///
pub mod settings {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SettingsArgs {
        /// Top level wrapper for all access related setting in IAP.
        /// Structure is documented below.
        #[builder(into, default)]
        pub access_settings: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::iap::SettingsAccessSettings>,
        >,
        /// Top level wrapper for all application related settings in IAP.
        /// Structure is documented below.
        #[builder(into, default)]
        pub application_settings: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::iap::SettingsApplicationSettings>,
        >,
        /// The resource name of the IAP protected resource. Name can have below resources:
        /// * organizations/{organization_id}
        /// * folders/{folder_id}
        /// * projects/{projects_id}
        /// * projects/{projects_id}/iap_web
        /// * projects/{projects_id}/iap_web/compute
        /// * projects/{projects_id}/iap_web/compute-{region}
        /// * projects/{projects_id}/iap_web/compute/service/{service_id}
        /// * projects/{projects_id}/iap_web/compute-{region}/service/{service_id}
        /// * projects/{projects_id}/iap_web/appengine-{app_id}
        /// * projects/{projects_id}/iap_web/appengine-{app_id}/service/{service_id}
        /// * projects/{projects_id}/iap_web/appengine-{app_id}/service/{service_id}/version/{version_id}
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SettingsResult {
        /// Top level wrapper for all access related setting in IAP.
        /// Structure is documented below.
        pub access_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::iap::SettingsAccessSettings>,
        >,
        /// Top level wrapper for all application related settings in IAP.
        /// Structure is documented below.
        pub application_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::iap::SettingsApplicationSettings>,
        >,
        /// The resource name of the IAP protected resource. Name can have below resources:
        /// * organizations/{organization_id}
        /// * folders/{folder_id}
        /// * projects/{projects_id}
        /// * projects/{projects_id}/iap_web
        /// * projects/{projects_id}/iap_web/compute
        /// * projects/{projects_id}/iap_web/compute-{region}
        /// * projects/{projects_id}/iap_web/compute/service/{service_id}
        /// * projects/{projects_id}/iap_web/compute-{region}/service/{service_id}
        /// * projects/{projects_id}/iap_web/appengine-{app_id}
        /// * projects/{projects_id}/iap_web/appengine-{app_id}/service/{service_id}
        /// * projects/{projects_id}/iap_web/appengine-{app_id}/service/{service_id}/version/{version_id}
        ///
        ///
        /// - - -
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: SettingsArgs,
    ) -> SettingsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_settings_binding = args
            .access_settings
            .get_output(context)
            .get_inner();
        let application_settings_binding = args
            .application_settings
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:iap/settings:Settings".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessSettings".into(),
                    value: &access_settings_binding,
                },
                register_interface::ObjectField {
                    name: "applicationSettings".into(),
                    value: &application_settings_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessSettings".into(),
                },
                register_interface::ResultField {
                    name: "applicationSettings".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SettingsResult {
            access_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessSettings").unwrap(),
            ),
            application_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationSettings").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
        }
    }
}
