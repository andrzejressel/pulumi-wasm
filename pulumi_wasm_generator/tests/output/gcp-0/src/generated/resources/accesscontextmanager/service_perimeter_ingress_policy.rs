/// Manage a single IngressPolicy in the status (enforced) configuration for a service perimeter.
/// IngressPolicies match requests based on ingressFrom and ingressTo stanzas. For an ingress policy to match,
/// both the ingressFrom and ingressTo stanzas must be matched. If an IngressPolicy matches a request,
/// the request is allowed through the perimeter boundary from outside the perimeter.
/// For example, access from the internet can be allowed either based on an AccessLevel or,
/// for traffic hosted on Google Cloud, the project of the source network.
/// For access from private networks, using the project of the hosting network is required.
/// Individual ingress policies can be limited by restricting which services and/
/// or actions they match using the ingressTo field.
///
/// > **Note:** By default, updates to this resource will remove the IngressPolicy from the
/// from the perimeter and add it back in a non-atomic manner. To ensure that the new IngressPolicy
/// is added before the old one is removed, add a `lifecycle` block with `create_before_destroy = true` to this resource.
/// > **Note:** If this resource is used alongside a `gcp.accesscontextmanager.ServicePerimeter` resource,
/// the service perimeter resource must have a `lifecycle` block with `ignore_changes = [status[0].ingress_policies]` so
/// they don't fight over which ingress rules should be in the policy.
///
///
/// To get more information about ServicePerimeterIngressPolicy, see:
///
/// * [API documentation](https://cloud.google.com/access-context-manager/docs/reference/rest/v1/accessPolicies.servicePerimeters#ingresspolicy)
/// * How-to Guides
///     * [Guide to Ingress and Egress Rules](https://cloud.google.com/vpc-service-controls/docs/ingress-egress-rules)
///
/// ## Example Usage
///
pub mod service_perimeter_ingress_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServicePerimeterIngressPolicyArgs {
        /// Defines the conditions on the source of a request causing this `IngressPolicy`
        /// to apply.
        /// Structure is documented below.
        #[builder(into, default)]
        pub ingress_from: pulumi_wasm_rust::Output<
            Option<
                super::super::types::accesscontextmanager::ServicePerimeterIngressPolicyIngressFrom,
            >,
        >,
        /// Defines the conditions on the `ApiOperation` and request destination that cause
        /// this `IngressPolicy` to apply.
        /// Structure is documented below.
        #[builder(into, default)]
        pub ingress_to: pulumi_wasm_rust::Output<
            Option<
                super::super::types::accesscontextmanager::ServicePerimeterIngressPolicyIngressTo,
            >,
        >,
        /// The name of the Service Perimeter to add this resource to.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub perimeter: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ServicePerimeterIngressPolicyResult {
        /// Defines the conditions on the source of a request causing this `IngressPolicy`
        /// to apply.
        /// Structure is documented below.
        pub ingress_from: pulumi_wasm_rust::Output<
            Option<
                super::super::types::accesscontextmanager::ServicePerimeterIngressPolicyIngressFrom,
            >,
        >,
        /// Defines the conditions on the `ApiOperation` and request destination that cause
        /// this `IngressPolicy` to apply.
        /// Structure is documented below.
        pub ingress_to: pulumi_wasm_rust::Output<
            Option<
                super::super::types::accesscontextmanager::ServicePerimeterIngressPolicyIngressTo,
            >,
        >,
        /// The name of the Service Perimeter to add this resource to.
        ///
        ///
        /// - - -
        pub perimeter: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ServicePerimeterIngressPolicyArgs,
    ) -> ServicePerimeterIngressPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let ingress_from_binding = args.ingress_from.get_inner();
        let ingress_to_binding = args.ingress_to.get_inner();
        let perimeter_binding = args.perimeter.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:accesscontextmanager/servicePerimeterIngressPolicy:ServicePerimeterIngressPolicy"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "ingressFrom".into(),
                    value: &ingress_from_binding,
                },
                register_interface::ObjectField {
                    name: "ingressTo".into(),
                    value: &ingress_to_binding,
                },
                register_interface::ObjectField {
                    name: "perimeter".into(),
                    value: &perimeter_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "ingressFrom".into(),
                },
                register_interface::ResultField {
                    name: "ingressTo".into(),
                },
                register_interface::ResultField {
                    name: "perimeter".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ServicePerimeterIngressPolicyResult {
            ingress_from: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ingressFrom").unwrap(),
            ),
            ingress_to: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ingressTo").unwrap(),
            ),
            perimeter: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("perimeter").unwrap(),
            ),
        }
    }
}
