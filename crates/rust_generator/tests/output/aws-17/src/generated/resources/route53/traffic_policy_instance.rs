/// Provides a Route53 traffic policy instance resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = traffic_policy_instance::create(
///         "test",
///         TrafficPolicyInstanceArgs::builder()
///             .hosted_zone_id("Z033120931TAQO548OGJC")
///             .name("test.example.com")
///             .traffic_policy_id("b3gb108f-ea6f-45a5-baab-9d112d8b4037")
///             .traffic_policy_version(1)
///             .ttl(360)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Route53 traffic policy instance using its id. For example:
///
/// ```sh
/// $ pulumi import aws:route53/trafficPolicyInstance:TrafficPolicyInstance test df579d9a-6396-410e-ac22-e7ad60cf9e7e
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod traffic_policy_instance {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TrafficPolicyInstanceArgs {
        /// ID of the hosted zone that you want Amazon Route 53 to create resource record sets in by using the configuration in a traffic policy.
        #[builder(into)]
        pub hosted_zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Domain name for which Amazon Route 53 responds to DNS queries by using the resource record sets that Route 53 creates for this traffic policy instance.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ID of the traffic policy that you want to use to create resource record sets in the specified hosted zone.
        #[builder(into)]
        pub traffic_policy_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Version of the traffic policy
        #[builder(into)]
        pub traffic_policy_version: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// TTL that you want Amazon Route 53 to assign to all the resource record sets that it creates in the specified hosted zone.
        #[builder(into)]
        pub ttl: pulumi_gestalt_rust::InputOrOutput<i32>,
    }
    #[allow(dead_code)]
    pub struct TrafficPolicyInstanceResult {
        /// ID of the hosted zone that you want Amazon Route 53 to create resource record sets in by using the configuration in a traffic policy.
        pub hosted_zone_id: pulumi_gestalt_rust::Output<String>,
        /// Domain name for which Amazon Route 53 responds to DNS queries by using the resource record sets that Route 53 creates for this traffic policy instance.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// ID of the traffic policy that you want to use to create resource record sets in the specified hosted zone.
        pub traffic_policy_id: pulumi_gestalt_rust::Output<String>,
        /// Version of the traffic policy
        pub traffic_policy_version: pulumi_gestalt_rust::Output<i32>,
        /// TTL that you want Amazon Route 53 to assign to all the resource record sets that it creates in the specified hosted zone.
        pub ttl: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TrafficPolicyInstanceArgs,
    ) -> TrafficPolicyInstanceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let hosted_zone_id_binding = args.hosted_zone_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let traffic_policy_id_binding = args.traffic_policy_id.get_output(context);
        let traffic_policy_version_binding = args
            .traffic_policy_version
            .get_output(context);
        let ttl_binding = args.ttl.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:route53/trafficPolicyInstance:TrafficPolicyInstance".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hostedZoneId".into(),
                    value: hosted_zone_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "trafficPolicyId".into(),
                    value: traffic_policy_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "trafficPolicyVersion".into(),
                    value: traffic_policy_version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ttl".into(),
                    value: ttl_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        TrafficPolicyInstanceResult {
            hosted_zone_id: o.get_field("hostedZoneId"),
            name: o.get_field("name"),
            traffic_policy_id: o.get_field("trafficPolicyId"),
            traffic_policy_version: o.get_field("trafficPolicyVersion"),
            ttl: o.get_field("ttl"),
        }
    }
}
