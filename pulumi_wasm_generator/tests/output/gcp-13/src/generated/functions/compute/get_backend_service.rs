pub mod get_backend_service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBackendServiceArgs {
        /// The name of the Backend Service.
        ///
        /// - - -
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The project in which the resource belongs. If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetBackendServiceResult {
        pub affinity_cookie_ttl_sec: pulumi_wasm_rust::Output<i32>,
        /// The set of backends that serve this Backend Service.
        pub backends: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetBackendServiceBackend>,
        >,
        pub cdn_policies: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetBackendServiceCdnPolicy>,
        >,
        pub circuit_breakers: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetBackendServiceCircuitBreaker>,
        >,
        pub compression_mode: pulumi_wasm_rust::Output<String>,
        /// Time for which instance will be drained (not accept new connections, but still work to finish started ones).
        pub connection_draining_timeout_sec: pulumi_wasm_rust::Output<i32>,
        pub consistent_hash: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetBackendServiceConsistentHash>,
        >,
        pub creation_timestamp: pulumi_wasm_rust::Output<String>,
        pub custom_request_headers: pulumi_wasm_rust::Output<Vec<String>>,
        pub custom_response_headers: pulumi_wasm_rust::Output<Vec<String>>,
        /// Textual description for the Backend Service.
        pub description: pulumi_wasm_rust::Output<String>,
        pub edge_security_policy: pulumi_wasm_rust::Output<String>,
        /// Whether or not Cloud CDN is enabled on the Backend Service.
        pub enable_cdn: pulumi_wasm_rust::Output<bool>,
        /// The fingerprint of the Backend Service.
        pub fingerprint: pulumi_wasm_rust::Output<String>,
        /// The unique identifier for the resource. This identifier is defined by the server.
        pub generated_id: pulumi_wasm_rust::Output<i32>,
        /// The set of HTTP/HTTPS health checks used by the Backend Service.
        pub health_checks: pulumi_wasm_rust::Output<Vec<String>>,
        pub iaps: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetBackendServiceIap>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub ip_address_selection_policy: pulumi_wasm_rust::Output<String>,
        pub load_balancing_scheme: pulumi_wasm_rust::Output<String>,
        pub locality_lb_policies: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetBackendServiceLocalityLbPolicy>,
        >,
        pub locality_lb_policy: pulumi_wasm_rust::Output<String>,
        pub log_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetBackendServiceLogConfig>,
        >,
        pub name: pulumi_wasm_rust::Output<String>,
        pub outlier_detections: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetBackendServiceOutlierDetection>,
        >,
        /// The name of a service that has been added to an instance group in this backend.
        pub port_name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The protocol for incoming requests.
        pub protocol: pulumi_wasm_rust::Output<String>,
        pub security_policy: pulumi_wasm_rust::Output<String>,
        pub security_settings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetBackendServiceSecuritySetting>,
        >,
        /// The URI of the Backend Service.
        pub self_link: pulumi_wasm_rust::Output<String>,
        pub service_lb_policy: pulumi_wasm_rust::Output<String>,
        /// The Backend Service session stickiness configuration.
        pub session_affinity: pulumi_wasm_rust::Output<String>,
        pub strong_session_affinity_cookies: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::compute::GetBackendServiceStrongSessionAffinityCooky,
            >,
        >,
        /// The number of seconds to wait for a backend to respond to a request before considering the request failed.
        pub timeout_sec: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetBackendServiceArgs) -> GetBackendServiceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:compute/getBackendService:getBackendService".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
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
                    name: "affinityCookieTtlSec".into(),
                },
                register_interface::ResultField {
                    name: "backends".into(),
                },
                register_interface::ResultField {
                    name: "cdnPolicies".into(),
                },
                register_interface::ResultField {
                    name: "circuitBreakers".into(),
                },
                register_interface::ResultField {
                    name: "compressionMode".into(),
                },
                register_interface::ResultField {
                    name: "connectionDrainingTimeoutSec".into(),
                },
                register_interface::ResultField {
                    name: "consistentHash".into(),
                },
                register_interface::ResultField {
                    name: "creationTimestamp".into(),
                },
                register_interface::ResultField {
                    name: "customRequestHeaders".into(),
                },
                register_interface::ResultField {
                    name: "customResponseHeaders".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "edgeSecurityPolicy".into(),
                },
                register_interface::ResultField {
                    name: "enableCdn".into(),
                },
                register_interface::ResultField {
                    name: "fingerprint".into(),
                },
                register_interface::ResultField {
                    name: "generatedId".into(),
                },
                register_interface::ResultField {
                    name: "healthChecks".into(),
                },
                register_interface::ResultField {
                    name: "iaps".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "ipAddressSelectionPolicy".into(),
                },
                register_interface::ResultField {
                    name: "loadBalancingScheme".into(),
                },
                register_interface::ResultField {
                    name: "localityLbPolicies".into(),
                },
                register_interface::ResultField {
                    name: "localityLbPolicy".into(),
                },
                register_interface::ResultField {
                    name: "logConfigs".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "outlierDetections".into(),
                },
                register_interface::ResultField {
                    name: "portName".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "protocol".into(),
                },
                register_interface::ResultField {
                    name: "securityPolicy".into(),
                },
                register_interface::ResultField {
                    name: "securitySettings".into(),
                },
                register_interface::ResultField {
                    name: "selfLink".into(),
                },
                register_interface::ResultField {
                    name: "serviceLbPolicy".into(),
                },
                register_interface::ResultField {
                    name: "sessionAffinity".into(),
                },
                register_interface::ResultField {
                    name: "strongSessionAffinityCookies".into(),
                },
                register_interface::ResultField {
                    name: "timeoutSec".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetBackendServiceResult {
            affinity_cookie_ttl_sec: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("affinityCookieTtlSec").unwrap(),
            ),
            backends: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backends").unwrap(),
            ),
            cdn_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cdnPolicies").unwrap(),
            ),
            circuit_breakers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("circuitBreakers").unwrap(),
            ),
            compression_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("compressionMode").unwrap(),
            ),
            connection_draining_timeout_sec: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionDrainingTimeoutSec").unwrap(),
            ),
            consistent_hash: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("consistentHash").unwrap(),
            ),
            creation_timestamp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTimestamp").unwrap(),
            ),
            custom_request_headers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customRequestHeaders").unwrap(),
            ),
            custom_response_headers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customResponseHeaders").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            edge_security_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("edgeSecurityPolicy").unwrap(),
            ),
            enable_cdn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableCdn").unwrap(),
            ),
            fingerprint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fingerprint").unwrap(),
            ),
            generated_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("generatedId").unwrap(),
            ),
            health_checks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("healthChecks").unwrap(),
            ),
            iaps: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("iaps").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            ip_address_selection_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipAddressSelectionPolicy").unwrap(),
            ),
            load_balancing_scheme: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loadBalancingScheme").unwrap(),
            ),
            locality_lb_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("localityLbPolicies").unwrap(),
            ),
            locality_lb_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("localityLbPolicy").unwrap(),
            ),
            log_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logConfigs").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            outlier_detections: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outlierDetections").unwrap(),
            ),
            port_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("portName").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            protocol: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protocol").unwrap(),
            ),
            security_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityPolicy").unwrap(),
            ),
            security_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securitySettings").unwrap(),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLink").unwrap(),
            ),
            service_lb_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceLbPolicy").unwrap(),
            ),
            session_affinity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sessionAffinity").unwrap(),
            ),
            strong_session_affinity_cookies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("strongSessionAffinityCookies").unwrap(),
            ),
            timeout_sec: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeoutSec").unwrap(),
            ),
        }
    }
}
