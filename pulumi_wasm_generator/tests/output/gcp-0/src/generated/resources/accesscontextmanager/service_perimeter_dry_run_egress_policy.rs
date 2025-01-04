/// Manage a single EgressPolicy in the spec (dry-run) configuration for a service perimeter.
/// EgressPolicies match requests based on egressFrom and egressTo stanzas.
/// For an EgressPolicy to match, both egressFrom and egressTo stanzas must be matched.
/// If an EgressPolicy matches a request, the request is allowed to span the ServicePerimeter
/// boundary. For example, an EgressPolicy can be used to allow VMs on networks
/// within the ServicePerimeter to access a defined set of projects outside the
/// perimeter in certain contexts (e.g. to read data from a Cloud Storage bucket
/// or query against a BigQuery dataset).
///
/// > **Note:** By default, updates to this resource will remove the EgressPolicy from the
/// from the perimeter and add it back in a non-atomic manner. To ensure that the new EgressPolicy
/// is added before the old one is removed, add a `lifecycle` block with `create_before_destroy = true` to this resource.
/// > **Note:** If this resource is used alongside a `gcp.accesscontextmanager.ServicePerimeter` resource,
/// the service perimeter resource must have a `lifecycle` block with `ignore_changes = [spec[0].egress_policies]` so
/// they don't fight over which egress rules should be in the policy.
///
///
/// To get more information about ServicePerimeterDryRunEgressPolicy, see:
///
/// * [API documentation](https://cloud.google.com/access-context-manager/docs/reference/rest/v1/accessPolicies.servicePerimeters#egresspolicy)
/// * How-to Guides
///     * [Guide to Ingress and Egress Rules](https://cloud.google.com/vpc-service-controls/docs/ingress-egress-rules)
///
/// ## Example Usage
///
pub mod service_perimeter_dry_run_egress_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServicePerimeterDryRunEgressPolicyArgs {
        /// Defines conditions on the source of a request causing this `EgressPolicy` to apply.
        /// Structure is documented below.
        #[builder(into, default)]
        pub egress_from: pulumi_wasm_rust::Output<
            Option<
                super::super::types::accesscontextmanager::ServicePerimeterDryRunEgressPolicyEgressFrom,
            >,
        >,
        /// Defines the conditions on the `ApiOperation` and destination resources that
        /// cause this `EgressPolicy` to apply.
        /// Structure is documented below.
        #[builder(into, default)]
        pub egress_to: pulumi_wasm_rust::Output<
            Option<
                super::super::types::accesscontextmanager::ServicePerimeterDryRunEgressPolicyEgressTo,
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
    pub struct ServicePerimeterDryRunEgressPolicyResult {
        /// Defines conditions on the source of a request causing this `EgressPolicy` to apply.
        /// Structure is documented below.
        pub egress_from: pulumi_wasm_rust::Output<
            Option<
                super::super::types::accesscontextmanager::ServicePerimeterDryRunEgressPolicyEgressFrom,
            >,
        >,
        /// Defines the conditions on the `ApiOperation` and destination resources that
        /// cause this `EgressPolicy` to apply.
        /// Structure is documented below.
        pub egress_to: pulumi_wasm_rust::Output<
            Option<
                super::super::types::accesscontextmanager::ServicePerimeterDryRunEgressPolicyEgressTo,
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
        args: ServicePerimeterDryRunEgressPolicyArgs,
    ) -> ServicePerimeterDryRunEgressPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let egress_from_binding = args.egress_from.get_inner();
        let egress_to_binding = args.egress_to.get_inner();
        let perimeter_binding = args.perimeter.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:accesscontextmanager/servicePerimeterDryRunEgressPolicy:ServicePerimeterDryRunEgressPolicy"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "egressFrom".into(),
                    value: &egress_from_binding,
                },
                register_interface::ObjectField {
                    name: "egressTo".into(),
                    value: &egress_to_binding,
                },
                register_interface::ObjectField {
                    name: "perimeter".into(),
                    value: &perimeter_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "egressFrom".into(),
                },
                register_interface::ResultField {
                    name: "egressTo".into(),
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
        ServicePerimeterDryRunEgressPolicyResult {
            egress_from: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("egressFrom").unwrap(),
            ),
            egress_to: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("egressTo").unwrap(),
            ),
            perimeter: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("perimeter").unwrap(),
            ),
        }
    }
}
