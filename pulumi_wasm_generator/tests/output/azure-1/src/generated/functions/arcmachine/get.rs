pub mod get {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetArgs {
        /// The name of this Azure Arc machine.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Hybrid Compute exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetResult {
        /// Specifies the AD fully qualified display name.
        pub active_directory_fqdn: pulumi_wasm_rust::Output<String>,
        /// The Azure Arc machine agent full version.
        pub agent_version: pulumi_wasm_rust::Output<String>,
        /// A `agent` block as defined below.
        pub agents: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::arcmachine::GetAgent>,
        >,
        /// Public Key that the client provides to be used during initial resource onboarding.
        pub client_public_key: pulumi_wasm_rust::Output<String>,
        /// A `cloud_metadata` block as defined below.
        pub cloud_metadatas: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::arcmachine::GetCloudMetadata>,
        >,
        /// A `detected_properties` block as defined below.
        pub detected_properties: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Specifies the Azure Arc machine display name.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the DNS fully qualified display name.
        pub dns_fqdn: pulumi_wasm_rust::Output<String>,
        /// Specifies the Windows domain name.
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// A `identity` block as defined below.
        pub identities: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::arcmachine::GetIdentity>,
        >,
        /// The time of the last status change.
        pub last_status_change_time: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the Azure Arc machine exists.
        pub location: pulumi_wasm_rust::Output<String>,
        /// A `location_data` block as defined below.
        pub location_datas: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::arcmachine::GetLocationData>,
        >,
        /// Specifies the Azure Arc machine fully qualified display name.
        pub machine_fqdn: pulumi_wasm_rust::Output<String>,
        /// Specifies whether any MS SQL instance is discovered on the machine.
        pub mssql_discovered: pulumi_wasm_rust::Output<bool>,
        /// A canonical name for the geographic or physical location.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The Operating System running on the Azure Arc machine.
        pub os_name: pulumi_wasm_rust::Output<String>,
        /// A `os_profile` block as defined below.
        pub os_profiles: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::arcmachine::GetOsProfile>,
        >,
        /// Specifies the Operating System product SKU.
        pub os_sku: pulumi_wasm_rust::Output<String>,
        /// The type of Operating System. Possible values are `windows` and `linux`.
        pub os_type: pulumi_wasm_rust::Output<String>,
        /// The version of Operating System running on the Azure Arc machine.
        pub os_version: pulumi_wasm_rust::Output<String>,
        /// The resource id of the parent cluster (Azure HCI) this machine is assigned to, if any.
        pub parent_cluster_resource_id: pulumi_wasm_rust::Output<String>,
        /// The resource id of the parent cluster (Azure HCI) this machine is assigned to, if any.
        pub private_link_scope_resource_id: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `service_status` block as defined below.
        pub service_statuses: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::arcmachine::GetServiceStatus>,
        >,
        /// The current status of the service.
        pub status: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to the Hybrid Compute.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Specifies the Azure Arc machine unique ID.
        pub vm_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the Arc Machine's unique SMBIOS ID.
        pub vm_uuid: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetArgs,
    ) -> GetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:arcmachine/get:get".into(),
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
                    name: "activeDirectoryFqdn".into(),
                },
                register_interface::ResultField {
                    name: "agentVersion".into(),
                },
                register_interface::ResultField {
                    name: "agents".into(),
                },
                register_interface::ResultField {
                    name: "clientPublicKey".into(),
                },
                register_interface::ResultField {
                    name: "cloudMetadatas".into(),
                },
                register_interface::ResultField {
                    name: "detectedProperties".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "dnsFqdn".into(),
                },
                register_interface::ResultField {
                    name: "domainName".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "identities".into(),
                },
                register_interface::ResultField {
                    name: "lastStatusChangeTime".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "locationDatas".into(),
                },
                register_interface::ResultField {
                    name: "machineFqdn".into(),
                },
                register_interface::ResultField {
                    name: "mssqlDiscovered".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "osName".into(),
                },
                register_interface::ResultField {
                    name: "osProfiles".into(),
                },
                register_interface::ResultField {
                    name: "osSku".into(),
                },
                register_interface::ResultField {
                    name: "osType".into(),
                },
                register_interface::ResultField {
                    name: "osVersion".into(),
                },
                register_interface::ResultField {
                    name: "parentClusterResourceId".into(),
                },
                register_interface::ResultField {
                    name: "privateLinkScopeResourceId".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "serviceStatuses".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "vmId".into(),
                },
                register_interface::ResultField {
                    name: "vmUuid".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetResult {
            active_directory_fqdn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("activeDirectoryFqdn").unwrap(),
            ),
            agent_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("agentVersion").unwrap(),
            ),
            agents: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("agents").unwrap(),
            ),
            client_public_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientPublicKey").unwrap(),
            ),
            cloud_metadatas: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cloudMetadatas").unwrap(),
            ),
            detected_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("detectedProperties").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            dns_fqdn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsFqdn").unwrap(),
            ),
            domain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainName").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            identities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identities").unwrap(),
            ),
            last_status_change_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastStatusChangeTime").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            location_datas: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("locationDatas").unwrap(),
            ),
            machine_fqdn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("machineFqdn").unwrap(),
            ),
            mssql_discovered: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mssqlDiscovered").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            os_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("osName").unwrap(),
            ),
            os_profiles: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("osProfiles").unwrap(),
            ),
            os_sku: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("osSku").unwrap(),
            ),
            os_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("osType").unwrap(),
            ),
            os_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("osVersion").unwrap(),
            ),
            parent_cluster_resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parentClusterResourceId").unwrap(),
            ),
            private_link_scope_resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateLinkScopeResourceId").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            service_statuses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceStatuses").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            vm_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vmId").unwrap(),
            ),
            vm_uuid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vmUuid").unwrap(),
            ),
        }
    }
}
