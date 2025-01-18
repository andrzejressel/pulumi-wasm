/// An app's App Attest configuration object. Note that the Team ID registered with your
/// app is used as part of the validation process. Make sure your `gcp.firebase.AppleApp` has a team_id present.
///
///
/// To get more information about AppAttestConfig, see:
///
/// * [API documentation](https://firebase.google.com/docs/reference/appcheck/rest/v1/projects.apps.appAttestConfig)
/// * How-to Guides
///     * [Official Documentation](https://firebase.google.com/docs/app-check)
///
/// ## Example Usage
///
/// ### Firebase App Check App Attest Config Minimal
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:firebase:AppleApp
///     properties:
///       project: my-project-name
///       displayName: Apple app
///       bundleId: bundle.id.appattest
///       teamId: '9987654321'
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
///   defaultAppCheckAppAttestConfig:
///     type: gcp:firebase:AppCheckAppAttestConfig
///     name: default
///     properties:
///       project: my-project-name
///       appId: ${default.appId}
///     options:
///       dependsOn:
///         - ${wait30s}
/// ```
/// ### Firebase App Check App Attest Config Full
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:firebase:AppleApp
///     properties:
///       project: my-project-name
///       displayName: Apple app
///       bundleId: bundle.id.appattest
///       teamId: '9987654321'
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
///   defaultAppCheckAppAttestConfig:
///     type: gcp:firebase:AppCheckAppAttestConfig
///     name: default
///     properties:
///       project: my-project-name
///       appId: ${default.appId}
///       tokenTtl: 7200s
///     options:
///       dependsOn:
///         - ${wait30s}
/// ```
///
/// ## Import
///
/// AppAttestConfig can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/apps/{{app_id}}/appAttestConfig`
///
/// * `{{project}}/{{app_id}}`
///
/// * `{{app_id}}`
///
/// When using the `pulumi import` command, AppAttestConfig can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:firebase/appCheckAppAttestConfig:AppCheckAppAttestConfig default projects/{{project}}/apps/{{app_id}}/appAttestConfig
/// ```
///
/// ```sh
/// $ pulumi import gcp:firebase/appCheckAppAttestConfig:AppCheckAppAttestConfig default {{project}}/{{app_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:firebase/appCheckAppAttestConfig:AppCheckAppAttestConfig default {{app_id}}
/// ```
///
pub mod app_check_app_attest_config {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AppCheckAppAttestConfigArgs {
        /// The ID of an
        /// [Apple App](https://firebase.google.com/docs/reference/firebase-management/rest/v1beta1/projects.iosApps#IosApp.FIELDS.app_id).
        ///
        ///
        /// - - -
        #[builder(into)]
        pub app_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the duration for which App Check tokens exchanged from App Attest artifacts will be valid.
        /// If unset, a default value of 1 hour is assumed. Must be between 30 minutes and 7 days, inclusive.
        /// A duration in seconds with up to nine fractional digits, ending with 's'. Example: "3.5s".
        #[builder(into, default)]
        pub token_ttl: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AppCheckAppAttestConfigResult {
        /// The ID of an
        /// [Apple App](https://firebase.google.com/docs/reference/firebase-management/rest/v1beta1/projects.iosApps#IosApp.FIELDS.app_id).
        ///
        ///
        /// - - -
        pub app_id: pulumi_wasm_rust::Output<String>,
        /// The relative resource name of the App Attest configuration object
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// Specifies the duration for which App Check tokens exchanged from App Attest artifacts will be valid.
        /// If unset, a default value of 1 hour is assumed. Must be between 30 minutes and 7 days, inclusive.
        /// A duration in seconds with up to nine fractional digits, ending with 's'. Example: "3.5s".
        pub token_ttl: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: AppCheckAppAttestConfigArgs,
    ) -> AppCheckAppAttestConfigResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let app_id_binding = args.app_id.get_inner();
        let project_binding = args.project.get_inner();
        let token_ttl_binding = args.token_ttl.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:firebase/appCheckAppAttestConfig:AppCheckAppAttestConfig".into(),
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
                    name: "tokenTtl".into(),
                    value: &token_ttl_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "appId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "tokenTtl".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AppCheckAppAttestConfigResult {
            app_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            token_ttl: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tokenTtl").unwrap(),
            ),
        }
    }
}
