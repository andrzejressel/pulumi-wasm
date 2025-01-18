/// Manages an Application Gateway for Containers Frontend.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = lication_load_balancer::create(
///         "example",
///         LicationLoadBalancerArgs::builder()
///             .location("West Europe")
///             .name("example")
///             .resource_group_name("example")
///             .build_struct(),
///     );
///     let exampleLicationLoadBalancerFrontend = lication_load_balancer_frontend::create(
///         "exampleLicationLoadBalancerFrontend",
///         LicationLoadBalancerFrontendArgs::builder()
///             .application_load_balancer_id("${example.id}")
///             .name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Application Gateway for Containers Frontend can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appconfiguration/licationLoadBalancerFrontend:LicationLoadBalancerFrontend example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.ServiceNetworking/trafficControllers/alb1/frontends/frontend1
/// ```
///
pub mod lication_load_balancer_frontend {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LicationLoadBalancerFrontendArgs {
        /// The ID of the Application Gateway for Containers. Changing this forces a new resource to be created.
        #[builder(into)]
        pub application_load_balancer_id: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Application Gateway for Containers Frontend. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of tags which should be assigned to the Application Gateway for Containers Frontend.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct LicationLoadBalancerFrontendResult {
        /// The ID of the Application Gateway for Containers. Changing this forces a new resource to be created.
        pub application_load_balancer_id: pulumi_wasm_rust::Output<String>,
        /// The Fully Qualified Domain Name of the DNS record associated to an Application Gateway for Containers Frontend.
        pub fully_qualified_domain_name: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Application Gateway for Containers Frontend. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Application Gateway for Containers Frontend.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: LicationLoadBalancerFrontendArgs,
    ) -> LicationLoadBalancerFrontendResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let application_load_balancer_id_binding = args
            .application_load_balancer_id
            .get_inner();
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appconfiguration/licationLoadBalancerFrontend:LicationLoadBalancerFrontend"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationLoadBalancerId".into(),
                    value: &application_load_balancer_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "applicationLoadBalancerId".into(),
                },
                register_interface::ResultField {
                    name: "fullyQualifiedDomainName".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LicationLoadBalancerFrontendResult {
            application_load_balancer_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationLoadBalancerId").unwrap(),
            ),
            fully_qualified_domain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fullyQualifiedDomainName").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
