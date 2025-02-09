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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod addons_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AddonsConfigArgs {
        /// Addon configurations of the Apigee organization.
        /// Structure is documented below.
        #[builder(into, default)]
        pub addons_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apigee::AddonsConfigAddonsConfig>,
        >,
        /// Name of the Apigee organization.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub org: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AddonsConfigResult {
        /// Addon configurations of the Apigee organization.
        /// Structure is documented below.
        pub addons_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::apigee::AddonsConfigAddonsConfig>,
        >,
        /// Name of the Apigee organization.
        ///
        ///
        /// - - -
        pub org: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AddonsConfigArgs,
    ) -> AddonsConfigResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let addons_config_binding = args.addons_config.get_output(context);
        let org_binding = args.org.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:apigee/addonsConfig:AddonsConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "addonsConfig".into(),
                    value: addons_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "org".into(),
                    value: org_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AddonsConfigResult {
            addons_config: o.get_field("addonsConfig"),
            org: o.get_field("org"),
        }
    }
}
