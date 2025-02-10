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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod deployment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DeploymentArgs {
        /// An `auto_scale_profile` block as defined below.
        #[builder(into, default)]
        pub auto_scale_profiles: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::nginx::DeploymentAutoScaleProfile>>,
        >,
        /// Specify the automatic upgrade channel for the NGINX deployment. Defaults to `stable`. The possible values are `stable` and `preview`.
        #[builder(into, default)]
        pub automatic_upgrade_channel: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specify the number of NGINX capacity units for this NGINX deployment.
        ///
        /// > **Note** For more information on NGINX capacity units, please refer to the [NGINX scaling guidance documentation](https://docs.nginx.com/nginxaas/azure/quickstart/scaling/)
        #[builder(into, default)]
        pub capacity: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Should the metrics be exported to Azure Monitor?
        #[builder(into, default)]
        pub diagnose_support_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specify the preferred support contact email address for receiving alerts and notifications.
        #[builder(into, default)]
        pub email: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `frontend_private` blocks as defined below. Changing this forces a new NGINX Deployment to be created.
        #[builder(into, default)]
        pub frontend_privates: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::nginx::DeploymentFrontendPrivate>>,
        >,
        /// A `frontend_public` block as defined below. Changing this forces a new NGINX Deployment to be created.
        #[builder(into, default)]
        pub frontend_public: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::nginx::DeploymentFrontendPublic>,
        >,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::nginx::DeploymentIdentity>,
        >,
        /// The Azure Region where the NGINX Deployment should exist. Changing this forces a new NGINX Deployment to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `logging_storage_account` blocks as defined below.
        #[builder(into, default)]
        pub logging_storage_accounts: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::nginx::DeploymentLoggingStorageAccount>>,
        >,
        #[builder(into, default)]
        pub managed_resource_group: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this NGINX Deployment. Changing this forces a new NGINX Deployment to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `network_interface` blocks as defined below. Changing this forces a new NGINX Deployment to be created.
        #[builder(into, default)]
        pub network_interfaces: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::nginx::DeploymentNetworkInterface>>,
        >,
        /// The name of the Resource Group where the NGINX Deployment should exist. Changing this forces a new NGINX Deployment to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into)]
        pub sku: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the NGINX Deployment.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DeploymentResult {
        /// An `auto_scale_profile` block as defined below.
        pub auto_scale_profiles: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::nginx::DeploymentAutoScaleProfile>>,
        >,
        /// Specify the automatic upgrade channel for the NGINX deployment. Defaults to `stable`. The possible values are `stable` and `preview`.
        pub automatic_upgrade_channel: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specify the number of NGINX capacity units for this NGINX deployment.
        ///
        /// > **Note** For more information on NGINX capacity units, please refer to the [NGINX scaling guidance documentation](https://docs.nginx.com/nginxaas/azure/quickstart/scaling/)
        pub capacity: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Should the metrics be exported to Azure Monitor?
        pub diagnose_support_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specify the preferred support contact email address for receiving alerts and notifications.
        pub email: pulumi_gestalt_rust::Output<Option<String>>,
        /// One or more `frontend_private` blocks as defined below. Changing this forces a new NGINX Deployment to be created.
        pub frontend_privates: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::nginx::DeploymentFrontendPrivate>>,
        >,
        /// A `frontend_public` block as defined below. Changing this forces a new NGINX Deployment to be created.
        pub frontend_public: pulumi_gestalt_rust::Output<
            Option<super::super::types::nginx::DeploymentFrontendPublic>,
        >,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::nginx::DeploymentIdentity>,
        >,
        /// The IP address of the deployment.
        pub ip_address: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the NGINX Deployment should exist. Changing this forces a new NGINX Deployment to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// One or more `logging_storage_account` blocks as defined below.
        pub logging_storage_accounts: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::nginx::DeploymentLoggingStorageAccount>>,
        >,
        pub managed_resource_group: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this NGINX Deployment. Changing this forces a new NGINX Deployment to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// One or more `network_interface` blocks as defined below. Changing this forces a new NGINX Deployment to be created.
        pub network_interfaces: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::nginx::DeploymentNetworkInterface>>,
        >,
        /// The version of deployed NGINX.
        pub nginx_version: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the NGINX Deployment should exist. Changing this forces a new NGINX Deployment to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        pub sku: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the NGINX Deployment.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DeploymentArgs,
    ) -> DeploymentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let auto_scale_profiles_binding = args.auto_scale_profiles.get_output(context);
        let automatic_upgrade_channel_binding = args
            .automatic_upgrade_channel
            .get_output(context);
        let capacity_binding = args.capacity.get_output(context);
        let diagnose_support_enabled_binding = args
            .diagnose_support_enabled
            .get_output(context);
        let email_binding = args.email.get_output(context);
        let frontend_privates_binding = args.frontend_privates.get_output(context);
        let frontend_public_binding = args.frontend_public.get_output(context);
        let identity_binding = args.identity.get_output(context);
        let location_binding = args.location.get_output(context);
        let logging_storage_accounts_binding = args
            .logging_storage_accounts
            .get_output(context);
        let managed_resource_group_binding = args
            .managed_resource_group
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let network_interfaces_binding = args.network_interfaces.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let sku_binding = args.sku.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:nginx/deployment:Deployment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoScaleProfiles".into(),
                    value: auto_scale_profiles_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "automaticUpgradeChannel".into(),
                    value: automatic_upgrade_channel_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "capacity".into(),
                    value: capacity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "diagnoseSupportEnabled".into(),
                    value: diagnose_support_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "email".into(),
                    value: email_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "frontendPrivates".into(),
                    value: frontend_privates_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "frontendPublic".into(),
                    value: frontend_public_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: identity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "loggingStorageAccounts".into(),
                    value: logging_storage_accounts_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "managedResourceGroup".into(),
                    value: managed_resource_group_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkInterfaces".into(),
                    value: network_interfaces_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sku".into(),
                    value: sku_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DeploymentResult {
            auto_scale_profiles: o.get_field("autoScaleProfiles"),
            automatic_upgrade_channel: o.get_field("automaticUpgradeChannel"),
            capacity: o.get_field("capacity"),
            diagnose_support_enabled: o.get_field("diagnoseSupportEnabled"),
            email: o.get_field("email"),
            frontend_privates: o.get_field("frontendPrivates"),
            frontend_public: o.get_field("frontendPublic"),
            identity: o.get_field("identity"),
            ip_address: o.get_field("ipAddress"),
            location: o.get_field("location"),
            logging_storage_accounts: o.get_field("loggingStorageAccounts"),
            managed_resource_group: o.get_field("managedResourceGroup"),
            name: o.get_field("name"),
            network_interfaces: o.get_field("networkInterfaces"),
            nginx_version: o.get_field("nginxVersion"),
            resource_group_name: o.get_field("resourceGroupName"),
            sku: o.get_field("sku"),
            tags: o.get_field("tags"),
        }
    }
}
