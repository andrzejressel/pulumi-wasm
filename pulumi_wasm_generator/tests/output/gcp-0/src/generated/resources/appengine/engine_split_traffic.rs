/// Traffic routing configuration for versions within a single service. Traffic splits define how traffic directed to the service is assigned to versions.
///
///
/// To get more information about ServiceSplitTraffic, see:
///
/// * [API documentation](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services)
///
/// ## Example Usage
///
/// ### App Engine Service Split Traffic
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
///   liveappV1:
///     type: gcp:appengine:StandardAppVersion
///     name: liveapp_v1
///     properties:
///       versionId: v1
///       service: liveapp
///       deleteServiceOnDestroy: true
///       runtime: nodejs20
///       entrypoint:
///         shell: node ./app.js
///       deployment:
///         zip:
///           sourceUrl: https://storage.googleapis.com/${bucket.name}/${object.name}
///       envVariables:
///         port: '8080'
///   liveappV2:
///     type: gcp:appengine:StandardAppVersion
///     name: liveapp_v2
///     properties:
///       versionId: v2
///       service: liveapp
///       noopOnDestroy: true
///       runtime: nodejs20
///       entrypoint:
///         shell: node ./app.js
///       deployment:
///         zip:
///           sourceUrl: https://storage.googleapis.com/${bucket.name}/${object.name}
///       envVariables:
///         port: '8080'
///   liveapp:
///     type: gcp:appengine:EngineSplitTraffic
///     properties:
///       service: ${liveappV2.service}
///       migrateTraffic: false
///       split:
///         shardBy: IP
///         allocations:
///           ${liveappV1.versionId}: 0.75
///           ${liveappV2.versionId}: 0.25
/// ```
///
/// ## Import
///
/// ServiceSplitTraffic can be imported using any of these accepted formats:
///
/// * `apps/{{project}}/services/{{service}}`
///
/// * `{{project}}/{{service}}`
///
/// * `{{service}}`
///
/// When using the `pulumi import` command, ServiceSplitTraffic can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:appengine/engineSplitTraffic:EngineSplitTraffic default apps/{{project}}/services/{{service}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:appengine/engineSplitTraffic:EngineSplitTraffic default {{project}}/{{service}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:appengine/engineSplitTraffic:EngineSplitTraffic default {{service}}
/// ```
///
pub mod engine_split_traffic {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EngineSplitTrafficArgs {
        /// If set to true traffic will be migrated to this version.
        #[builder(into, default)]
        pub migrate_traffic: pulumi_wasm_rust::Output<Option<bool>>,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the service these settings apply to.
        #[builder(into)]
        pub service: pulumi_wasm_rust::Output<String>,
        /// Mapping that defines fractional HTTP traffic diversion to different versions within the service.
        /// Structure is documented below.
        #[builder(into)]
        pub split: pulumi_wasm_rust::Output<
            super::super::types::appengine::EngineSplitTrafficSplit,
        >,
    }
    #[allow(dead_code)]
    pub struct EngineSplitTrafficResult {
        /// If set to true traffic will be migrated to this version.
        pub migrate_traffic: pulumi_wasm_rust::Output<Option<bool>>,
        pub project: pulumi_wasm_rust::Output<String>,
        /// The name of the service these settings apply to.
        pub service: pulumi_wasm_rust::Output<String>,
        /// Mapping that defines fractional HTTP traffic diversion to different versions within the service.
        /// Structure is documented below.
        pub split: pulumi_wasm_rust::Output<
            super::super::types::appengine::EngineSplitTrafficSplit,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EngineSplitTrafficArgs) -> EngineSplitTrafficResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let migrate_traffic_binding = args.migrate_traffic.get_inner();
        let project_binding = args.project.get_inner();
        let service_binding = args.service.get_inner();
        let split_binding = args.split.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:appengine/engineSplitTraffic:EngineSplitTraffic".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "migrateTraffic".into(),
                    value: &migrate_traffic_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "service".into(),
                    value: &service_binding,
                },
                register_interface::ObjectField {
                    name: "split".into(),
                    value: &split_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "migrateTraffic".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "service".into(),
                },
                register_interface::ResultField {
                    name: "split".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EngineSplitTrafficResult {
            migrate_traffic: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("migrateTraffic").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            service: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("service").unwrap(),
            ),
            split: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("split").unwrap(),
            ),
        }
    }
}
