/// An Anthos cluster running on Azure.
///
/// For more information, see:
/// * [Multicloud overview](https://cloud.google.com/kubernetes-engine/multi-cloud/docs)
/// ## Example Usage
///
/// ### Basic_azure_cluster
/// A basic example of a containerazure azure cluster
/// ```yaml
/// resources:
///   primary:
///     type: gcp:container:AzureCluster
///     properties:
///       authorization:
///         adminUsers:
///           - username: mmv2@google.com
///         adminGroups:
///           - group: group@domain.com
///       azureRegion: westus2
///       client: projects/my-project-number/locations/us-west1/azureClients/${basic.name}
///       controlPlane:
///         sshConfig:
///           authorizedKey: ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAACAQC8yaayO6lnb2v+SedxUMa2c8vtIEzCzBjM3EJJsv8Vm9zUDWR7dXWKoNGARUb2mNGXASvI6mFIDXTIlkQ0poDEPpMaXR0g2cb5xT8jAAJq7fqXL3+0rcJhY/uigQ+MrT6s+ub0BFVbsmGHNrMQttXX9gtmwkeAEvj3mra9e5pkNf90qlKnZz6U0SVArxVsLx07vHPHDIYrl0OPG4zUREF52igbBPiNrHJFDQJT/4YlDMJmo/QT/A1D6n9ocemvZSzhRx15/Arjowhr+VVKSbaxzPtEfY0oIg2SrqJnnr/l3Du5qIefwh5VmCZe4xopPUaDDoOIEFriZ88sB+3zz8ib8sk8zJJQCgeP78tQvXCgS+4e5W3TUg9mxjB6KjXTyHIVhDZqhqde0OI3Fy1UuVzRUwnBaLjBnAwP5EoFQGRmDYk/rEYe7HTmovLeEBUDQocBQKT4Ripm/xJkkWY7B07K/tfo56dGUCkvyIVXKBInCh+dLK7gZapnd4UWkY0xBYcwo1geMLRq58iFTLA2j/JmpmHXp7m0l7jJii7d44uD3tTIFYThn7NlOnvhLim/YcBK07GMGIN7XwrrKZKmxXaspw6KBWVhzuw1UPxctxshYEaMLfFg/bwOw8HvMPr9VtrElpSB7oiOh91PDIPdPBgHCi7N2QgQ5l/ZDBHieSpNrQ== thomasrodgers
///         subnetId: /subscriptions/12345678-1234-1234-1234-123456789111/resourceGroups/my--dev-byo/providers/Microsoft.Network/virtualNetworks/my--dev-vnet/subnets/default
///         version: ${versions.validVersions[0]}
///       fleet:
///         project: my-project-number
///       location: us-west1
///       name: name
///       networking:
///         podAddressCidrBlocks:
///           - 10.200.0.0/16
///         serviceAddressCidrBlocks:
///           - 10.32.0.0/24
///         virtualNetworkId: /subscriptions/12345678-1234-1234-1234-123456789111/resourceGroups/my--dev-byo/providers/Microsoft.Network/virtualNetworks/my--dev-vnet
///       resourceGroupId: /subscriptions/12345678-1234-1234-1234-123456789111/resourceGroups/my--dev-cluster
///       project: my-project-name
///   basic:
///     type: gcp:container:AzureClient
///     properties:
///       applicationId: 12345678-1234-1234-1234-123456789111
///       location: us-west1
///       name: client-name
///       tenantId: 12345678-1234-1234-1234-123456789111
///       project: my-project-name
/// variables:
///   versions:
///     fn::invoke:
///       function: gcp:container:getAzureVersions
///       arguments:
///         project: my-project-name
///         location: us-west1
/// ```
/// ### Beta_basic_enum_azure_cluster
/// A basic example of a containerazure azure cluster with lowercase enums (beta)
/// ```yaml
/// resources:
///   primary:
///     type: gcp:container:AzureCluster
///     properties:
///       authorization:
///         adminUsers:
///           - username: mmv2@google.com
///       azureRegion: westus2
///       client: projects/my-project-number/locations/us-west1/azureClients/${basic.name}
///       controlPlane:
///         sshConfig:
///           authorizedKey: ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAACAQC8yaayO6lnb2v+SedxUMa2c8vtIEzCzBjM3EJJsv8Vm9zUDWR7dXWKoNGARUb2mNGXASvI6mFIDXTIlkQ0poDEPpMaXR0g2cb5xT8jAAJq7fqXL3+0rcJhY/uigQ+MrT6s+ub0BFVbsmGHNrMQttXX9gtmwkeAEvj3mra9e5pkNf90qlKnZz6U0SVArxVsLx07vHPHDIYrl0OPG4zUREF52igbBPiNrHJFDQJT/4YlDMJmo/QT/A1D6n9ocemvZSzhRx15/Arjowhr+VVKSbaxzPtEfY0oIg2SrqJnnr/l3Du5qIefwh5VmCZe4xopPUaDDoOIEFriZ88sB+3zz8ib8sk8zJJQCgeP78tQvXCgS+4e5W3TUg9mxjB6KjXTyHIVhDZqhqde0OI3Fy1UuVzRUwnBaLjBnAwP5EoFQGRmDYk/rEYe7HTmovLeEBUDQocBQKT4Ripm/xJkkWY7B07K/tfo56dGUCkvyIVXKBInCh+dLK7gZapnd4UWkY0xBYcwo1geMLRq58iFTLA2j/JmpmHXp7m0l7jJii7d44uD3tTIFYThn7NlOnvhLim/YcBK07GMGIN7XwrrKZKmxXaspw6KBWVhzuw1UPxctxshYEaMLfFg/bwOw8HvMPr9VtrElpSB7oiOh91PDIPdPBgHCi7N2QgQ5l/ZDBHieSpNrQ== thomasrodgers
///         subnetId: /subscriptions/12345678-1234-1234-1234-123456789111/resourceGroups/my--dev-byo/providers/Microsoft.Network/virtualNetworks/my--dev-vnet/subnets/default
///         version: ${versions.validVersions[0]}
///       fleet:
///         project: my-project-number
///       location: us-west1
///       name: name
///       networking:
///         podAddressCidrBlocks:
///           - 10.200.0.0/16
///         serviceAddressCidrBlocks:
///           - 10.32.0.0/24
///         virtualNetworkId: /subscriptions/12345678-1234-1234-1234-123456789111/resourceGroups/my--dev-byo/providers/Microsoft.Network/virtualNetworks/my--dev-vnet
///       resourceGroupId: /subscriptions/12345678-1234-1234-1234-123456789111/resourceGroups/my--dev-cluster
///       project: my-project-name
///       loggingConfig:
///         componentConfig:
///           enableComponents:
///             - system_components
///             - workloads
///   basic:
///     type: gcp:container:AzureClient
///     properties:
///       applicationId: 12345678-1234-1234-1234-123456789111
///       location: us-west1
///       name: client-name
///       tenantId: 12345678-1234-1234-1234-123456789111
///       project: my-project-name
/// variables:
///   versions:
///     fn::invoke:
///       function: gcp:container:getAzureVersions
///       arguments:
///         project: my-project-name
///         location: us-west1
/// ```
///
/// ## Import
///
/// Cluster can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/azureClusters/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, Cluster can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:container/azureCluster:AzureCluster default projects/{{project}}/locations/{{location}}/azureClusters/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:container/azureCluster:AzureCluster default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:container/azureCluster:AzureCluster default {{location}}/{{name}}
/// ```
///
pub mod azure_cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AzureClusterArgs {
        /// Optional. Annotations on the cluster. This field has the same restrictions as Kubernetes annotations. The total size of
        /// all keys and values combined is limited to 256k. Keys can have 2 segments: prefix (optional) and name (required),
        /// separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with
        /// alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between. **Note**: This field is
        /// non-authoritative, and will only manage the annotations present in your configuration. Please refer to the field
        /// `effective_annotations` for all of the annotations present on the resource.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration related to the cluster RBAC settings.
        #[builder(into)]
        pub authorization: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::container::AzureClusterAuthorization,
        >,
        /// The Azure region where the cluster runs. Each Google Cloud region supports a subset of nearby Azure regions. You can call to list all supported Azure regions within a given Google Cloud region.
        #[builder(into)]
        pub azure_region: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Azure authentication configuration for management of Azure resources
        #[builder(into, default)]
        pub azure_services_authentication: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::container::AzureClusterAzureServicesAuthentication,
            >,
        >,
        /// Name of the AzureClient. The `AzureClient` resource must reside on the same GCP project and region as the
        /// `AzureCluster`. `AzureClient` names are formatted as
        /// `projects/<project-number>/locations/<region>/azureClients/<client-id>`. See Resource Names
        /// (https:cloud.google.com/apis/design/resource_names) for more details on Google Cloud resource names.
        #[builder(into, default)]
        pub client: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration related to the cluster control plane.
        #[builder(into)]
        pub control_plane: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::container::AzureClusterControlPlane,
        >,
        /// Optional. A human readable description of this cluster. Cannot be longer than 255 UTF-8 encoded bytes.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Fleet configuration.
        #[builder(into)]
        pub fleet: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::container::AzureClusterFleet,
        >,
        /// The location for the resource
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Logging configuration.
        #[builder(into, default)]
        pub logging_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::AzureClusterLoggingConfig>,
        >,
        /// The name of this resource.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Cluster-wide networking configuration.
        #[builder(into)]
        pub networking: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::container::AzureClusterNetworking,
        >,
        /// The project for the resource
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ARM ID of the resource group where the cluster resources are deployed. For example: `/subscriptions/*/resourceGroups/*`
        #[builder(into)]
        pub resource_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AzureClusterResult {
        /// Optional. Annotations on the cluster. This field has the same restrictions as Kubernetes annotations. The total size of
        /// all keys and values combined is limited to 256k. Keys can have 2 segments: prefix (optional) and name (required),
        /// separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with
        /// alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between. **Note**: This field is
        /// non-authoritative, and will only manage the annotations present in your configuration. Please refer to the field
        /// `effective_annotations` for all of the annotations present on the resource.
        pub annotations: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration related to the cluster RBAC settings.
        pub authorization: pulumi_gestalt_rust::Output<
            super::super::types::container::AzureClusterAuthorization,
        >,
        /// The Azure region where the cluster runs. Each Google Cloud region supports a subset of nearby Azure regions. You can call to list all supported Azure regions within a given Google Cloud region.
        pub azure_region: pulumi_gestalt_rust::Output<String>,
        /// Azure authentication configuration for management of Azure resources
        pub azure_services_authentication: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::container::AzureClusterAzureServicesAuthentication,
            >,
        >,
        /// Name of the AzureClient. The `AzureClient` resource must reside on the same GCP project and region as the
        /// `AzureCluster`. `AzureClient` names are formatted as
        /// `projects/<project-number>/locations/<region>/azureClients/<client-id>`. See Resource Names
        /// (https:cloud.google.com/apis/design/resource_names) for more details on Google Cloud resource names.
        pub client: pulumi_gestalt_rust::Output<Option<String>>,
        /// Configuration related to the cluster control plane.
        pub control_plane: pulumi_gestalt_rust::Output<
            super::super::types::container::AzureClusterControlPlane,
        >,
        /// Output only. The time at which this cluster was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Optional. A human readable description of this cluster. Cannot be longer than 255 UTF-8 encoded bytes.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        pub effective_annotations: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Output only. The endpoint of the cluster's API server.
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        /// Allows clients to perform consistent read-modify-writes through optimistic concurrency control. May be sent on update and delete requests to ensure the client has an up-to-date value before proceeding.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// Fleet configuration.
        pub fleet: pulumi_gestalt_rust::Output<
            super::super::types::container::AzureClusterFleet,
        >,
        /// The location for the resource
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Logging configuration.
        pub logging_config: pulumi_gestalt_rust::Output<
            super::super::types::container::AzureClusterLoggingConfig,
        >,
        /// The name of this resource.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Cluster-wide networking configuration.
        pub networking: pulumi_gestalt_rust::Output<
            super::super::types::container::AzureClusterNetworking,
        >,
        /// The project for the resource
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Output only. If set, there are currently changes in flight to the cluster.
        pub reconciling: pulumi_gestalt_rust::Output<bool>,
        /// The ARM ID of the resource group where the cluster resources are deployed. For example: `/subscriptions/*/resourceGroups/*`
        pub resource_group_id: pulumi_gestalt_rust::Output<String>,
        /// Output only. The current state of the cluster. Possible values: STATE_UNSPECIFIED, PROVISIONING, RUNNING, RECONCILING, STOPPING, ERROR, DEGRADED
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Output only. A globally unique identifier for the cluster.
        pub uid: pulumi_gestalt_rust::Output<String>,
        /// Output only. The time at which this cluster was last updated.
        pub update_time: pulumi_gestalt_rust::Output<String>,
        /// Output only. Workload Identity settings.
        pub workload_identity_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::types::container::AzureClusterWorkloadIdentityConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: AzureClusterArgs,
    ) -> AzureClusterResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let annotations_binding = args.annotations.get_output(context).get_inner();
        let authorization_binding = args.authorization.get_output(context).get_inner();
        let azure_region_binding = args.azure_region.get_output(context).get_inner();
        let azure_services_authentication_binding = args
            .azure_services_authentication
            .get_output(context)
            .get_inner();
        let client_binding = args.client.get_output(context).get_inner();
        let control_plane_binding = args.control_plane.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let fleet_binding = args.fleet.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let logging_config_binding = args.logging_config.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let networking_binding = args.networking.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let resource_group_id_binding = args
            .resource_group_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:container/azureCluster:AzureCluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding,
                },
                register_interface::ObjectField {
                    name: "authorization".into(),
                    value: &authorization_binding,
                },
                register_interface::ObjectField {
                    name: "azureRegion".into(),
                    value: &azure_region_binding,
                },
                register_interface::ObjectField {
                    name: "azureServicesAuthentication".into(),
                    value: &azure_services_authentication_binding,
                },
                register_interface::ObjectField {
                    name: "client".into(),
                    value: &client_binding,
                },
                register_interface::ObjectField {
                    name: "controlPlane".into(),
                    value: &control_plane_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "fleet".into(),
                    value: &fleet_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "loggingConfig".into(),
                    value: &logging_config_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "networking".into(),
                    value: &networking_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupId".into(),
                    value: &resource_group_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AzureClusterResult {
            annotations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("annotations"),
            ),
            authorization: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("authorization"),
            ),
            azure_region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("azureRegion"),
            ),
            azure_services_authentication: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("azureServicesAuthentication"),
            ),
            client: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("client"),
            ),
            control_plane: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("controlPlane"),
            ),
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            effective_annotations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveAnnotations"),
            ),
            endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endpoint"),
            ),
            etag: pulumi_gestalt_rust::__private::into_domain(o.extract_field("etag")),
            fleet: pulumi_gestalt_rust::__private::into_domain(o.extract_field("fleet")),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            logging_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("loggingConfig"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            networking: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networking"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            reconciling: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("reconciling"),
            ),
            resource_group_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupId"),
            ),
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
            uid: pulumi_gestalt_rust::__private::into_domain(o.extract_field("uid")),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
            workload_identity_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workloadIdentityConfigs"),
            ),
        }
    }
}
