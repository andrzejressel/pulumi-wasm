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
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProjectDefaultNetworkTierArgs {
        /// The default network tier to be configured for the project.
        /// This field can take the following values: `PREMIUM` or `STANDARD`.
        ///
        /// - - -
        #[builder(into)]
        pub network_tier: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
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
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ProjectDefaultNetworkTierArgs,
    ) -> ProjectDefaultNetworkTierResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let network_tier_binding = args.network_tier.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/projectDefaultNetworkTier:ProjectDefaultNetworkTier"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        ProjectDefaultNetworkTierResult {
            network_tier: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("networkTier"),
            ),
            project: pulumi_wasm_rust::__private::into_domain(o.extract_field("project")),
        }
    }
}
