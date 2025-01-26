/// Manages an Azure VMware Solution Private Cloud.
///
/// ## Example Usage
///
/// > **NOTE :**  Normal `pulumi up` could ignore this note. Please disable correlation request id for continuous operations in one build (like acctest). The continuous operations like `update` or `delete` could not be triggered when it shares the same `correlation-id` with its previous operation.
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
///     let examplePrivateCloud = private_cloud::create(
///         "examplePrivateCloud",
///         PrivateCloudArgs::builder()
///             .internet_connection_enabled(false)
///             .location("${example.location}")
///             .management_cluster(
///                 PrivateCloudManagementCluster::builder().size(3).build_struct(),
///             )
///             .name("example-vmware-private-cloud")
///             .network_subnet_cidr("192.168.48.0/22")
///             .nsxt_password("QazWsx13$Edc")
///             .resource_group_name("${example.name}")
///             .sku_name("av36")
///             .vcenter_password("WsxEdc23$Rfv")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Azure VMware Solution Private Clouds can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:avs/privateCloud:PrivateCloud example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.AVS/privateClouds/privateCloud1
/// ```
///
pub mod private_cloud {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PrivateCloudArgs {
        /// Is the Azure VMware Solution Private Cloud connected to the internet? This field can not be updated with `management_cluster[0].size` together.
        /// > **NOTE :** `internet_connection_enabled` and `management_cluster[0].size` cannot be updated at the same time.
        #[builder(into, default)]
        pub internet_connection_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The Azure Region where the Azure VMware Solution Private Cloud should exist. Changing this forces a new Azure VMware Solution Private Cloud to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A `management_cluster` block as defined below.
        /// > **NOTE :** `internet_connection_enabled` and `management_cluster[0].size` cannot be updated at the same time.
        #[builder(into)]
        pub management_cluster: pulumi_wasm_rust::InputOrOutput<
            super::super::types::avs::PrivateCloudManagementCluster,
        >,
        /// The name which should be used for this Azure VMware Solution Private Cloud. Changing this forces a new Azure VMware Solution Private Cloud to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The subnet which should be unique across virtual network in your subscription as well as on-premise. Changing this forces a new Azure VMware Solution Private Cloud to be created.
        #[builder(into)]
        pub network_subnet_cidr: pulumi_wasm_rust::InputOrOutput<String>,
        /// The password of the VMware NSX Manager cloudadmin. Changing this forces a new Azure VMware Solution Private Cloud to be created.
        #[builder(into, default)]
        pub nsxt_password: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Azure VMware Solution Private Cloud should exist. Changing this forces a new Azure VMware Solution Private Cloud to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Name of the SKU used for this Azure VMware Solution Private Cloud. Possible values are `av20`, `av36`, `av36t`, `av36p`, `av36pt`, `av52`, `av52t`, and `av64`. Changing this forces a new Azure VMware Solution Private Cloud to be created.
        #[builder(into)]
        pub sku_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Azure VMware Solution Private Cloud.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The password of the VMware vCenter Server cloudadmin. Changing this forces a new Azure VMware Solution Private Cloud to be created.
        #[builder(into, default)]
        pub vcenter_password: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct PrivateCloudResult {
        /// A `circuit` block as defined below.
        pub circuits: pulumi_wasm_rust::Output<
            Vec<super::super::types::avs::PrivateCloudCircuit>,
        >,
        /// The endpoint for the VMware HCX Cloud Manager.
        pub hcx_cloud_manager_endpoint: pulumi_wasm_rust::Output<String>,
        /// Is the Azure VMware Solution Private Cloud connected to the internet? This field can not be updated with `management_cluster[0].size` together.
        /// > **NOTE :** `internet_connection_enabled` and `management_cluster[0].size` cannot be updated at the same time.
        pub internet_connection_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Azure Region where the Azure VMware Solution Private Cloud should exist. Changing this forces a new Azure VMware Solution Private Cloud to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// A `management_cluster` block as defined below.
        /// > **NOTE :** `internet_connection_enabled` and `management_cluster[0].size` cannot be updated at the same time.
        pub management_cluster: pulumi_wasm_rust::Output<
            super::super::types::avs::PrivateCloudManagementCluster,
        >,
        /// The network used to access VMware vCenter Server and NSX Manager.
        pub management_subnet_cidr: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Azure VMware Solution Private Cloud. Changing this forces a new Azure VMware Solution Private Cloud to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The subnet which should be unique across virtual network in your subscription as well as on-premise. Changing this forces a new Azure VMware Solution Private Cloud to be created.
        pub network_subnet_cidr: pulumi_wasm_rust::Output<String>,
        /// The thumbprint of the VMware NSX Manager SSL certificate.
        pub nsxt_certificate_thumbprint: pulumi_wasm_rust::Output<String>,
        /// The endpoint for the VMware NSX Manager.
        pub nsxt_manager_endpoint: pulumi_wasm_rust::Output<String>,
        /// The password of the VMware NSX Manager cloudadmin. Changing this forces a new Azure VMware Solution Private Cloud to be created.
        pub nsxt_password: pulumi_wasm_rust::Output<Option<String>>,
        /// The network which is used for virtual machine cold migration, cloning, and snapshot migration.
        pub provisioning_subnet_cidr: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Azure VMware Solution Private Cloud should exist. Changing this forces a new Azure VMware Solution Private Cloud to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The Name of the SKU used for this Azure VMware Solution Private Cloud. Possible values are `av20`, `av36`, `av36t`, `av36p`, `av36pt`, `av52`, `av52t`, and `av64`. Changing this forces a new Azure VMware Solution Private Cloud to be created.
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Azure VMware Solution Private Cloud.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The thumbprint of the VMware vCenter Server SSL certificate.
        pub vcenter_certificate_thumbprint: pulumi_wasm_rust::Output<String>,
        /// The password of the VMware vCenter Server cloudadmin. Changing this forces a new Azure VMware Solution Private Cloud to be created.
        pub vcenter_password: pulumi_wasm_rust::Output<Option<String>>,
        /// The endpoint for VMware vCenter Server Appliance.
        pub vcsa_endpoint: pulumi_wasm_rust::Output<String>,
        /// The network which is used for live migration of virtual machines.
        pub vmotion_subnet_cidr: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: PrivateCloudArgs,
    ) -> PrivateCloudResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let internet_connection_enabled_binding = args
            .internet_connection_enabled
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let management_cluster_binding = args
            .management_cluster
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let network_subnet_cidr_binding = args
            .network_subnet_cidr
            .get_output(context)
            .get_inner();
        let nsxt_password_binding = args.nsxt_password.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let sku_name_binding = args.sku_name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let vcenter_password_binding = args
            .vcenter_password
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:avs/privateCloud:PrivateCloud".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "internetConnectionEnabled".into(),
                    value: &internet_connection_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "managementCluster".into(),
                    value: &management_cluster_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "networkSubnetCidr".into(),
                    value: &network_subnet_cidr_binding,
                },
                register_interface::ObjectField {
                    name: "nsxtPassword".into(),
                    value: &nsxt_password_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "skuName".into(),
                    value: &sku_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "vcenterPassword".into(),
                    value: &vcenter_password_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "circuits".into(),
                },
                register_interface::ResultField {
                    name: "hcxCloudManagerEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "internetConnectionEnabled".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "managementCluster".into(),
                },
                register_interface::ResultField {
                    name: "managementSubnetCidr".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networkSubnetCidr".into(),
                },
                register_interface::ResultField {
                    name: "nsxtCertificateThumbprint".into(),
                },
                register_interface::ResultField {
                    name: "nsxtManagerEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "nsxtPassword".into(),
                },
                register_interface::ResultField {
                    name: "provisioningSubnetCidr".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "skuName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "vcenterCertificateThumbprint".into(),
                },
                register_interface::ResultField {
                    name: "vcenterPassword".into(),
                },
                register_interface::ResultField {
                    name: "vcsaEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "vmotionSubnetCidr".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PrivateCloudResult {
            circuits: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("circuits").unwrap(),
            ),
            hcx_cloud_manager_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hcxCloudManagerEndpoint").unwrap(),
            ),
            internet_connection_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("internetConnectionEnabled").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            management_cluster: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managementCluster").unwrap(),
            ),
            management_subnet_cidr: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managementSubnetCidr").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network_subnet_cidr: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkSubnetCidr").unwrap(),
            ),
            nsxt_certificate_thumbprint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nsxtCertificateThumbprint").unwrap(),
            ),
            nsxt_manager_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nsxtManagerEndpoint").unwrap(),
            ),
            nsxt_password: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nsxtPassword").unwrap(),
            ),
            provisioning_subnet_cidr: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("provisioningSubnetCidr").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            sku_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skuName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            vcenter_certificate_thumbprint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vcenterCertificateThumbprint").unwrap(),
            ),
            vcenter_password: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vcenterPassword").unwrap(),
            ),
            vcsa_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vcsaEndpoint").unwrap(),
            ),
            vmotion_subnet_cidr: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vmotionSubnetCidr").unwrap(),
            ),
        }
    }
}
