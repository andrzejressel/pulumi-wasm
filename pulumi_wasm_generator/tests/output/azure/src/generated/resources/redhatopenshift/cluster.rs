/// Manages a fully managed Azure Red Hat OpenShift Cluster (also known as ARO).
///
/// > **Note:** All arguments including the client secret will be stored in the raw state as plain-text. [Read more about sensitive data in state](https://www.terraform.io/docs/state/sensitive-data.html).
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleApplication:
///     type: azuread:Application
///     name: example
///     properties:
///       displayName: example-aro
///   exampleServicePrincipal:
///     type: azuread:ServicePrincipal
///     name: example
///     properties:
///       clientId: ${exampleApplication.clientId}
///   exampleServicePrincipalPassword:
///     type: azuread:ServicePrincipalPassword
///     name: example
///     properties:
///       servicePrincipalId: ${exampleServicePrincipal.objectId}
///   roleNetwork1:
///     type: azure:authorization:Assignment
///     name: role_network1
///     properties:
///       scope: ${exampleVirtualNetwork.id}
///       roleDefinitionName: Network Contributor
///       principalId: ${exampleServicePrincipal.objectId}
///   roleNetwork2:
///     type: azure:authorization:Assignment
///     name: role_network2
///     properties:
///       scope: ${exampleVirtualNetwork.id}
///       roleDefinitionName: Network Contributor
///       principalId: ${redhatopenshift.objectId}
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: example-resources
///       location: West US
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: example-vnet
///       addressSpaces:
///         - 10.0.0.0/22
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///   mainSubnet:
///     type: azure:network:Subnet
///     name: main_subnet
///     properties:
///       name: main-subnet
///       resourceGroupName: ${exampleResourceGroup.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.0.0/23
///       serviceEndpoints:
///         - Microsoft.Storage
///         - Microsoft.ContainerRegistry
///   workerSubnet:
///     type: azure:network:Subnet
///     name: worker_subnet
///     properties:
///       name: worker-subnet
///       resourceGroupName: ${exampleResourceGroup.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.2.0/23
///       serviceEndpoints:
///         - Microsoft.Storage
///         - Microsoft.ContainerRegistry
///   exampleCluster:
///     type: azure:redhatopenshift:Cluster
///     name: example
///     properties:
///       name: examplearo
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       clusterProfile:
///         domain: aro-example.com
///         version: 4.13.23
///       networkProfile:
///         podCidr: 10.128.0.0/14
///         serviceCidr: 172.30.0.0/16
///       mainProfile:
///         vmSize: Standard_D8s_v3
///         subnetId: ${mainSubnet.id}
///       apiServerProfile:
///         visibility: Public
///       ingressProfile:
///         visibility: Public
///       workerProfile:
///         vmSize: Standard_D4s_v3
///         diskSizeGb: 128
///         nodeCount: 3
///         subnetId: ${workerSubnet.id}
///       servicePrincipal:
///         clientId: ${exampleApplication.clientId}
///         clientSecret: ${exampleServicePrincipalPassword.value}
///     options:
///       dependsOn:
///         - ${roleNetwork1}
///         - ${roleNetwork2}
/// variables:
///   example:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
///   exampleGetClientConfig:
///     fn::invoke:
///       function: azuread:getClientConfig
///       arguments: {}
///   redhatopenshift:
///     fn::invoke:
///       function: azuread:getServicePrincipal
///       arguments:
///         clientId: f1dd0a37-89c6-4e07-bcd1-ffd3d43d8875
/// outputs:
///   consoleUrl: ${exampleCluster.consoleUrl}
/// ```
///
/// ## Import
///
/// Red Hat OpenShift Clusters can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:redhatopenshift/cluster:Cluster cluster1 /subscriptions/00000000-0000-0000-0000-000000000000/resourcegroups/group1/providers/Microsoft.RedHatOpenShift/openShiftClusters/cluster1
/// ```
///
pub mod cluster {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterArgs {
        /// An `api_server_profile` block as defined below. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_server_profile: pulumi_wasm_rust::Output<
            super::super::types::redhatopenshift::ClusterApiServerProfile,
        >,
        /// A `cluster_profile` block as defined below. Changing this forces a new resource to be created.
        #[builder(into)]
        pub cluster_profile: pulumi_wasm_rust::Output<
            super::super::types::redhatopenshift::ClusterClusterProfile,
        >,
        /// An `ingress_profile` block as defined below. Changing this forces a new resource to be created.
        #[builder(into)]
        pub ingress_profile: pulumi_wasm_rust::Output<
            super::super::types::redhatopenshift::ClusterIngressProfile,
        >,
        /// The location where the Azure Red Hat OpenShift Cluster should be created. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// A `main_profile` block as defined below. Changing this forces a new resource to be created.
        #[builder(into)]
        pub main_profile: pulumi_wasm_rust::Output<
            super::super::types::redhatopenshift::ClusterMainProfile,
        >,
        /// The name of the Azure Red Hat OpenShift Cluster to create. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A `network_profile` block as defined below. Changing this forces a new resource to be created.
        #[builder(into)]
        pub network_profile: pulumi_wasm_rust::Output<
            super::super::types::redhatopenshift::ClusterNetworkProfile,
        >,
        /// Specifies the Resource Group where the Azure Red Hat OpenShift Cluster should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `service_principal` block as defined below.
        #[builder(into)]
        pub service_principal: pulumi_wasm_rust::Output<
            super::super::types::redhatopenshift::ClusterServicePrincipal,
        >,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `worker_profile` block as defined below. Changing this forces a new resource to be created.
        #[builder(into)]
        pub worker_profile: pulumi_wasm_rust::Output<
            super::super::types::redhatopenshift::ClusterWorkerProfile,
        >,
    }
    #[allow(dead_code)]
    pub struct ClusterResult {
        /// An `api_server_profile` block as defined below. Changing this forces a new resource to be created.
        pub api_server_profile: pulumi_wasm_rust::Output<
            super::super::types::redhatopenshift::ClusterApiServerProfile,
        >,
        /// A `cluster_profile` block as defined below. Changing this forces a new resource to be created.
        pub cluster_profile: pulumi_wasm_rust::Output<
            super::super::types::redhatopenshift::ClusterClusterProfile,
        >,
        /// The Red Hat OpenShift cluster console URL.
        pub console_url: pulumi_wasm_rust::Output<String>,
        /// An `ingress_profile` block as defined below. Changing this forces a new resource to be created.
        pub ingress_profile: pulumi_wasm_rust::Output<
            super::super::types::redhatopenshift::ClusterIngressProfile,
        >,
        /// The location where the Azure Red Hat OpenShift Cluster should be created. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// A `main_profile` block as defined below. Changing this forces a new resource to be created.
        pub main_profile: pulumi_wasm_rust::Output<
            super::super::types::redhatopenshift::ClusterMainProfile,
        >,
        /// The name of the Azure Red Hat OpenShift Cluster to create. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `network_profile` block as defined below. Changing this forces a new resource to be created.
        pub network_profile: pulumi_wasm_rust::Output<
            super::super::types::redhatopenshift::ClusterNetworkProfile,
        >,
        /// Specifies the Resource Group where the Azure Red Hat OpenShift Cluster should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `service_principal` block as defined below.
        pub service_principal: pulumi_wasm_rust::Output<
            super::super::types::redhatopenshift::ClusterServicePrincipal,
        >,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `worker_profile` block as defined below. Changing this forces a new resource to be created.
        pub worker_profile: pulumi_wasm_rust::Output<
            super::super::types::redhatopenshift::ClusterWorkerProfile,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ClusterArgs) -> ClusterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_server_profile_binding = args.api_server_profile.get_inner();
        let cluster_profile_binding = args.cluster_profile.get_inner();
        let ingress_profile_binding = args.ingress_profile.get_inner();
        let location_binding = args.location.get_inner();
        let main_profile_binding = args.main_profile.get_inner();
        let name_binding = args.name.get_inner();
        let network_profile_binding = args.network_profile.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let service_principal_binding = args.service_principal.get_inner();
        let tags_binding = args.tags.get_inner();
        let worker_profile_binding = args.worker_profile.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:redhatopenshift/cluster:Cluster".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiServerProfile".into(),
                    value: &api_server_profile_binding,
                },
                register_interface::ObjectField {
                    name: "clusterProfile".into(),
                    value: &cluster_profile_binding,
                },
                register_interface::ObjectField {
                    name: "ingressProfile".into(),
                    value: &ingress_profile_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "mainProfile".into(),
                    value: &main_profile_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "networkProfile".into(),
                    value: &network_profile_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "servicePrincipal".into(),
                    value: &service_principal_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "workerProfile".into(),
                    value: &worker_profile_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiServerProfile".into(),
                },
                register_interface::ResultField {
                    name: "clusterProfile".into(),
                },
                register_interface::ResultField {
                    name: "consoleUrl".into(),
                },
                register_interface::ResultField {
                    name: "ingressProfile".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "mainProfile".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networkProfile".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "servicePrincipal".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "workerProfile".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ClusterResult {
            api_server_profile: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiServerProfile").unwrap(),
            ),
            cluster_profile: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterProfile").unwrap(),
            ),
            console_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("consoleUrl").unwrap(),
            ),
            ingress_profile: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ingressProfile").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            main_profile: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mainProfile").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network_profile: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkProfile").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            service_principal: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("servicePrincipal").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            worker_profile: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workerProfile").unwrap(),
            ),
        }
    }
}