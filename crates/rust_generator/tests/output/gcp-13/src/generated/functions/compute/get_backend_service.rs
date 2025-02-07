pub mod get_backend_service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBackendServiceArgs {
        /// The name of the Backend Service.
        ///
        /// - - -
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The project in which the resource belongs. If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetBackendServiceResult {
        pub affinity_cookie_ttl_sec: pulumi_gestalt_rust::Output<i32>,
        /// The set of backends that serve this Backend Service.
        pub backends: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetBackendServiceBackend>,
        >,
        pub cdn_policies: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetBackendServiceCdnPolicy>,
        >,
        pub circuit_breakers: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetBackendServiceCircuitBreaker>,
        >,
        pub compression_mode: pulumi_gestalt_rust::Output<String>,
        /// Time for which instance will be drained (not accept new connections, but still work to finish started ones).
        pub connection_draining_timeout_sec: pulumi_gestalt_rust::Output<i32>,
        pub consistent_hash: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetBackendServiceConsistentHash>,
        >,
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        pub custom_request_headers: pulumi_gestalt_rust::Output<Vec<String>>,
        pub custom_response_headers: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Textual description for the Backend Service.
        pub description: pulumi_gestalt_rust::Output<String>,
        pub edge_security_policy: pulumi_gestalt_rust::Output<String>,
        /// Whether or not Cloud CDN is enabled on the Backend Service.
        pub enable_cdn: pulumi_gestalt_rust::Output<bool>,
        /// The fingerprint of the Backend Service.
        pub fingerprint: pulumi_gestalt_rust::Output<String>,
        /// The unique identifier for the resource. This identifier is defined by the server.
        pub generated_id: pulumi_gestalt_rust::Output<i32>,
        /// The set of HTTP/HTTPS health checks used by the Backend Service.
        pub health_checks: pulumi_gestalt_rust::Output<Vec<String>>,
        pub iaps: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetBackendServiceIap>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub ip_address_selection_policy: pulumi_gestalt_rust::Output<String>,
        pub load_balancing_scheme: pulumi_gestalt_rust::Output<String>,
        pub locality_lb_policies: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetBackendServiceLocalityLbPolicy>,
        >,
        pub locality_lb_policy: pulumi_gestalt_rust::Output<String>,
        pub log_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetBackendServiceLogConfig>,
        >,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub outlier_detections: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetBackendServiceOutlierDetection>,
        >,
        /// The name of a service that has been added to an instance group in this backend.
        pub port_name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        /// The protocol for incoming requests.
        pub protocol: pulumi_gestalt_rust::Output<String>,
        pub security_policy: pulumi_gestalt_rust::Output<String>,
        pub security_settings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetBackendServiceSecuritySetting>,
        >,
        /// The URI of the Backend Service.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        pub service_lb_policy: pulumi_gestalt_rust::Output<String>,
        /// The Backend Service session stickiness configuration.
        pub session_affinity: pulumi_gestalt_rust::Output<String>,
        pub strong_session_affinity_cookies: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::compute::GetBackendServiceStrongSessionAffinityCooky,
            >,
        >,
        /// The number of seconds to wait for a backend to respond to a request before considering the request failed.
        pub timeout_sec: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetBackendServiceArgs,
    ) -> GetBackendServiceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetBackendServiceResult {
            affinity_cookie_ttl_sec: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("affinityCookieTtlSec"),
            ),
            backends: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("backends"),
            ),
            cdn_policies: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cdnPolicies"),
            ),
            circuit_breakers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("circuitBreakers"),
            ),
            compression_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("compressionMode"),
            ),
            connection_draining_timeout_sec: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("connectionDrainingTimeoutSec"),
            ),
            consistent_hash: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("consistentHash"),
            ),
            creation_timestamp: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("creationTimestamp"),
            ),
            custom_request_headers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customRequestHeaders"),
            ),
            custom_response_headers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customResponseHeaders"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            edge_security_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("edgeSecurityPolicy"),
            ),
            enable_cdn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableCdn"),
            ),
            fingerprint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fingerprint"),
            ),
            generated_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("generatedId"),
            ),
            health_checks: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("healthChecks"),
            ),
            iaps: pulumi_gestalt_rust::__private::into_domain(o.extract_field("iaps")),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            ip_address_selection_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipAddressSelectionPolicy"),
            ),
            load_balancing_scheme: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("loadBalancingScheme"),
            ),
            locality_lb_policies: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("localityLbPolicies"),
            ),
            locality_lb_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("localityLbPolicy"),
            ),
            log_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("logConfigs"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            outlier_detections: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("outlierDetections"),
            ),
            port_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("portName"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            protocol: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("protocol"),
            ),
            security_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("securityPolicy"),
            ),
            security_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("securitySettings"),
            ),
            self_link: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            service_lb_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceLbPolicy"),
            ),
            session_affinity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sessionAffinity"),
            ),
            strong_session_affinity_cookies: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("strongSessionAffinityCookies"),
            ),
            timeout_sec: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timeoutSec"),
            ),
        }
    }
}
