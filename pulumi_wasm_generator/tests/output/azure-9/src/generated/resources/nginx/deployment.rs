/// Manages an NGINX Deployment.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-rg
///       location: West Europe
///   examplePublicIp:
///     type: azure:network:PublicIp
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       allocationMethod: Static
///       sku: Standard
///       tags:
///         environment: Production
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: example-vnet
///       addressSpaces:
///         - 10.0.0.0/16
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleSubnet:
///     type: azure:network:Subnet
///     name: example
///     properties:
///       name: example-subnet
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.2.0/24
///       delegations:
///         - name: delegation
///           serviceDelegation:
///             name: NGINX.NGINXPLUS/nginxDeployments
///             actions:
///               - Microsoft.Network/virtualNetworks/subnets/join/action
///   exampleDeployment:
///     type: azure:nginx:Deployment
///     name: example
///     properties:
///       name: example-nginx
///       resourceGroupName: ${example.name}
///       sku: standardv2_Monthly
///       location: ${example.location}
///       diagnoseSupportEnabled: true
///       automaticUpgradeChannel: stable
///       frontendPublic:
///         ipAddresses:
///           - ${examplePublicIp.id}
///       networkInterfaces:
///         - subnetId: ${exampleSubnet.id}
///       capacity: 20
///       email: user@test.com
/// ```
///
/// ## Import
///
/// NGINX Deployments can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:nginx/deployment:Deployment example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/group1/providers/Nginx.NginxPlus/nginxDeployments/dep1
/// ```
///
pub mod deployment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DeploymentArgs {
        /// An `auto_scale_profile` block as defined below.
        #[builder(into, default)]
        pub auto_scale_profiles: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::nginx::DeploymentAutoScaleProfile>>,
        >,
        /// Specify the automatic upgrade channel for the NGINX deployment. Defaults to `stable`. The possible values are `stable` and `preview`.
        #[builder(into, default)]
        pub automatic_upgrade_channel: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specify the number of NGINX capacity units for this NGINX deployment.
        ///
        /// > **Note** For more information on NGINX capacity units, please refer to the [NGINX scaling guidance documentation](https://docs.nginx.com/nginxaas/azure/quickstart/scaling/)
        #[builder(into, default)]
        pub capacity: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Should the metrics be exported to Azure Monitor?
        #[builder(into, default)]
        pub diagnose_support_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Specify the preferred support contact email address for receiving alerts and notifications.
        #[builder(into, default)]
        pub email: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// One or more `frontend_private` blocks as defined below. Changing this forces a new NGINX Deployment to be created.
        #[builder(into, default)]
        pub frontend_privates: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::nginx::DeploymentFrontendPrivate>>,
        >,
        /// A `frontend_public` block as defined below. Changing this forces a new NGINX Deployment to be created.
        #[builder(into, default)]
        pub frontend_public: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::nginx::DeploymentFrontendPublic>,
        >,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::nginx::DeploymentIdentity>,
        >,
        /// The Azure Region where the NGINX Deployment should exist. Changing this forces a new NGINX Deployment to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// One or more `logging_storage_account` blocks as defined below.
        #[builder(into, default)]
        pub logging_storage_accounts: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::nginx::DeploymentLoggingStorageAccount>>,
        >,
        #[builder(into, default)]
        pub managed_resource_group: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this NGINX Deployment. Changing this forces a new NGINX Deployment to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// One or more `network_interface` blocks as defined below. Changing this forces a new NGINX Deployment to be created.
        #[builder(into, default)]
        pub network_interfaces: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::nginx::DeploymentNetworkInterface>>,
        >,
        /// The name of the Resource Group where the NGINX Deployment should exist. Changing this forces a new NGINX Deployment to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into)]
        pub sku: pulumi_wasm_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the NGINX Deployment.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DeploymentResult {
        /// An `auto_scale_profile` block as defined below.
        pub auto_scale_profiles: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::nginx::DeploymentAutoScaleProfile>>,
        >,
        /// Specify the automatic upgrade channel for the NGINX deployment. Defaults to `stable`. The possible values are `stable` and `preview`.
        pub automatic_upgrade_channel: pulumi_wasm_rust::Output<Option<String>>,
        /// Specify the number of NGINX capacity units for this NGINX deployment.
        ///
        /// > **Note** For more information on NGINX capacity units, please refer to the [NGINX scaling guidance documentation](https://docs.nginx.com/nginxaas/azure/quickstart/scaling/)
        pub capacity: pulumi_wasm_rust::Output<Option<i32>>,
        /// Should the metrics be exported to Azure Monitor?
        pub diagnose_support_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specify the preferred support contact email address for receiving alerts and notifications.
        pub email: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `frontend_private` blocks as defined below. Changing this forces a new NGINX Deployment to be created.
        pub frontend_privates: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::nginx::DeploymentFrontendPrivate>>,
        >,
        /// A `frontend_public` block as defined below. Changing this forces a new NGINX Deployment to be created.
        pub frontend_public: pulumi_wasm_rust::Output<
            Option<super::super::types::nginx::DeploymentFrontendPublic>,
        >,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::nginx::DeploymentIdentity>,
        >,
        /// The IP address of the deployment.
        pub ip_address: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the NGINX Deployment should exist. Changing this forces a new NGINX Deployment to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// One or more `logging_storage_account` blocks as defined below.
        pub logging_storage_accounts: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::nginx::DeploymentLoggingStorageAccount>>,
        >,
        pub managed_resource_group: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this NGINX Deployment. Changing this forces a new NGINX Deployment to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// One or more `network_interface` blocks as defined below. Changing this forces a new NGINX Deployment to be created.
        pub network_interfaces: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::nginx::DeploymentNetworkInterface>>,
        >,
        /// The version of deployed NGINX.
        pub nginx_version: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the NGINX Deployment should exist. Changing this forces a new NGINX Deployment to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        pub sku: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the NGINX Deployment.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: DeploymentArgs,
    ) -> DeploymentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let auto_scale_profiles_binding = args
            .auto_scale_profiles
            .get_output(context)
            .get_inner();
        let automatic_upgrade_channel_binding = args
            .automatic_upgrade_channel
            .get_output(context)
            .get_inner();
        let capacity_binding = args.capacity.get_output(context).get_inner();
        let diagnose_support_enabled_binding = args
            .diagnose_support_enabled
            .get_output(context)
            .get_inner();
        let email_binding = args.email.get_output(context).get_inner();
        let frontend_privates_binding = args
            .frontend_privates
            .get_output(context)
            .get_inner();
        let frontend_public_binding = args
            .frontend_public
            .get_output(context)
            .get_inner();
        let identity_binding = args.identity.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let logging_storage_accounts_binding = args
            .logging_storage_accounts
            .get_output(context)
            .get_inner();
        let managed_resource_group_binding = args
            .managed_resource_group
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let network_interfaces_binding = args
            .network_interfaces
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let sku_binding = args.sku.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:nginx/deployment:Deployment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autoScaleProfiles".into(),
                    value: &auto_scale_profiles_binding,
                },
                register_interface::ObjectField {
                    name: "automaticUpgradeChannel".into(),
                    value: &automatic_upgrade_channel_binding,
                },
                register_interface::ObjectField {
                    name: "capacity".into(),
                    value: &capacity_binding,
                },
                register_interface::ObjectField {
                    name: "diagnoseSupportEnabled".into(),
                    value: &diagnose_support_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "email".into(),
                    value: &email_binding,
                },
                register_interface::ObjectField {
                    name: "frontendPrivates".into(),
                    value: &frontend_privates_binding,
                },
                register_interface::ObjectField {
                    name: "frontendPublic".into(),
                    value: &frontend_public_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "loggingStorageAccounts".into(),
                    value: &logging_storage_accounts_binding,
                },
                register_interface::ObjectField {
                    name: "managedResourceGroup".into(),
                    value: &managed_resource_group_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "networkInterfaces".into(),
                    value: &network_interfaces_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "sku".into(),
                    value: &sku_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
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
                    name: "frontendPublic".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
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
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DeploymentResult {
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
            frontend_public: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("frontendPublic").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
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
