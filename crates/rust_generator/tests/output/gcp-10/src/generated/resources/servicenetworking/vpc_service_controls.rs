/// Manages the VPC Service Controls configuration for a service
/// networking connection
///
/// When enabled, Google Cloud makes the following
/// route configuration changes in the service producer VPC network:
/// - Removes the IPv4 default route (destination 0.0.0.0/0,
///   next hop default internet gateway), Google Cloud then creates an
///   IPv4 route for destination 199.36.153.4/30 using the default
///   internet gateway next hop.
/// - Creates Cloud DNS managed private zones and authorizes those zones
///   for the service producer VPC network. The zones include
///   googleapis.com, gcr.io, pkg.dev, notebooks.cloud.google.com,
///   kernels.googleusercontent.com, backupdr.cloud.google.com, and
///   backupdr.googleusercontent.com as necessary domains or host names
///   for Google APIs and services that are compatible with VPC Service
///   Controls. Record data in the zones resolves all host names to
///   199.36.153.4, 199.36.153.5, 199.36.153.6, and 199.36.153.7.
///
/// When disabled, Google Cloud makes the following route configuration
/// changes in the service producer VPC network:
/// - Restores a default route (destination 0.0.0.0/0, next hop default
///   internet gateway)
/// - Deletes the Cloud DNS managed private zones that provided the host
///   name overrides.
///
///
/// To get more information about VPCServiceControls, see:
///
/// * [API documentation](https://cloud.google.com/service-infrastructure/docs/service-networking/reference/rest/v1/services)
/// * How-to Guides
///     * [Enable VPC Service Controls for service networking](https://cloud.google.com/sdk/gcloud/reference/services/vpc-peerings/enable-vpc-service-controls)
///     * [Private Google Access with VPC Service Controls](https://cloud.google.com/vpc-service-controls/docs/private-connectivity)
///     * [Set up private connectivity to Google APIs and services](https://cloud.google.com/vpc-service-controls/docs/set-up-private-connectivity)
///
/// > **Note:** Destroying a `gcp.servicenetworking.VpcServiceControls`
/// resource will remove it from state, but will not change the
/// underlying VPC Service Controls configuration for the service
/// producer network.
///
/// ## Example Usage
///
/// ### Service Networking Vpc Service Controls Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = network::create(
///         "default",
///         NetworkArgs::builder().name("example-network").build_struct(),
///     );
///     let defaultConnection = connection::create(
///         "defaultConnection",
///         ConnectionArgs::builder()
///             .network("${default.id}")
///             .reserved_peering_ranges(vec!["${defaultGlobalAddress.name}",])
///             .service("servicenetworking.googleapis.com")
///             .build_struct(),
///     );
///     let defaultGlobalAddress = global_address::create(
///         "defaultGlobalAddress",
///         GlobalAddressArgs::builder()
///             .address_type("INTERNAL")
///             .name("psa-range")
///             .network("${default.id}")
///             .prefix_length(16)
///             .purpose("VPC_PEERING")
///             .build_struct(),
///     );
///     let defaultVpcServiceControls = vpc_service_controls::create(
///         "defaultVpcServiceControls",
///         VpcServiceControlsArgs::builder()
///             .enabled(true)
///             .network("${default.name}")
///             .service("servicenetworking.googleapis.com")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// VPCServiceControls can be imported using any of these accepted formats:
///
/// * `services/{{service}}/projects/{{project}}/networks/{{network}}`
///
/// * `{{service}}/{{project}}/{{network}}`
///
/// * `{{service}}/{{network}}`
///
/// When using the `pulumi import` command, VPCServiceControls can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:servicenetworking/vpcServiceControls:VpcServiceControls default services/{{service}}/projects/{{project}}/networks/{{network}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:servicenetworking/vpcServiceControls:VpcServiceControls default {{service}}/{{project}}/{{network}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:servicenetworking/vpcServiceControls:VpcServiceControls default {{service}}/{{network}}
/// ```
///
pub mod vpc_service_controls {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcServiceControlsArgs {
        /// Desired VPC Service Controls state service producer VPC network, as
        /// described at the top of this page.
        #[builder(into)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// The network that the consumer is using to connect with services.
        #[builder(into)]
        pub network: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The id of the Google Cloud project containing the consumer network.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The service that is managing peering connectivity for a service
        /// producer's organization. For Google services that support this
        /// functionality, this value is `servicenetworking.googleapis.com`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub service: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VpcServiceControlsResult {
        /// Desired VPC Service Controls state service producer VPC network, as
        /// described at the top of this page.
        pub enabled: pulumi_gestalt_rust::Output<bool>,
        /// The network that the consumer is using to connect with services.
        pub network: pulumi_gestalt_rust::Output<String>,
        /// The id of the Google Cloud project containing the consumer network.
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        /// The service that is managing peering connectivity for a service
        /// producer's organization. For Google services that support this
        /// functionality, this value is `servicenetworking.googleapis.com`.
        ///
        ///
        /// - - -
        pub service: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: VpcServiceControlsArgs,
    ) -> VpcServiceControlsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let network_binding = args.network.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let service_binding = args.service.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:servicenetworking/vpcServiceControls:VpcServiceControls".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "network".into(),
                    value: &network_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "service".into(),
                    value: &service_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        VpcServiceControlsResult {
            enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            network: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("network"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            service: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("service"),
            ),
        }
    }
}
