/// An app's DeviceCheck configuration object. Note that the Team ID registered with your
/// app is used as part of the validation process. Make sure your `gcp.firebase.AppleApp` has a team_id present.
///
///
/// To get more information about DeviceCheckConfig, see:
///
/// * [API documentation](https://firebase.google.com/docs/reference/appcheck/rest/v1/projects.apps.deviceCheckConfig)
/// * How-to Guides
///     * [Official Documentation](https://firebase.google.com/docs/app-check)
///
///
///
/// ## Example Usage
///
/// ### Firebase App Check Device Check Config Full
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:firebase:AppleApp
///     properties:
///       project: my-project-name
///       displayName: Apple app
///       bundleId: bundle.id.devicecheck
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
///   defaultAppCheckDeviceCheckConfig:
///     type: gcp:firebase:AppCheckDeviceCheckConfig
///     name: default
///     properties:
///       project: my-project-name
///       appId: ${default.appId}
///       tokenTtl: 7200s
///       keyId: Key ID
///       privateKey:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: path/to/private-key.p8
///           return: result
///     options:
///       dependsOn:
///         - ${wait30s}
/// ```
///
/// ## Import
///
/// DeviceCheckConfig can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/apps/{{app_id}}/deviceCheckConfig`
///
/// * `{{project}}/{{app_id}}`
///
/// * `{{app_id}}`
///
/// When using the `pulumi import` command, DeviceCheckConfig can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:firebase/appCheckDeviceCheckConfig:AppCheckDeviceCheckConfig default projects/{{project}}/apps/{{app_id}}/deviceCheckConfig
/// ```
///
/// ```sh
/// $ pulumi import gcp:firebase/appCheckDeviceCheckConfig:AppCheckDeviceCheckConfig default {{project}}/{{app_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:firebase/appCheckDeviceCheckConfig:AppCheckDeviceCheckConfig default {{app_id}}
/// ```
///
pub mod app_check_device_check_config {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AppCheckDeviceCheckConfigArgs {
        /// The ID of an
        /// [Apple App](https://firebase.google.com/docs/reference/firebase-management/rest/v1beta1/projects.iosApps#IosApp.FIELDS.app_id).
        ///
        ///
        /// - - -
        #[builder(into)]
        pub app_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The key identifier of a private key enabled with DeviceCheck, created in your Apple Developer account.
        #[builder(into)]
        pub key_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The contents of the private key (.p8) file associated with the key specified by keyId.
        /// **Note**: This property is sensitive and will not be displayed in the plan.
        #[builder(into)]
        pub private_key: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the duration for which App Check tokens exchanged from DeviceCheck artifacts will be valid.
        /// If unset, a default value of 1 hour is assumed. Must be between 30 minutes and 7 days, inclusive.
        /// A duration in seconds with up to nine fractional digits, ending with 's'. Example: "3.5s".
        #[builder(into, default)]
        pub token_ttl: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AppCheckDeviceCheckConfigResult {
        /// The ID of an
        /// [Apple App](https://firebase.google.com/docs/reference/firebase-management/rest/v1beta1/projects.iosApps#IosApp.FIELDS.app_id).
        ///
        ///
        /// - - -
        pub app_id: pulumi_wasm_rust::Output<String>,
        /// The key identifier of a private key enabled with DeviceCheck, created in your Apple Developer account.
        pub key_id: pulumi_wasm_rust::Output<String>,
        /// The relative resource name of the DeviceCheck configuration object
        pub name: pulumi_wasm_rust::Output<String>,
        /// The contents of the private key (.p8) file associated with the key specified by keyId.
        /// **Note**: This property is sensitive and will not be displayed in the plan.
        pub private_key: pulumi_wasm_rust::Output<String>,
        /// Whether the privateKey field was previously set. Since App Check will never return the
        /// privateKey field, this field is the only way to find out whether it was previously set.
        pub private_key_set: pulumi_wasm_rust::Output<bool>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// Specifies the duration for which App Check tokens exchanged from DeviceCheck artifacts will be valid.
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
        args: AppCheckDeviceCheckConfigArgs,
    ) -> AppCheckDeviceCheckConfigResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let app_id_binding = args.app_id.get_output(context).get_inner();
        let key_id_binding = args.key_id.get_output(context).get_inner();
        let private_key_binding = args.private_key.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let token_ttl_binding = args.token_ttl.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:firebase/appCheckDeviceCheckConfig:AppCheckDeviceCheckConfig"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "appId".into(),
                    value: &app_id_binding,
                },
                register_interface::ObjectField {
                    name: "keyId".into(),
                    value: &key_id_binding,
                },
                register_interface::ObjectField {
                    name: "privateKey".into(),
                    value: &private_key_binding,
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
                    name: "keyId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "privateKey".into(),
                },
                register_interface::ResultField {
                    name: "privateKeySet".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "tokenTtl".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AppCheckDeviceCheckConfigResult {
            app_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appId").unwrap(),
            ),
            key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            private_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateKey").unwrap(),
            ),
            private_key_set: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateKeySet").unwrap(),
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
