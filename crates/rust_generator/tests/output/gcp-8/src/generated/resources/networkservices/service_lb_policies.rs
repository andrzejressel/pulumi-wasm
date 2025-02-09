/// ## Example Usage
///
/// ### Network Services Service Lb Policies Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod service_lb_policies {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceLbPoliciesArgs {
        /// Option to specify if an unhealthy MIG/NEG should be considered for global load balancing and traffic routing.
        /// Structure is documented below.
        #[builder(into, default)]
        pub auto_capacity_drain: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::networkservices::ServiceLbPoliciesAutoCapacityDrain,
            >,
        >,
        /// A free-text description of the resource. Max length 1024 characters.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Option to specify health based failover behavior. This is not related to Network load balancer FailoverPolicy.
        /// Structure is documented below.
        #[builder(into, default)]
        pub failover_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::networkservices::ServiceLbPoliciesFailoverConfig>,
        >,
        /// Set of label tags associated with the ServiceLbPolicy resource.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of load balancing algorithm to be used. The default behavior is WATERFALL_BY_REGION.
        /// Possible values are: `SPRAY_TO_REGION`, `SPRAY_TO_WORLD`, `WATERFALL_BY_REGION`, `WATERFALL_BY_ZONE`.
        #[builder(into, default)]
        pub load_balancing_algorithm: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The location of the service lb policy.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the ServiceLbPolicy resource. It matches pattern `projects/{project}/locations/{location}/serviceLbPolicies/{service_lb_policy_name}`.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ServiceLbPoliciesResult {
        /// Option to specify if an unhealthy MIG/NEG should be considered for global load balancing and traffic routing.
        /// Structure is documented below.
        pub auto_capacity_drain: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::networkservices::ServiceLbPoliciesAutoCapacityDrain,
            >,
        >,
        /// Time the ServiceLbPolicy was created in UTC.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// A free-text description of the resource. Max length 1024 characters.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Option to specify health based failover behavior. This is not related to Network load balancer FailoverPolicy.
        /// Structure is documented below.
        pub failover_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::networkservices::ServiceLbPoliciesFailoverConfig>,
        >,
        /// Set of label tags associated with the ServiceLbPolicy resource.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of load balancing algorithm to be used. The default behavior is WATERFALL_BY_REGION.
        /// Possible values are: `SPRAY_TO_REGION`, `SPRAY_TO_WORLD`, `WATERFALL_BY_REGION`, `WATERFALL_BY_ZONE`.
        pub load_balancing_algorithm: pulumi_gestalt_rust::Output<Option<String>>,
        /// The location of the service lb policy.
        ///
        ///
        /// - - -
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Name of the ServiceLbPolicy resource. It matches pattern `projects/{project}/locations/{location}/serviceLbPolicies/{service_lb_policy_name}`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Time the ServiceLbPolicy was updated in UTC.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServiceLbPoliciesArgs,
    ) -> ServiceLbPoliciesResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let auto_capacity_drain_binding = args.auto_capacity_drain.get_output(context);
        let description_binding = args.description.get_output(context);
        let failover_config_binding = args.failover_config.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let load_balancing_algorithm_binding = args
            .load_balancing_algorithm
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:networkservices/serviceLbPolicies:ServiceLbPolicies".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoCapacityDrain".into(),
                    value: auto_capacity_drain_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "failoverConfig".into(),
                    value: failover_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: labels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "loadBalancingAlgorithm".into(),
                    value: load_balancing_algorithm_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ServiceLbPoliciesResult {
            auto_capacity_drain: o.get_field("autoCapacityDrain"),
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            effective_labels: o.get_field("effectiveLabels"),
            failover_config: o.get_field("failoverConfig"),
            labels: o.get_field("labels"),
            load_balancing_algorithm: o.get_field("loadBalancingAlgorithm"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            update_time: o.get_field("updateTime"),
        }
    }
}
