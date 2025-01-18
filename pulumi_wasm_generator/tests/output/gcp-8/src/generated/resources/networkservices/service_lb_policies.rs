/// ## Example Usage
///
/// ### Network Services Service Lb Policies Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = service_lb_policies::create(
///         "default",
///         ServiceLbPoliciesArgs::builder()
///             .location("global")
///             .name("my-lb-policy")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Network Services Service Lb Policies Advanced
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:networkservices:ServiceLbPolicies
///     properties:
///       name: my-lb-policy
///       location: global
///       description: my description
///       loadBalancingAlgorithm: SPRAY_TO_REGION
///       autoCapacityDrain:
///         enable: true
///       failoverConfig:
///         failoverHealthThreshold: 70
///       labels:
///         foo: bar
///   defaultBackendService:
///     type: gcp:compute:BackendService
///     name: default
///     properties:
///       name: my-lb-backend
///       description: my description
///       loadBalancingScheme: INTERNAL_SELF_MANAGED
///       protocol: HTTP
///       serviceLbPolicy: //networkservices.googleapis.com/${default.id}
/// ```
///
/// ## Import
///
/// ServiceLbPolicies can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/serviceLbPolicies/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, ServiceLbPolicies can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:networkservices/serviceLbPolicies:ServiceLbPolicies default projects/{{project}}/locations/{{location}}/serviceLbPolicies/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkservices/serviceLbPolicies:ServiceLbPolicies default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkservices/serviceLbPolicies:ServiceLbPolicies default {{location}}/{{name}}
/// ```
///
pub mod service_lb_policies {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceLbPoliciesArgs {
        /// Option to specify if an unhealthy MIG/NEG should be considered for global load balancing and traffic routing.
        /// Structure is documented below.
        #[builder(into, default)]
        pub auto_capacity_drain: pulumi_wasm_rust::Output<
            Option<
                super::super::types::networkservices::ServiceLbPoliciesAutoCapacityDrain,
            >,
        >,
        /// A free-text description of the resource. Max length 1024 characters.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Option to specify health based failover behavior. This is not related to Network load balancer FailoverPolicy.
        /// Structure is documented below.
        #[builder(into, default)]
        pub failover_config: pulumi_wasm_rust::Output<
            Option<super::super::types::networkservices::ServiceLbPoliciesFailoverConfig>,
        >,
        /// Set of label tags associated with the ServiceLbPolicy resource.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of load balancing algorithm to be used. The default behavior is WATERFALL_BY_REGION.
        /// Possible values are: `SPRAY_TO_REGION`, `SPRAY_TO_WORLD`, `WATERFALL_BY_REGION`, `WATERFALL_BY_ZONE`.
        #[builder(into, default)]
        pub load_balancing_algorithm: pulumi_wasm_rust::Output<Option<String>>,
        /// The location of the service lb policy.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// Name of the ServiceLbPolicy resource. It matches pattern `projects/{project}/locations/{location}/serviceLbPolicies/{service_lb_policy_name}`.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ServiceLbPoliciesResult {
        /// Option to specify if an unhealthy MIG/NEG should be considered for global load balancing and traffic routing.
        /// Structure is documented below.
        pub auto_capacity_drain: pulumi_wasm_rust::Output<
            Option<
                super::super::types::networkservices::ServiceLbPoliciesAutoCapacityDrain,
            >,
        >,
        /// Time the ServiceLbPolicy was created in UTC.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// A free-text description of the resource. Max length 1024 characters.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Option to specify health based failover behavior. This is not related to Network load balancer FailoverPolicy.
        /// Structure is documented below.
        pub failover_config: pulumi_wasm_rust::Output<
            Option<super::super::types::networkservices::ServiceLbPoliciesFailoverConfig>,
        >,
        /// Set of label tags associated with the ServiceLbPolicy resource.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of load balancing algorithm to be used. The default behavior is WATERFALL_BY_REGION.
        /// Possible values are: `SPRAY_TO_REGION`, `SPRAY_TO_WORLD`, `WATERFALL_BY_REGION`, `WATERFALL_BY_ZONE`.
        pub load_balancing_algorithm: pulumi_wasm_rust::Output<Option<String>>,
        /// The location of the service lb policy.
        ///
        ///
        /// - - -
        pub location: pulumi_wasm_rust::Output<String>,
        /// Name of the ServiceLbPolicy resource. It matches pattern `projects/{project}/locations/{location}/serviceLbPolicies/{service_lb_policy_name}`.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Time the ServiceLbPolicy was updated in UTC.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ServiceLbPoliciesArgs) -> ServiceLbPoliciesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let auto_capacity_drain_binding = args.auto_capacity_drain.get_inner();
        let description_binding = args.description.get_inner();
        let failover_config_binding = args.failover_config.get_inner();
        let labels_binding = args.labels.get_inner();
        let load_balancing_algorithm_binding = args.load_balancing_algorithm.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:networkservices/serviceLbPolicies:ServiceLbPolicies".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autoCapacityDrain".into(),
                    value: &auto_capacity_drain_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "failoverConfig".into(),
                    value: &failover_config_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "loadBalancingAlgorithm".into(),
                    value: &load_balancing_algorithm_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
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
                    name: "autoCapacityDrain".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "failoverConfig".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "loadBalancingAlgorithm".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ServiceLbPoliciesResult {
            auto_capacity_drain: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoCapacityDrain").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            failover_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("failoverConfig").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            load_balancing_algorithm: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loadBalancingAlgorithm").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
