/// Manages an Application Gateway for Containers (ALB).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleLicationLoadBalancer = lication_load_balancer::create(
///         "exampleLicationLoadBalancer",
///         LicationLoadBalancerArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Application Gateway for Containers (ALB) can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appconfiguration/licationLoadBalancer:LicationLoadBalancer example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.ServiceNetworking/trafficControllers/alb1
/// ```
///
pub mod lication_load_balancer {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LicationLoadBalancerArgs {
        /// The Azure Region where the Application Gateway for Containers (ALB) should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Application Gateway for Containers (ALB). Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of Resource Group where the Application Gateway for Containers (ALB) should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Application Gateway for Containers (ALB).
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct LicationLoadBalancerResult {
        /// The Azure Region where the Application Gateway for Containers (ALB) should exist. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Application Gateway for Containers (ALB). Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The primary configuration endpoints of the Application Gateway for Containers (ALB).
        pub primary_configuration_endpoint: pulumi_wasm_rust::Output<String>,
        /// The name of Resource Group where the Application Gateway for Containers (ALB) should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Application Gateway for Containers (ALB).
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
        args: LicationLoadBalancerArgs,
    ) -> LicationLoadBalancerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appconfiguration/licationLoadBalancer:LicationLoadBalancer"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "primaryConfigurationEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
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
        LicationLoadBalancerResult {
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            primary_configuration_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryConfigurationEndpoint").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}