pub mod get_deployment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDeploymentArgs {
        /// The name of this NGINX Deployment.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the NGINX Deployment exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetDeploymentResult {
        /// An `auto_scale_profile` block as defined below.
        pub auto_scale_profiles: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::nginx::GetDeploymentAutoScaleProfile>,
        >,
        /// The automatic upgrade channel for this NGINX deployment.
        pub automatic_upgrade_channel: pulumi_wasm_rust::Output<String>,
        /// The number of NGINX capacity units for this NGINX Deployment.
        pub capacity: pulumi_wasm_rust::Output<i32>,
        /// Whether metrics are exported to Azure Monitor.
        pub diagnose_support_enabled: pulumi_wasm_rust::Output<bool>,
        /// Preferred email associated with the NGINX Deployment.
        pub email: pulumi_wasm_rust::Output<String>,
        /// A `frontend_private` block as defined below.
        pub frontend_privates: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::nginx::GetDeploymentFrontendPrivate>,
        >,
        /// A `frontend_public` block as defined below.
        pub frontend_publics: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::nginx::GetDeploymentFrontendPublic>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// A `identity` block as defined below.
        pub identities: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::nginx::GetDeploymentIdentity>,
        >,
        /// The list of Public IP Resource IDs for this NGINX Deployment.
        pub ip_address: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the NGINX Deployment exists.
        pub location: pulumi_wasm_rust::Output<String>,
        /// A `logging_storage_account` block as defined below.
        pub logging_storage_accounts: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::nginx::GetDeploymentLoggingStorageAccount>,
        >,
        pub managed_resource_group: pulumi_wasm_rust::Output<String>,
        /// Name of the autoscaling profile.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `network_interface` block as defined below.
        pub network_interfaces: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::nginx::GetDeploymentNetworkInterface>,
        >,
        /// NGINX version of the Deployment.
        pub nginx_version: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The NGINX Deployment SKU.
        pub sku: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to the NGINX Deployment.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetDeploymentArgs,
    ) -> GetDeploymentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:nginx/getDeployment:getDeployment".into(),
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
                    name: "autoScaleProfiles".into(),
                },
                register_interface::ResultField {
                    name: "automaticUpgradeChannel".into(),
                },
                register_interface::ResultField {
                    name: "capacity".into(),
                },
                register_interface::ResultField {
                    name: "diagnoseSupportEnabled".into(),
                },
                register_interface::ResultField {
                    name: "email".into(),
                },
                register_interface::ResultField {
                    name: "frontendPrivates".into(),
                },
                register_interface::ResultField {
                    name: "frontendPublics".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "identities".into(),
                },
                register_interface::ResultField {
                    name: "ipAddress".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "loggingStorageAccounts".into(),
                },
                register_interface::ResultField {
                    name: "managedResourceGroup".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networkInterfaces".into(),
                },
                register_interface::ResultField {
                    name: "nginxVersion".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "sku".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetDeploymentResult {
            auto_scale_profiles: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoScaleProfiles").unwrap(),
            ),
            automatic_upgrade_channel: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("automaticUpgradeChannel").unwrap(),
            ),
            capacity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("capacity").unwrap(),
            ),
            diagnose_support_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("diagnoseSupportEnabled").unwrap(),
            ),
            email: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("email").unwrap(),
            ),
            frontend_privates: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("frontendPrivates").unwrap(),
            ),
            frontend_publics: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("frontendPublics").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            identities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identities").unwrap(),
            ),
            ip_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipAddress").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            logging_storage_accounts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loggingStorageAccounts").unwrap(),
            ),
            managed_resource_group: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managedResourceGroup").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network_interfaces: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkInterfaces").unwrap(),
            ),
            nginx_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nginxVersion").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            sku: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sku").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
