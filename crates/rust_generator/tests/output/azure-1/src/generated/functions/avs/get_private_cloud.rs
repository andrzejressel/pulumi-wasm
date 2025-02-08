#[allow(clippy::doc_lazy_continuation)]
pub mod get_private_cloud {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPrivateCloudArgs {
        /// The name of this Azure VMware Solution Private Cloud.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Azure VMware Solution Private Cloud exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetPrivateCloudResult {
        /// A `circuit` block as defined below.
        pub circuits: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::avs::GetPrivateCloudCircuit>,
        >,
        /// The endpoint for the VMware HCX Cloud Manager.
        pub hcx_cloud_manager_endpoint: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Is the Azure VMware Solution Private Cloud connected to the internet?
        pub internet_connection_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The Azure Region where the Azure VMware Solution Private Cloud exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// A `management_cluster` block as defined below.
        pub management_clusters: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::avs::GetPrivateCloudManagementCluster>,
        >,
        /// The network used to access VMware vCenter Server and NSX Manager.
        pub management_subnet_cidr: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The subnet CIDR of the Azure VMware Solution Private Cloud.
        pub network_subnet_cidr: pulumi_gestalt_rust::Output<String>,
        /// The thumbprint of the VMware NSX Manager SSL certificate.
        pub nsxt_certificate_thumbprint: pulumi_gestalt_rust::Output<String>,
        /// The endpoint for the VMware NSX Manager.
        pub nsxt_manager_endpoint: pulumi_gestalt_rust::Output<String>,
        /// The network which isused for virtual machine cold migration, cloning, and snapshot migration.
        pub provisioning_subnet_cidr: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The Name of the SKU used for this Azure VMware Solution Private Cloud.
        pub sku_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags assigned to the Azure VMware Solution Private Cloud.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The thumbprint of the VMware vCenter Server SSL certificate.
        pub vcenter_certificate_thumbprint: pulumi_gestalt_rust::Output<String>,
        /// The endpoint for VMware vCenter Server Appliance.
        pub vcsa_endpoint: pulumi_gestalt_rust::Output<String>,
        /// The network which is used for live migration of virtual machines.
        pub vmotion_subnet_cidr: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetPrivateCloudArgs,
    ) -> GetPrivateCloudResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetPrivateCloudResult {
            circuits: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("circuits"),
            ),
            hcx_cloud_manager_endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hcxCloudManagerEndpoint"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            internet_connection_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("internetConnectionEnabled"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            management_clusters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("managementClusters"),
            ),
            management_subnet_cidr: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("managementSubnetCidr"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            network_subnet_cidr: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkSubnetCidr"),
            ),
            nsxt_certificate_thumbprint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nsxtCertificateThumbprint"),
            ),
            nsxt_manager_endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nsxtManagerEndpoint"),
            ),
            provisioning_subnet_cidr: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("provisioningSubnetCidr"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            sku_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("skuName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            vcenter_certificate_thumbprint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vcenterCertificateThumbprint"),
            ),
            vcsa_endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vcsaEndpoint"),
            ),
            vmotion_subnet_cidr: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vmotionSubnetCidr"),
            ),
        }
    }
}
