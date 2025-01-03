/// A key for signing Cloud CDN signed URLs for Backend Services.
///
///
/// To get more information about BackendServiceSignedUrlKey, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/backendServices)
/// * How-to Guides
///     * [Using Signed URLs](https://cloud.google.com/cdn/docs/using-signed-urls/)
///
///
///
/// ## Example Usage
///
/// ### Backend Service Signed Url Key
///
///
/// ```yaml
/// resources:
///   urlSignature:
///     type: random:RandomId
///     name: url_signature
///     properties:
///       byteLength: 16
///   backendKey:
///     type: gcp:compute:BackendServiceSignedUrlKey
///     name: backend_key
///     properties:
///       name: test-key
///       keyValue: ${urlSignature.b64Url}
///       backendService: ${exampleBackend.name}
///   exampleBackend:
///     type: gcp:compute:BackendService
///     name: example_backend
///     properties:
///       name: my-backend-service
///       description: Our company website
///       portName: http
///       protocol: HTTP
///       timeoutSec: 10
///       enableCdn: true
///       backends:
///         - group: ${webservers.instanceGroup}
///       healthChecks: ${default.id}
///   webservers:
///     type: gcp:compute:InstanceGroupManager
///     properties:
///       name: my-webservers
///       versions:
///         - instanceTemplate: ${webserver.id}
///           name: primary
///       baseInstanceName: webserver
///       zone: us-central1-f
///       targetSize: 1
///   webserver:
///     type: gcp:compute:InstanceTemplate
///     properties:
///       name: standard-webserver
///       machineType: e2-medium
///       networkInterfaces:
///         - network: default
///       disks:
///         - sourceImage: debian-cloud/debian-11
///           autoDelete: true
///           boot: true
///   default:
///     type: gcp:compute:HttpHealthCheck
///     properties:
///       name: test
///       requestPath: /
///       checkIntervalSec: 1
///       timeoutSec: 1
/// ```
///
/// ## Import
///
/// This resource does not support import.
///
pub mod backend_service_signed_url_key {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackendServiceSignedUrlKeyArgs {
        /// The backend service this signed URL key belongs.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub backend_service: pulumi_wasm_rust::Output<String>,
        /// 128-bit key value used for signing the URL. The key value must be a
        /// valid RFC 4648 Section 5 base64url encoded string.
        /// **Note**: This property is sensitive and will not be displayed in the plan.
        #[builder(into)]
        pub key_value: pulumi_wasm_rust::Output<String>,
        /// Name of the signed URL key.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct BackendServiceSignedUrlKeyResult {
        /// The backend service this signed URL key belongs.
        ///
        ///
        /// - - -
        pub backend_service: pulumi_wasm_rust::Output<String>,
        /// 128-bit key value used for signing the URL. The key value must be a
        /// valid RFC 4648 Section 5 base64url encoded string.
        /// **Note**: This property is sensitive and will not be displayed in the plan.
        pub key_value: pulumi_wasm_rust::Output<String>,
        /// Name of the signed URL key.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: BackendServiceSignedUrlKeyArgs,
    ) -> BackendServiceSignedUrlKeyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let backend_service_binding = args.backend_service.get_inner();
        let key_value_binding = args.key_value.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/backendServiceSignedUrlKey:BackendServiceSignedUrlKey"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "backendService".into(),
                    value: &backend_service_binding,
                },
                register_interface::ObjectField {
                    name: "keyValue".into(),
                    value: &key_value_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "backendService".into(),
                },
                register_interface::ResultField {
                    name: "keyValue".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        BackendServiceSignedUrlKeyResult {
            backend_service: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backendService").unwrap(),
            ),
            key_value: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyValue").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
        }
    }
}
