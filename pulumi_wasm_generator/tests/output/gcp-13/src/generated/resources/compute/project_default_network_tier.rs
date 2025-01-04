/// Configures the Google Compute Engine
/// [Default Network Tier](https://cloud.google.com/network-tiers/docs/using-network-service-tiers#setting_the_tier_for_all_resources_in_a_project)
/// for a project.
///
/// For more information, see,
/// [the Project API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/projects/setDefaultNetworkTier).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = project_default_network_tier::create(
///         "default",
///         ProjectDefaultNetworkTierArgs::builder().network_tier("PREMIUM").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Compute Engine Default Network Tier can be imported using any of these accepted formats:
///
/// * `{{project_id}}`
///
/// When using the `pulumi import` command, Compute Engine Default Network Tier can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/projectDefaultNetworkTier:ProjectDefaultNetworkTier default {{project_id}}
/// ```
///
pub mod project_default_network_tier {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProjectDefaultNetworkTierArgs {
        /// The default network tier to be configured for the project.
        /// This field can take the following values: `PREMIUM` or `STANDARD`.
        ///
        /// - - -
        #[builder(into)]
        pub network_tier: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ProjectDefaultNetworkTierResult {
        /// The default network tier to be configured for the project.
        /// This field can take the following values: `PREMIUM` or `STANDARD`.
        ///
        /// - - -
        pub network_tier: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ProjectDefaultNetworkTierArgs,
    ) -> ProjectDefaultNetworkTierResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let network_tier_binding = args.network_tier.get_inner();
        let project_binding = args.project.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/projectDefaultNetworkTier:ProjectDefaultNetworkTier"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "networkTier".into(),
                    value: &network_tier_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "networkTier".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ProjectDefaultNetworkTierResult {
            network_tier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkTier").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
        }
    }
}
