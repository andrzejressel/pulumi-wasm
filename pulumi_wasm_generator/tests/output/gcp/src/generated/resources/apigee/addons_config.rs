/// Configures the add-ons for the Apigee organization. The existing add-on configuration will be fully replaced.
///
///
/// To get more information about AddonsConfig, see:
///
/// * [API documentation](https://cloud.google.com/apigee/docs/reference/apis/apigee/rest/v1/organizations#setaddons)
/// * How-to Guides
///     * [Creating an API organization](https://cloud.google.com/apigee/docs/api-platform/get-started/create-org)
///
/// ## Example Usage
///
/// ### Apigee Addons Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let testOrganization = addons_config::create(
///         "testOrganization",
///         AddonsConfigArgs::builder()
///             .addons_config(
///                 AddonsConfigAddonsConfig::builder()
///                     .apiSecurityConfig(
///                         AddonsConfigAddonsConfigApiSecurityConfig::builder()
///                             .enabled(true)
///                             .build_struct(),
///                     )
///                     .monetizationConfig(
///                         AddonsConfigAddonsConfigMonetizationConfig::builder()
///                             .enabled(true)
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .org("test_organization")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Apigee Addons Full
///
///
/// ```yaml
/// resources:
///   apigee:
///     type: gcp:projects:Service
///     properties:
///       project: ${current.project}
///       service: apigee.googleapis.com
///   compute:
///     type: gcp:projects:Service
///     properties:
///       project: ${current.project}
///       service: compute.googleapis.com
///   servicenetworking:
///     type: gcp:projects:Service
///     properties:
///       project: ${current.project}
///       service: servicenetworking.googleapis.com
///   apigeeNetwork:
///     type: gcp:compute:Network
///     name: apigee_network
///     properties:
///       name: apigee-network
///       project: ${current.project}
///     options:
///       dependsOn:
///         - ${compute}
///   apigeeRange:
///     type: gcp:compute:GlobalAddress
///     name: apigee_range
///     properties:
///       name: apigee-range
///       purpose: VPC_PEERING
///       addressType: INTERNAL
///       prefixLength: 16
///       network: ${apigeeNetwork.id}
///       project: ${current.project}
///   apigeeVpcConnection:
///     type: gcp:servicenetworking:Connection
///     name: apigee_vpc_connection
///     properties:
///       network: ${apigeeNetwork.id}
///       service: servicenetworking.googleapis.com
///       reservedPeeringRanges:
///         - ${apigeeRange.name}
///   org:
///     type: gcp:apigee:Organization
///     properties:
///       analyticsRegion: us-central1
///       projectId: ${current.project}
///       authorizedNetwork: ${apigeeNetwork.id}
///       billingType: EVALUATION
///     options:
///       dependsOn:
///         - ${apigeeVpcConnection}
///         - ${apigee}
///   testOrganization:
///     type: gcp:apigee:AddonsConfig
///     name: test_organization
///     properties:
///       org: ${org.name}
///       addonsConfig:
///         integrationConfig:
///           enabled: true
///         apiSecurityConfig:
///           enabled: true
///         connectorsPlatformConfig:
///           enabled: true
///         monetizationConfig:
///           enabled: true
///         advancedApiOpsConfig:
///           enabled: true
/// variables:
///   current:
///     fn::invoke:
///       function: gcp:organizations:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// AddonsConfig can be imported using any of these accepted formats:
///
/// * `organizations/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, AddonsConfig can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:apigee/addonsConfig:AddonsConfig default organizations/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:apigee/addonsConfig:AddonsConfig default {{name}}
/// ```
///
pub mod addons_config {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AddonsConfigArgs {
        /// Addon configurations of the Apigee organization.
        /// Structure is documented below.
        #[builder(into, default)]
        pub addons_config: pulumi_wasm_rust::Output<
            Option<super::super::types::apigee::AddonsConfigAddonsConfig>,
        >,
        /// Name of the Apigee organization.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub org: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct AddonsConfigResult {
        /// Addon configurations of the Apigee organization.
        /// Structure is documented below.
        pub addons_config: pulumi_wasm_rust::Output<
            Option<super::super::types::apigee::AddonsConfigAddonsConfig>,
        >,
        /// Name of the Apigee organization.
        ///
        ///
        /// - - -
        pub org: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AddonsConfigArgs) -> AddonsConfigResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let addons_config_binding = args.addons_config.get_inner();
        let org_binding = args.org.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:apigee/addonsConfig:AddonsConfig".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "addonsConfig".into(),
                    value: &addons_config_binding,
                },
                register_interface::ObjectField {
                    name: "org".into(),
                    value: &org_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "addonsConfig".into(),
                },
                register_interface::ResultField {
                    name: "org".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AddonsConfigResult {
            addons_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("addonsConfig").unwrap(),
            ),
            org: pulumi_wasm_rust::__private::into_domain(hashmap.remove("org").unwrap()),
        }
    }
}
