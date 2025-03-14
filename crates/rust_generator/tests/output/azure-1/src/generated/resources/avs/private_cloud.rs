/// Manages an Azure VMware Solution Private Cloud.
///
/// ## Example Usage
///
/// > **NOTE :**  Normal `pulumi up` could ignore this note. Please disable correlation request id for continuous operations in one build (like acctest). The continuous operations like `update` or `delete` could not be triggered when it shares the same `correlation-id` with its previous operation.
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod private_cloud {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PrivateCloudArgs {
        /// Is the Azure VMware Solution Private Cloud connected to the internet? This field can not be updated with `management_cluster[0].size` together.
        /// > **NOTE :** `internet_connection_enabled` and `management_cluster[0].size` cannot be updated at the same time.
        #[builder(into, default)]
        pub internet_connection_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The Azure Region where the Azure VMware Solution Private Cloud should exist. Changing this forces a new Azure VMware Solution Private Cloud to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `management_cluster` block as defined below.
        /// > **NOTE :** `internet_connection_enabled` and `management_cluster[0].size` cannot be updated at the same time.
        #[builder(into)]
        pub management_cluster: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::avs::PrivateCloudManagementCluster,
        >,
        /// The name which should be used for this Azure VMware Solution Private Cloud. Changing this forces a new Azure VMware Solution Private Cloud to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The subnet which should be unique across virtual network in your subscription as well as on-premise. Changing this forces a new Azure VMware Solution Private Cloud to be created.
        #[builder(into)]
        pub network_subnet_cidr: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The password of the VMware NSX Manager cloudadmin. Changing this forces a new Azure VMware Solution Private Cloud to be created.
        #[builder(into, default)]
        pub nsxt_password: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Azure VMware Solution Private Cloud should exist. Changing this forces a new Azure VMware Solution Private Cloud to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Name of the SKU used for this Azure VMware Solution Private Cloud. Possible values are `av20`, `av36`, `av36t`, `av36p`, `av36pt`, `av52`, `av52t`, and `av64`. Changing this forces a new Azure VMware Solution Private Cloud to be created.
        #[builder(into)]
        pub sku_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Azure VMware Solution Private Cloud.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The password of the VMware vCenter Server cloudadmin. Changing this forces a new Azure VMware Solution Private Cloud to be created.
        #[builder(into, default)]
        pub vcenter_password: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct PrivateCloudResult {
        /// A `circuit` block as defined below.
        pub circuits: pulumi_gestalt_rust::Output<
            Vec<super::super::types::avs::PrivateCloudCircuit>,
        >,
        /// The endpoint for the VMware HCX Cloud Manager.
        pub hcx_cloud_manager_endpoint: pulumi_gestalt_rust::Output<String>,
        /// Is the Azure VMware Solution Private Cloud connected to the internet? This field can not be updated with `management_cluster[0].size` together.
        /// > **NOTE :** `internet_connection_enabled` and `management_cluster[0].size` cannot be updated at the same time.
        pub internet_connection_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The Azure Region where the Azure VMware Solution Private Cloud should exist. Changing this forces a new Azure VMware Solution Private Cloud to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// A `management_cluster` block as defined below.
        /// > **NOTE :** `internet_connection_enabled` and `management_cluster[0].size` cannot be updated at the same time.
        pub management_cluster: pulumi_gestalt_rust::Output<
            super::super::types::avs::PrivateCloudManagementCluster,
        >,
        /// The network used to access VMware vCenter Server and NSX Manager.
        pub management_subnet_cidr: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Azure VMware Solution Private Cloud. Changing this forces a new Azure VMware Solution Private Cloud to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The subnet which should be unique across virtual network in your subscription as well as on-premise. Changing this forces a new Azure VMware Solution Private Cloud to be created.
        pub network_subnet_cidr: pulumi_gestalt_rust::Output<String>,
        /// The thumbprint of the VMware NSX Manager SSL certificate.
        pub nsxt_certificate_thumbprint: pulumi_gestalt_rust::Output<String>,
        /// The endpoint for the VMware NSX Manager.
        pub nsxt_manager_endpoint: pulumi_gestalt_rust::Output<String>,
        /// The password of the VMware NSX Manager cloudadmin. Changing this forces a new Azure VMware Solution Private Cloud to be created.
        pub nsxt_password: pulumi_gestalt_rust::Output<Option<String>>,
        /// The network which is used for virtual machine cold migration, cloning, and snapshot migration.
        pub provisioning_subnet_cidr: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Azure VMware Solution Private Cloud should exist. Changing this forces a new Azure VMware Solution Private Cloud to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The Name of the SKU used for this Azure VMware Solution Private Cloud. Possible values are `av20`, `av36`, `av36t`, `av36p`, `av36pt`, `av52`, `av52t`, and `av64`. Changing this forces a new Azure VMware Solution Private Cloud to be created.
        pub sku_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Azure VMware Solution Private Cloud.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The thumbprint of the VMware vCenter Server SSL certificate.
        pub vcenter_certificate_thumbprint: pulumi_gestalt_rust::Output<String>,
        /// The password of the VMware vCenter Server cloudadmin. Changing this forces a new Azure VMware Solution Private Cloud to be created.
        pub vcenter_password: pulumi_gestalt_rust::Output<Option<String>>,
        /// The endpoint for VMware vCenter Server Appliance.
        pub vcsa_endpoint: pulumi_gestalt_rust::Output<String>,
        /// The network which is used for live migration of virtual machines.
        pub vmotion_subnet_cidr: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PrivateCloudArgs,
    ) -> PrivateCloudResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let internet_connection_enabled_binding = args
            .internet_connection_enabled
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let management_cluster_binding = args.management_cluster.get_output(context);
        let name_binding = args.name.get_output(context);
        let network_subnet_cidr_binding = args.network_subnet_cidr.get_output(context);
        let nsxt_password_binding = args.nsxt_password.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let sku_name_binding = args.sku_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let vcenter_password_binding = args.vcenter_password.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:avs/privateCloud:PrivateCloud".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "internetConnectionEnabled".into(),
                    value: &internet_connection_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "managementCluster".into(),
                    value: &management_cluster_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkSubnetCidr".into(),
                    value: &network_subnet_cidr_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nsxtPassword".into(),
                    value: &nsxt_password_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skuName".into(),
                    value: &sku_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vcenterPassword".into(),
                    value: &vcenter_password_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        PrivateCloudResult {
            circuits: o.get_field("circuits"),
            hcx_cloud_manager_endpoint: o.get_field("hcxCloudManagerEndpoint"),
            internet_connection_enabled: o.get_field("internetConnectionEnabled"),
            location: o.get_field("location"),
            management_cluster: o.get_field("managementCluster"),
            management_subnet_cidr: o.get_field("managementSubnetCidr"),
            name: o.get_field("name"),
            network_subnet_cidr: o.get_field("networkSubnetCidr"),
            nsxt_certificate_thumbprint: o.get_field("nsxtCertificateThumbprint"),
            nsxt_manager_endpoint: o.get_field("nsxtManagerEndpoint"),
            nsxt_password: o.get_field("nsxtPassword"),
            provisioning_subnet_cidr: o.get_field("provisioningSubnetCidr"),
            resource_group_name: o.get_field("resourceGroupName"),
            sku_name: o.get_field("skuName"),
            tags: o.get_field("tags"),
            vcenter_certificate_thumbprint: o.get_field("vcenterCertificateThumbprint"),
            vcenter_password: o.get_field("vcenterPassword"),
            vcsa_endpoint: o.get_field("vcsaEndpoint"),
            vmotion_subnet_cidr: o.get_field("vmotionSubnetCidr"),
        }
    }
}
