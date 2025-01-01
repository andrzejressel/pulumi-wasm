/// An Anthos node pool running on Azure.
///
/// For more information, see:
/// * [Multicloud overview](https://cloud.google.com/kubernetes-engine/multi-cloud/docs)
/// ## Example Usage
///
/// ### Basic_azure_node_pool
/// A basic example of a containerazure azure node pool
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
///   basic:
///     type: gcp:container:AzureClient
///     properties:
///       applicationId: 12345678-1234-1234-1234-123456789111
///       location: us-west1
///       name: client-name
///       tenantId: 12345678-1234-1234-1234-123456789111
///       project: my-project-name
///   primaryAzureNodePool:
///     type: gcp:container:AzureNodePool
///     name: primary
///     properties:
///       autoscaling:
///         maxNodeCount: 3
///         minNodeCount: 2
///       cluster: ${primary.name}
///       config:
///         sshConfig:
///           authorizedKey: ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAACAQC8yaayO6lnb2v+SedxUMa2c8vtIEzCzBjM3EJJsv8Vm9zUDWR7dXWKoNGARUb2mNGXASvI6mFIDXTIlkQ0poDEPpMaXR0g2cb5xT8jAAJq7fqXL3+0rcJhY/uigQ+MrT6s+ub0BFVbsmGHNrMQttXX9gtmwkeAEvj3mra9e5pkNf90qlKnZz6U0SVArxVsLx07vHPHDIYrl0OPG4zUREF52igbBPiNrHJFDQJT/4YlDMJmo/QT/A1D6n9ocemvZSzhRx15/Arjowhr+VVKSbaxzPtEfY0oIg2SrqJnnr/l3Du5qIefwh5VmCZe4xopPUaDDoOIEFriZ88sB+3zz8ib8sk8zJJQCgeP78tQvXCgS+4e5W3TUg9mxjB6KjXTyHIVhDZqhqde0OI3Fy1UuVzRUwnBaLjBnAwP5EoFQGRmDYk/rEYe7HTmovLeEBUDQocBQKT4Ripm/xJkkWY7B07K/tfo56dGUCkvyIVXKBInCh+dLK7gZapnd4UWkY0xBYcwo1geMLRq58iFTLA2j/JmpmHXp7m0l7jJii7d44uD3tTIFYThn7NlOnvhLim/YcBK07GMGIN7XwrrKZKmxXaspw6KBWVhzuw1UPxctxshYEaMLfFg/bwOw8HvMPr9VtrElpSB7oiOh91PDIPdPBgHCi7N2QgQ5l/ZDBHieSpNrQ== thomasrodgers
///         proxyConfig:
///           resourceGroupId: /subscriptions/12345678-1234-1234-1234-123456789111/resourceGroups/my--dev-cluster
///           secretId: https://my--dev-keyvault.vault.azure.net/secrets/my--dev-secret/0000000000000000000000000000000000
///         rootVolume:
///           sizeGib: 32
///         tags:
///           owner: mmv2
///         labels:
///           key_one: label_one
///         vmSize: Standard_DS2_v2
///       location: us-west1
///       maxPodsConstraint:
///         maxPodsPerNode: 110
///       name: node-pool-name
///       subnetId: /subscriptions/12345678-1234-1234-1234-123456789111/resourceGroups/my--dev-byo/providers/Microsoft.Network/virtualNetworks/my--dev-vnet/subnets/default
///       version: ${versions.validVersions[0]}
///       annotations:
///         annotation-one: value-one
///       management:
///         autoRepair: true
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
/// NodePool can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/azureClusters/{{cluster}}/azureNodePools/{{name}}`
///
/// * `{{project}}/{{location}}/{{cluster}}/{{name}}`
///
/// * `{{location}}/{{cluster}}/{{name}}`
///
/// When using the `pulumi import` command, NodePool can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:container/azureNodePool:AzureNodePool default projects/{{project}}/locations/{{location}}/azureClusters/{{cluster}}/azureNodePools/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:container/azureNodePool:AzureNodePool default {{project}}/{{location}}/{{cluster}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:container/azureNodePool:AzureNodePool default {{location}}/{{cluster}}/{{name}}
/// ```
///
pub mod azure_node_pool {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AzureNodePoolArgs {
        /// Optional. Annotations on the node pool. This field has the same restrictions as Kubernetes annotations. The total size
        /// of all keys and values combined is limited to 256k. Keys can have 2 segments: prefix (optional) and name (required),
        /// separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with
        /// alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between. **Note**: This field is
        /// non-authoritative, and will only manage the annotations present in your configuration. Please refer to the field
        /// `effective_annotations` for all of the annotations present on the resource.
        #[builder(into, default)]
        pub annotations: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Autoscaler configuration for this node pool.
        #[builder(into)]
        pub autoscaling: pulumi_wasm_rust::Output<
            super::super::types::container::AzureNodePoolAutoscaling,
        >,
        /// Optional. The Azure availability zone of the nodes in this nodepool. When unspecified, it defaults to `1`.
        #[builder(into, default)]
        pub azure_availability_zone: pulumi_wasm_rust::Output<Option<String>>,
        /// The azureCluster for the resource
        #[builder(into)]
        pub cluster: pulumi_wasm_rust::Output<String>,
        /// The node configuration of the node pool.
        #[builder(into)]
        pub config: pulumi_wasm_rust::Output<
            super::super::types::container::AzureNodePoolConfig,
        >,
        /// The location for the resource
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// The Management configuration for this node pool.
        #[builder(into, default)]
        pub management: pulumi_wasm_rust::Output<
            Option<super::super::types::container::AzureNodePoolManagement>,
        >,
        /// The constraint on the maximum number of pods that can be run simultaneously on a node in the node pool.
        #[builder(into)]
        pub max_pods_constraint: pulumi_wasm_rust::Output<
            super::super::types::container::AzureNodePoolMaxPodsConstraint,
        >,
        /// The name of this resource.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The project for the resource
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The ARM ID of the subnet where the node pool VMs run. Make sure it's a subnet under the virtual network in the cluster configuration.
        #[builder(into)]
        pub subnet_id: pulumi_wasm_rust::Output<String>,
        /// The Kubernetes version (e.g. `1.19.10-gke.1000`) running on this node pool.
        #[builder(into)]
        pub version: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct AzureNodePoolResult {
        /// Optional. Annotations on the node pool. This field has the same restrictions as Kubernetes annotations. The total size
        /// of all keys and values combined is limited to 256k. Keys can have 2 segments: prefix (optional) and name (required),
        /// separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with
        /// alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between. **Note**: This field is
        /// non-authoritative, and will only manage the annotations present in your configuration. Please refer to the field
        /// `effective_annotations` for all of the annotations present on the resource.
        pub annotations: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Autoscaler configuration for this node pool.
        pub autoscaling: pulumi_wasm_rust::Output<
            super::super::types::container::AzureNodePoolAutoscaling,
        >,
        /// Optional. The Azure availability zone of the nodes in this nodepool. When unspecified, it defaults to `1`.
        pub azure_availability_zone: pulumi_wasm_rust::Output<String>,
        /// The azureCluster for the resource
        pub cluster: pulumi_wasm_rust::Output<String>,
        /// The node configuration of the node pool.
        pub config: pulumi_wasm_rust::Output<
            super::super::types::container::AzureNodePoolConfig,
        >,
        /// Output only. The time at which this node pool was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        pub effective_annotations: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Allows clients to perform consistent read-modify-writes through optimistic concurrency control. May be sent on update and delete requests to ensure the client has an up-to-date value before proceeding.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// The location for the resource
        pub location: pulumi_wasm_rust::Output<String>,
        /// The Management configuration for this node pool.
        pub management: pulumi_wasm_rust::Output<
            super::super::types::container::AzureNodePoolManagement,
        >,
        /// The constraint on the maximum number of pods that can be run simultaneously on a node in the node pool.
        pub max_pods_constraint: pulumi_wasm_rust::Output<
            super::super::types::container::AzureNodePoolMaxPodsConstraint,
        >,
        /// The name of this resource.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The project for the resource
        pub project: pulumi_wasm_rust::Output<String>,
        /// Output only. If set, there are currently pending changes to the node pool.
        pub reconciling: pulumi_wasm_rust::Output<bool>,
        /// Output only. The current state of the node pool. Possible values: STATE_UNSPECIFIED, PROVISIONING, RUNNING, RECONCILING, STOPPING, ERROR, DEGRADED
        pub state: pulumi_wasm_rust::Output<String>,
        /// The ARM ID of the subnet where the node pool VMs run. Make sure it's a subnet under the virtual network in the cluster configuration.
        pub subnet_id: pulumi_wasm_rust::Output<String>,
        /// Output only. A globally unique identifier for the node pool.
        pub uid: pulumi_wasm_rust::Output<String>,
        /// Output only. The time at which this node pool was last updated.
        pub update_time: pulumi_wasm_rust::Output<String>,
        /// The Kubernetes version (e.g. `1.19.10-gke.1000`) running on this node pool.
        pub version: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AzureNodePoolArgs) -> AzureNodePoolResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let annotations_binding = args.annotations.get_inner();
        let autoscaling_binding = args.autoscaling.get_inner();
        let azure_availability_zone_binding = args.azure_availability_zone.get_inner();
        let cluster_binding = args.cluster.get_inner();
        let config_binding = args.config.get_inner();
        let location_binding = args.location.get_inner();
        let management_binding = args.management.get_inner();
        let max_pods_constraint_binding = args.max_pods_constraint.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let subnet_id_binding = args.subnet_id.get_inner();
        let version_binding = args.version.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:container/azureNodePool:AzureNodePool".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding,
                },
                register_interface::ObjectField {
                    name: "autoscaling".into(),
                    value: &autoscaling_binding,
                },
                register_interface::ObjectField {
                    name: "azureAvailabilityZone".into(),
                    value: &azure_availability_zone_binding,
                },
                register_interface::ObjectField {
                    name: "cluster".into(),
                    value: &cluster_binding,
                },
                register_interface::ObjectField {
                    name: "config".into(),
                    value: &config_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "management".into(),
                    value: &management_binding,
                },
                register_interface::ObjectField {
                    name: "maxPodsConstraint".into(),
                    value: &max_pods_constraint_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding,
                },
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "annotations".into(),
                },
                register_interface::ResultField {
                    name: "autoscaling".into(),
                },
                register_interface::ResultField {
                    name: "azureAvailabilityZone".into(),
                },
                register_interface::ResultField {
                    name: "cluster".into(),
                },
                register_interface::ResultField {
                    name: "config".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "effectiveAnnotations".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "management".into(),
                },
                register_interface::ResultField {
                    name: "maxPodsConstraint".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "reconciling".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "subnetId".into(),
                },
                register_interface::ResultField {
                    name: "uid".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AzureNodePoolResult {
            annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("annotations").unwrap(),
            ),
            autoscaling: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoscaling").unwrap(),
            ),
            azure_availability_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("azureAvailabilityZone").unwrap(),
            ),
            cluster: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cluster").unwrap(),
            ),
            config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("config").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            effective_annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveAnnotations").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            management: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("management").unwrap(),
            ),
            max_pods_constraint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxPodsConstraint").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            reconciling: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reconciling").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            subnet_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetId").unwrap(),
            ),
            uid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uid").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}
