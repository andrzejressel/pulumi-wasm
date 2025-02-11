#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetArgs {
        /// The name of this Azure Arc machine.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Hybrid Compute exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetResult {
        /// Specifies the AD fully qualified display name.
        pub active_directory_fqdn: pulumi_gestalt_rust::Output<String>,
        /// The Azure Arc machine agent full version.
        pub agent_version: pulumi_gestalt_rust::Output<String>,
        /// A `agent` block as defined below.
        pub agents: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::arcmachine::GetAgent>,
        >,
        /// Public Key that the client provides to be used during initial resource onboarding.
        pub client_public_key: pulumi_gestalt_rust::Output<String>,
        /// A `cloud_metadata` block as defined below.
        pub cloud_metadatas: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::arcmachine::GetCloudMetadata>,
        >,
        /// A `detected_properties` block as defined below.
        pub detected_properties: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Specifies the Azure Arc machine display name.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the DNS fully qualified display name.
        pub dns_fqdn: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Windows domain name.
        pub domain_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A `identity` block as defined below.
        pub identities: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::arcmachine::GetIdentity>,
        >,
        /// The time of the last status change.
        pub last_status_change_time: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the Azure Arc machine exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// A `location_data` block as defined below.
        pub location_datas: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::arcmachine::GetLocationData>,
        >,
        /// Specifies the Azure Arc machine fully qualified display name.
        pub machine_fqdn: pulumi_gestalt_rust::Output<String>,
        /// Specifies whether any MS SQL instance is discovered on the machine.
        pub mssql_discovered: pulumi_gestalt_rust::Output<bool>,
        /// A canonical name for the geographic or physical location.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Operating System running on the Azure Arc machine.
        pub os_name: pulumi_gestalt_rust::Output<String>,
        /// A `os_profile` block as defined below.
        pub os_profiles: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::arcmachine::GetOsProfile>,
        >,
        /// Specifies the Operating System product SKU.
        pub os_sku: pulumi_gestalt_rust::Output<String>,
        /// The type of Operating System. Possible values are `windows` and `linux`.
        pub os_type: pulumi_gestalt_rust::Output<String>,
        /// The version of Operating System running on the Azure Arc machine.
        pub os_version: pulumi_gestalt_rust::Output<String>,
        /// The resource id of the parent cluster (Azure HCI) this machine is assigned to, if any.
        pub parent_cluster_resource_id: pulumi_gestalt_rust::Output<String>,
        /// The resource id of the parent cluster (Azure HCI) this machine is assigned to, if any.
        pub private_link_scope_resource_id: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A `service_status` block as defined below.
        pub service_statuses: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::arcmachine::GetServiceStatus>,
        >,
        /// The current status of the service.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags assigned to the Hybrid Compute.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Specifies the Azure Arc machine unique ID.
        pub vm_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Arc Machine's unique SMBIOS ID.
        pub vm_uuid: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(context: &pulumi_gestalt_rust::Context, args: GetArgs) -> GetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:arcmachine/get:get".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetResult {
            active_directory_fqdn: o.get_field("activeDirectoryFqdn"),
            agent_version: o.get_field("agentVersion"),
            agents: o.get_field("agents"),
            client_public_key: o.get_field("clientPublicKey"),
            cloud_metadatas: o.get_field("cloudMetadatas"),
            detected_properties: o.get_field("detectedProperties"),
            display_name: o.get_field("displayName"),
            dns_fqdn: o.get_field("dnsFqdn"),
            domain_name: o.get_field("domainName"),
            id: o.get_field("id"),
            identities: o.get_field("identities"),
            last_status_change_time: o.get_field("lastStatusChangeTime"),
            location: o.get_field("location"),
            location_datas: o.get_field("locationDatas"),
            machine_fqdn: o.get_field("machineFqdn"),
            mssql_discovered: o.get_field("mssqlDiscovered"),
            name: o.get_field("name"),
            os_name: o.get_field("osName"),
            os_profiles: o.get_field("osProfiles"),
            os_sku: o.get_field("osSku"),
            os_type: o.get_field("osType"),
            os_version: o.get_field("osVersion"),
            parent_cluster_resource_id: o.get_field("parentClusterResourceId"),
            private_link_scope_resource_id: o.get_field("privateLinkScopeResourceId"),
            resource_group_name: o.get_field("resourceGroupName"),
            service_statuses: o.get_field("serviceStatuses"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            vm_id: o.get_field("vmId"),
            vm_uuid: o.get_field("vmUuid"),
        }
    }
}
