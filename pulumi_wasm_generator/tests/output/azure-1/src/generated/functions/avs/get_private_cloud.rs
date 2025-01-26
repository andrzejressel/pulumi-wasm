pub mod get_private_cloud {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPrivateCloudArgs {
        /// The name of this Azure VMware Solution Private Cloud.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Azure VMware Solution Private Cloud exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetPrivateCloudResult {
        /// A `circuit` block as defined below.
        pub circuits: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::avs::GetPrivateCloudCircuit>,
        >,
        /// The endpoint for the VMware HCX Cloud Manager.
        pub hcx_cloud_manager_endpoint: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Is the Azure VMware Solution Private Cloud connected to the internet?
        pub internet_connection_enabled: pulumi_wasm_rust::Output<bool>,
        /// The Azure Region where the Azure VMware Solution Private Cloud exists.
        pub location: pulumi_wasm_rust::Output<String>,
        /// A `management_cluster` block as defined below.
        pub management_clusters: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::avs::GetPrivateCloudManagementCluster>,
        >,
        /// The network used to access VMware vCenter Server and NSX Manager.
        pub management_subnet_cidr: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// The subnet CIDR of the Azure VMware Solution Private Cloud.
        pub network_subnet_cidr: pulumi_wasm_rust::Output<String>,
        /// The thumbprint of the VMware NSX Manager SSL certificate.
        pub nsxt_certificate_thumbprint: pulumi_wasm_rust::Output<String>,
        /// The endpoint for the VMware NSX Manager.
        pub nsxt_manager_endpoint: pulumi_wasm_rust::Output<String>,
        /// The network which isused for virtual machine cold migration, cloning, and snapshot migration.
        pub provisioning_subnet_cidr: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The Name of the SKU used for this Azure VMware Solution Private Cloud.
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to the Azure VMware Solution Private Cloud.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The thumbprint of the VMware vCenter Server SSL certificate.
        pub vcenter_certificate_thumbprint: pulumi_wasm_rust::Output<String>,
        /// The endpoint for VMware vCenter Server Appliance.
        pub vcsa_endpoint: pulumi_wasm_rust::Output<String>,
        /// The network which is used for live migration of virtual machines.
        pub vmotion_subnet_cidr: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetPrivateCloudArgs,
    ) -> GetPrivateCloudResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:avs/getPrivateCloud:getPrivateCloud".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
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
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "internetConnectionEnabled".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "managementClusters".into(),
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
                    name: "vcsaEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "vmotionSubnetCidr".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetPrivateCloudResult {
            circuits: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("circuits").unwrap(),
            ),
            hcx_cloud_manager_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hcxCloudManagerEndpoint").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            internet_connection_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("internetConnectionEnabled").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            management_clusters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managementClusters").unwrap(),
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
            vcsa_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vcsaEndpoint").unwrap(),
            ),
            vmotion_subnet_cidr: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vmotionSubnetCidr").unwrap(),
            ),
        }
    }
}
