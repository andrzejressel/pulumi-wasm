/// A NetworkSettings resource is a container for ingress settings for a version or service.
///
///
/// To get more information about ServiceNetworkSettings, see:
///
/// * [API documentation](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services)
///
/// ## Example Usage
///
/// ### App Engine Service Network Settings
///
///
/// ```yaml
/// resources:
///   bucket:
///     type: gcp:storage:Bucket
///     properties:
///       name: appengine-static-content
///       location: US
///   object:
///     type: gcp:storage:BucketObject
///     properties:
///       name: hello-world.zip
///       bucket: ${bucket.name}
///       source:
///         fn::FileAsset: ./test-fixtures/hello-world.zip
///   internalapp:
///     type: gcp:appengine:StandardAppVersion
///     properties:
///       versionId: v1
///       service: internalapp
///       deleteServiceOnDestroy: true
///       runtime: nodejs20
///       entrypoint:
///         shell: node ./app.js
///       deployment:
///         zip:
///           sourceUrl: https://storage.googleapis.com/${bucket.name}/${object.name}
///       envVariables:
///         port: '8080'
///   internalappServiceNetworkSettings:
///     type: gcp:appengine:ServiceNetworkSettings
///     name: internalapp
///     properties:
///       service: ${internalapp.service}
///       networkSettings:
///         ingressTrafficAllowed: INGRESS_TRAFFIC_ALLOWED_INTERNAL_ONLY
/// ```
///
/// ## Import
///
/// ServiceNetworkSettings can be imported using any of these accepted formats:
///
/// * `apps/{{project}}/services/{{service}}`
///
/// * `{{project}}/{{service}}`
///
/// * `{{service}}`
///
/// When using the `pulumi import` command, ServiceNetworkSettings can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:appengine/serviceNetworkSettings:ServiceNetworkSettings default apps/{{project}}/services/{{service}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:appengine/serviceNetworkSettings:ServiceNetworkSettings default {{project}}/{{service}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:appengine/serviceNetworkSettings:ServiceNetworkSettings default {{service}}
/// ```
///
pub mod service_network_settings {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceNetworkSettingsArgs {
        /// Ingress settings for this service. Will apply to all versions.
        /// Structure is documented below.
        #[builder(into)]
        pub network_settings: pulumi_wasm_rust::InputOrOutput<
            super::super::types::appengine::ServiceNetworkSettingsNetworkSettings,
        >,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the service these settings apply to.
        #[builder(into)]
        pub service: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ServiceNetworkSettingsResult {
        /// Ingress settings for this service. Will apply to all versions.
        /// Structure is documented below.
        pub network_settings: pulumi_wasm_rust::Output<
            super::super::types::appengine::ServiceNetworkSettingsNetworkSettings,
        >,
        pub project: pulumi_wasm_rust::Output<String>,
        /// The name of the service these settings apply to.
        pub service: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ServiceNetworkSettingsArgs,
    ) -> ServiceNetworkSettingsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let network_settings_binding = args
            .network_settings
            .get_output(context)
            .get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let service_binding = args.service.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:appengine/serviceNetworkSettings:ServiceNetworkSettings".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "networkSettings".into(),
                    value: &network_settings_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "service".into(),
                    value: &service_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "networkSettings".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "service".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ServiceNetworkSettingsResult {
            network_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkSettings").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            service: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("service").unwrap(),
            ),
        }
    }
}
