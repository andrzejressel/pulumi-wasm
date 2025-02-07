/// Manages a Container App Environment.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleAnalyticsWorkspace = analytics_workspace::create(
///         "exampleAnalyticsWorkspace",
///         AnalyticsWorkspaceArgs::builder()
///             .location("${example.location}")
///             .name("acctest-01")
///             .resource_group_name("${example.name}")
///             .retention_in_days(30)
///             .sku("PerGB2018")
///             .build_struct(),
///     );
///     let exampleEnvironment = environment::create(
///         "exampleEnvironment",
///         EnvironmentArgs::builder()
///             .location("${example.location}")
///             .log_analytics_workspace_id("${exampleAnalyticsWorkspace.id}")
///             .name("my-environment")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// A Container App Environment can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:containerapp/environment:Environment example "/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resGroup1/providers/Microsoft.App/managedEnvironments/myEnvironment"
/// ```
///
pub mod environment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnvironmentArgs {
        /// Application Insights connection string used by Dapr to export Service to Service communication telemetry. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub dapr_application_insights_connection_string: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Name of the platform-managed resource group created for the Managed Environment to host infrastructure resources. Changing this forces a new resource to be created.
        ///
        /// > **Note:** Only valid if a `workload_profile` is specified. If `infrastructure_subnet_id` is specified, this resource group will be created in the same subscription as `infrastructure_subnet_id`.
        #[builder(into, default)]
        pub infrastructure_resource_group_name: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The existing Subnet to use for the Container Apps Control Plane. Changing this forces a new resource to be created.
        ///
        /// > **Note:** The Subnet must have a `/21` or larger address space.
        #[builder(into, default)]
        pub infrastructure_subnet_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Should the Container Environment operate in Internal Load Balancing Mode? Defaults to `false`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** can only be set to `true` if `infrastructure_subnet_id` is specified.
        #[builder(into, default)]
        pub internal_load_balancer_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Specifies the supported Azure location where the Container App Environment is to exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID for the Log Analytics Workspace to link this Container Apps Managed Environment to.
        #[builder(into, default)]
        pub log_analytics_workspace_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Should mutual transport layer security (mTLS) be enabled? Defaults to `false`.
        ///
        /// > **Note:** This feature is in public preview. Enabling mTLS for your applications may increase response latency and reduce maximum throughput in high-load scenarios.
        #[builder(into, default)]
        pub mutual_tls_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the Container Apps Managed Environment. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which the Container App Environment is to be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The profile of the workload to scope the container app execution. A `workload_profile` block as defined below.
        #[builder(into, default)]
        pub workload_profiles: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::containerapp::EnvironmentWorkloadProfile>>,
        >,
        /// Should the Container App Environment be created with Zone Redundancy enabled? Defaults to `false`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** can only be set to `true` if `infrastructure_subnet_id` is specified.
        #[builder(into, default)]
        pub zone_redundancy_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct EnvironmentResult {
        /// The ID of the Custom Domain Verification for this Container App Environment.
        pub custom_domain_verification_id: pulumi_gestalt_rust::Output<String>,
        /// Application Insights connection string used by Dapr to export Service to Service communication telemetry. Changing this forces a new resource to be created.
        pub dapr_application_insights_connection_string: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// The default, publicly resolvable, name of this Container App Environment.
        pub default_domain: pulumi_gestalt_rust::Output<String>,
        /// The network addressing in which the Container Apps in this Container App Environment will reside in CIDR notation.
        pub docker_bridge_cidr: pulumi_gestalt_rust::Output<String>,
        /// Name of the platform-managed resource group created for the Managed Environment to host infrastructure resources. Changing this forces a new resource to be created.
        ///
        /// > **Note:** Only valid if a `workload_profile` is specified. If `infrastructure_subnet_id` is specified, this resource group will be created in the same subscription as `infrastructure_subnet_id`.
        pub infrastructure_resource_group_name: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// The existing Subnet to use for the Container Apps Control Plane. Changing this forces a new resource to be created.
        ///
        /// > **Note:** The Subnet must have a `/21` or larger address space.
        pub infrastructure_subnet_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Should the Container Environment operate in Internal Load Balancing Mode? Defaults to `false`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** can only be set to `true` if `infrastructure_subnet_id` is specified.
        pub internal_load_balancer_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the Container App Environment is to exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The ID for the Log Analytics Workspace to link this Container Apps Managed Environment to.
        pub log_analytics_workspace_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Should mutual transport layer security (mTLS) be enabled? Defaults to `false`.
        ///
        /// > **Note:** This feature is in public preview. Enabling mTLS for your applications may increase response latency and reduce maximum throughput in high-load scenarios.
        pub mutual_tls_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the Container Apps Managed Environment. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The IP range, in CIDR notation, that is reserved for environment infrastructure IP addresses.
        pub platform_reserved_cidr: pulumi_gestalt_rust::Output<String>,
        /// The IP address from the IP range defined by `platform_reserved_cidr` that is reserved for the internal DNS server.
        pub platform_reserved_dns_ip_address: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which the Container App Environment is to be created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The Static IP address of the Environment.
        pub static_ip_address: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The profile of the workload to scope the container app execution. A `workload_profile` block as defined below.
        pub workload_profiles: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::containerapp::EnvironmentWorkloadProfile>>,
        >,
        /// Should the Container App Environment be created with Zone Redundancy enabled? Defaults to `false`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** can only be set to `true` if `infrastructure_subnet_id` is specified.
        pub zone_redundancy_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: EnvironmentArgs,
    ) -> EnvironmentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let dapr_application_insights_connection_string_binding = args
            .dapr_application_insights_connection_string
            .get_output(context)
            .get_inner();
        let infrastructure_resource_group_name_binding = args
            .infrastructure_resource_group_name
            .get_output(context)
            .get_inner();
        let infrastructure_subnet_id_binding = args
            .infrastructure_subnet_id
            .get_output(context)
            .get_inner();
        let internal_load_balancer_enabled_binding = args
            .internal_load_balancer_enabled
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let log_analytics_workspace_id_binding = args
            .log_analytics_workspace_id
            .get_output(context)
            .get_inner();
        let mutual_tls_enabled_binding = args
            .mutual_tls_enabled
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let workload_profiles_binding = args
            .workload_profiles
            .get_output(context)
            .get_inner();
        let zone_redundancy_enabled_binding = args
            .zone_redundancy_enabled
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:containerapp/environment:Environment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "daprApplicationInsightsConnectionString".into(),
                    value: &dapr_application_insights_connection_string_binding,
                },
                register_interface::ObjectField {
                    name: "infrastructureResourceGroupName".into(),
                    value: &infrastructure_resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "infrastructureSubnetId".into(),
                    value: &infrastructure_subnet_id_binding,
                },
                register_interface::ObjectField {
                    name: "internalLoadBalancerEnabled".into(),
                    value: &internal_load_balancer_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "logAnalyticsWorkspaceId".into(),
                    value: &log_analytics_workspace_id_binding,
                },
                register_interface::ObjectField {
                    name: "mutualTlsEnabled".into(),
                    value: &mutual_tls_enabled_binding,
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
                register_interface::ObjectField {
                    name: "workloadProfiles".into(),
                    value: &workload_profiles_binding,
                },
                register_interface::ObjectField {
                    name: "zoneRedundancyEnabled".into(),
                    value: &zone_redundancy_enabled_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        EnvironmentResult {
            custom_domain_verification_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customDomainVerificationId"),
            ),
            dapr_application_insights_connection_string: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("daprApplicationInsightsConnectionString"),
            ),
            default_domain: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultDomain"),
            ),
            docker_bridge_cidr: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dockerBridgeCidr"),
            ),
            infrastructure_resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("infrastructureResourceGroupName"),
            ),
            infrastructure_subnet_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("infrastructureSubnetId"),
            ),
            internal_load_balancer_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("internalLoadBalancerEnabled"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            log_analytics_workspace_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("logAnalyticsWorkspaceId"),
            ),
            mutual_tls_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("mutualTlsEnabled"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            platform_reserved_cidr: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("platformReservedCidr"),
            ),
            platform_reserved_dns_ip_address: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("platformReservedDnsIpAddress"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            static_ip_address: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("staticIpAddress"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            workload_profiles: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workloadProfiles"),
            ),
            zone_redundancy_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("zoneRedundancyEnabled"),
            ),
        }
    }
}
