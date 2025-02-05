/// Provides a Route53 traffic policy instance resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod traffic_policy_instance {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TrafficPolicyInstanceArgs {
        /// ID of the hosted zone that you want Amazon Route 53 to create resource record sets in by using the configuration in a traffic policy.
        #[builder(into)]
        pub hosted_zone_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Domain name for which Amazon Route 53 responds to DNS queries by using the resource record sets that Route 53 creates for this traffic policy instance.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// ID of the traffic policy that you want to use to create resource record sets in the specified hosted zone.
        #[builder(into)]
        pub traffic_policy_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Version of the traffic policy
        #[builder(into)]
        pub traffic_policy_version: pulumi_wasm_rust::InputOrOutput<i32>,
        /// TTL that you want Amazon Route 53 to assign to all the resource record sets that it creates in the specified hosted zone.
        #[builder(into)]
        pub ttl: pulumi_wasm_rust::InputOrOutput<i32>,
    }
    #[allow(dead_code)]
    pub struct TrafficPolicyInstanceResult {
        /// ID of the hosted zone that you want Amazon Route 53 to create resource record sets in by using the configuration in a traffic policy.
        pub hosted_zone_id: pulumi_wasm_rust::Output<String>,
        /// Domain name for which Amazon Route 53 responds to DNS queries by using the resource record sets that Route 53 creates for this traffic policy instance.
        pub name: pulumi_wasm_rust::Output<String>,
        /// ID of the traffic policy that you want to use to create resource record sets in the specified hosted zone.
        pub traffic_policy_id: pulumi_wasm_rust::Output<String>,
        /// Version of the traffic policy
        pub traffic_policy_version: pulumi_wasm_rust::Output<i32>,
        /// TTL that you want Amazon Route 53 to assign to all the resource record sets that it creates in the specified hosted zone.
        pub ttl: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: TrafficPolicyInstanceArgs,
    ) -> TrafficPolicyInstanceResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let hosted_zone_id_binding = args.hosted_zone_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let traffic_policy_id_binding = args
            .traffic_policy_id
            .get_output(context)
            .get_inner();
        let traffic_policy_version_binding = args
            .traffic_policy_version
            .get_output(context)
            .get_inner();
        let ttl_binding = args.ttl.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:route53/trafficPolicyInstance:TrafficPolicyInstance".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "hostedZoneId".into(),
                    value: &hosted_zone_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "trafficPolicyId".into(),
                    value: &traffic_policy_id_binding,
                },
                register_interface::ObjectField {
                    name: "trafficPolicyVersion".into(),
                    value: &traffic_policy_version_binding,
                },
                register_interface::ObjectField {
                    name: "ttl".into(),
                    value: &ttl_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        TrafficPolicyInstanceResult {
            hosted_zone_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("hostedZoneId"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            traffic_policy_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("trafficPolicyId"),
            ),
            traffic_policy_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("trafficPolicyVersion"),
            ),
            ttl: pulumi_wasm_rust::__private::into_domain(o.extract_field("ttl")),
        }
    }
}
