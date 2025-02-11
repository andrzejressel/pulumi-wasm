#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        args: GetBackendServiceArgs,
    ) -> GetBackendServiceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:compute/getBackendService:getBackendService".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetBackendServiceResult {
            affinity_cookie_ttl_sec: o.get_field("affinityCookieTtlSec"),
            backends: o.get_field("backends"),
            cdn_policies: o.get_field("cdnPolicies"),
            circuit_breakers: o.get_field("circuitBreakers"),
            compression_mode: o.get_field("compressionMode"),
            connection_draining_timeout_sec: o.get_field("connectionDrainingTimeoutSec"),
            consistent_hash: o.get_field("consistentHash"),
            creation_timestamp: o.get_field("creationTimestamp"),
            custom_request_headers: o.get_field("customRequestHeaders"),
            custom_response_headers: o.get_field("customResponseHeaders"),
            description: o.get_field("description"),
            edge_security_policy: o.get_field("edgeSecurityPolicy"),
            enable_cdn: o.get_field("enableCdn"),
            fingerprint: o.get_field("fingerprint"),
            generated_id: o.get_field("generatedId"),
            health_checks: o.get_field("healthChecks"),
            iaps: o.get_field("iaps"),
            id: o.get_field("id"),
            ip_address_selection_policy: o.get_field("ipAddressSelectionPolicy"),
            load_balancing_scheme: o.get_field("loadBalancingScheme"),
            locality_lb_policies: o.get_field("localityLbPolicies"),
            locality_lb_policy: o.get_field("localityLbPolicy"),
            log_configs: o.get_field("logConfigs"),
            name: o.get_field("name"),
            outlier_detections: o.get_field("outlierDetections"),
            port_name: o.get_field("portName"),
            project: o.get_field("project"),
            protocol: o.get_field("protocol"),
            security_policy: o.get_field("securityPolicy"),
            security_settings: o.get_field("securitySettings"),
            self_link: o.get_field("selfLink"),
            service_lb_policy: o.get_field("serviceLbPolicy"),
            session_affinity: o.get_field("sessionAffinity"),
            strong_session_affinity_cookies: o.get_field("strongSessionAffinityCookies"),
            timeout_sec: o.get_field("timeoutSec"),
        }
    }
}
