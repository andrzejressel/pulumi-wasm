/// Manage a single IngressPolicy in the spec (dry-run) configuration for a service perimeter.
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
/// the service perimeter resource must have a `lifecycle` block with `ignore_changes = [spec[0].ingress_policies]` so
/// they don't fight over which ingress rules should be in the policy.
///
///
/// To get more information about ServicePerimeterDryRunIngressPolicy, see:
///
/// * [API documentation](https://cloud.google.com/access-context-manager/docs/reference/rest/v1/accessPolicies.servicePerimeters#ingresspolicy)
/// * How-to Guides
///     * [Guide to Ingress and Egress Rules](https://cloud.google.com/vpc-service-controls/docs/ingress-egress-rules)
///
/// ## Example Usage
///
#[allow(clippy::doc_lazy_continuation)]
pub mod service_perimeter_dry_run_ingress_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServicePerimeterDryRunIngressPolicyArgs {
        /// Defines the conditions on the source of a request causing this `IngressPolicy`
        /// to apply.
        /// Structure is documented below.
        #[builder(into, default)]
        pub ingress_from: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::accesscontextmanager::ServicePerimeterDryRunIngressPolicyIngressFrom,
            >,
        >,
        /// Defines the conditions on the `ApiOperation` and request destination that cause
        /// this `IngressPolicy` to apply.
        /// Structure is documented below.
        #[builder(into, default)]
        pub ingress_to: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::accesscontextmanager::ServicePerimeterDryRunIngressPolicyIngressTo,
            >,
        >,
        /// The name of the Service Perimeter to add this resource to.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub perimeter: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ServicePerimeterDryRunIngressPolicyResult {
        /// Defines the conditions on the source of a request causing this `IngressPolicy`
        /// to apply.
        /// Structure is documented below.
        pub ingress_from: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::accesscontextmanager::ServicePerimeterDryRunIngressPolicyIngressFrom,
            >,
        >,
        /// Defines the conditions on the `ApiOperation` and request destination that cause
        /// this `IngressPolicy` to apply.
        /// Structure is documented below.
        pub ingress_to: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::accesscontextmanager::ServicePerimeterDryRunIngressPolicyIngressTo,
            >,
        >,
        /// The name of the Service Perimeter to add this resource to.
        ///
        ///
        /// - - -
        pub perimeter: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ServicePerimeterDryRunIngressPolicyArgs,
    ) -> ServicePerimeterDryRunIngressPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let ingress_from_binding = args.ingress_from.get_output(context).get_inner();
        let ingress_to_binding = args.ingress_to.get_output(context).get_inner();
        let perimeter_binding = args.perimeter.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:accesscontextmanager/servicePerimeterDryRunIngressPolicy:ServicePerimeterDryRunIngressPolicy"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        ServicePerimeterDryRunIngressPolicyResult {
            ingress_from: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ingressFrom"),
            ),
            ingress_to: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ingressTo"),
            ),
            perimeter: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("perimeter"),
            ),
        }
    }
}
