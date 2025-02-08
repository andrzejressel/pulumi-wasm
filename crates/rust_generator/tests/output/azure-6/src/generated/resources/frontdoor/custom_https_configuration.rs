/// !> **IMPORTANT** This deploys an Azure Front Door (classic) resource which has been deprecated and will receive security updates only. Please migrate your existing Azure Front Door (classic) deployments to the new Azure Front Door (standard/premium) resources. For your convenience, the service team has exposed a `Front Door Classic` to `Front Door Standard/Premium` [migration tool](https://learn.microsoft.com/azure/frontdoor/tier-migration) to allow you to migrate your existing `Front Door Classic` instances to the new `Front Door Standard/Premium` product tiers.
///
/// Manages the Custom HTTPS Configuration for an Azure Front Door (classic) Frontend Endpoint.
///
/// > **NOTE:** Defining custom HTTPS configurations using a separate `azure.frontdoor.CustomHttpsConfiguration` resource allows for parallel creation/update.
///
/// !> **BREAKING CHANGE:** In order to address the ordering issue we have changed the design on how to retrieve existing sub resources such as frontend endpoints. Existing design will be deprecated and will result in an incorrect configuration. Please refer to the updated documentation below for more information.
///
/// !> **BREAKING CHANGE:** The `resource_group_name` field has been removed as of the `v2.58.0` provider release. If the `resource_group_name` field has been defined in your current `azure.frontdoor.CustomHttpsConfiguration` resource configuration file please remove it else you will receive a `An argument named "resource_group_name" is not expected here.` error. If your pre-existing Front Door instance contained inline `custom_https_configuration` blocks there are additional steps that will need to be completed to successfully migrate your Front Door onto the `v2.58.0` provider which can be found in this guide.
///
/// !> **Be Aware:** Azure is rolling out a breaking change on Friday 9th April 2021 which may cause issues with the CDN/FrontDoor resources. More information is available in this GitHub issue - however unfortunately this may necessitate a breaking change to the CDN and Front Door resources, more information will be posted in the GitHub issue as the necessary changes are identified.
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: FrontDoorExampleResourceGroup
///       location: West Europe
///   exampleFrontdoor:
///     type: azure:frontdoor:Frontdoor
///     name: example
///     properties:
///       name: example-FrontDoor
///       resourceGroupName: ${example.name}
///       routingRules:
///         - name: exampleRoutingRule1
///           acceptedProtocols:
///             - Http
///             - Https
///           patternsToMatches:
///             - /*
///           frontendEndpoints:
///             - exampleFrontendEndpoint1
///           forwardingConfiguration:
///             forwardingProtocol: MatchRequest
///             backendPoolName: exampleBackendBing
///       backendPoolLoadBalancings:
///         - name: exampleLoadBalancingSettings1
///       backendPoolHealthProbes:
///         - name: exampleHealthProbeSetting1
///       backendPools:
///         - name: exampleBackendBing
///           backends:
///             - hostHeader: www.bing.com
///               address: www.bing.com
///               httpPort: 80
///               httpsPort: 443
///           loadBalancingName: exampleLoadBalancingSettings1
///           healthProbeName: exampleHealthProbeSetting1
///       frontendEndpoints:
///         - name: exampleFrontendEndpoint1
///           hostName: example-FrontDoor.azurefd.net
///         - name: exampleFrontendEndpoint2
///           hostName: examplefd1.examplefd.net
///   exampleCustomHttps0:
///     type: azure:frontdoor:CustomHttpsConfiguration
///     name: example_custom_https_0
///     properties:
///       frontendEndpointId: ${exampleFrontdoor.frontendEndpointsMap.exampleFrontendEndpoint1}
///       customHttpsProvisioningEnabled: false
///   exampleCustomHttps1:
///     type: azure:frontdoor:CustomHttpsConfiguration
///     name: example_custom_https_1
///     properties:
///       frontendEndpointId: ${exampleFrontdoor.frontendEndpointsMap.exampleFrontendEndpoint2}
///       customHttpsProvisioningEnabled: true
///       customHttpsConfiguration:
///         certificateSource: AzureKeyVault
///         azureKeyVaultCertificateSecretName: examplefd1
///         azureKeyVaultCertificateVaultId: ${vault.id}
/// variables:
///   vault:
///     fn::invoke:
///       function: azure:keyvault:getKeyVault
///       arguments:
///         name: example-vault
///         resourceGroupName: example-vault-rg
/// ```
///
/// ## Import
///
/// Front Door Custom HTTPS Configurations can be imported using the `resource id` of the Front Door Custom HTTPS Configuration, e.g.
///
/// ```sh
/// $ pulumi import azure:frontdoor/customHttpsConfiguration:CustomHttpsConfiguration example_custom_https_1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/frontDoors/frontdoor1/customHttpsConfiguration/endpoint1
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod custom_https_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CustomHttpsConfigurationArgs {
        /// A `custom_https_configuration` block as defined above.
        #[builder(into, default)]
        pub custom_https_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::frontdoor::CustomHttpsConfigurationCustomHttpsConfiguration,
            >,
        >,
        /// Should the HTTPS protocol be enabled for this custom domain associated with the Front Door?
        #[builder(into)]
        pub custom_https_provisioning_enabled: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// The ID of the Front Door Frontend Endpoint which this configuration refers to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub frontend_endpoint_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct CustomHttpsConfigurationResult {
        /// A `custom_https_configuration` block as defined above.
        pub custom_https_configuration: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::frontdoor::CustomHttpsConfigurationCustomHttpsConfiguration,
            >,
        >,
        /// Should the HTTPS protocol be enabled for this custom domain associated with the Front Door?
        pub custom_https_provisioning_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The ID of the Front Door Frontend Endpoint which this configuration refers to. Changing this forces a new resource to be created.
        pub frontend_endpoint_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: CustomHttpsConfigurationArgs,
    ) -> CustomHttpsConfigurationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let custom_https_configuration_binding = args
            .custom_https_configuration
            .get_output(context)
            .get_inner();
        let custom_https_provisioning_enabled_binding = args
            .custom_https_provisioning_enabled
            .get_output(context)
            .get_inner();
        let frontend_endpoint_id_binding = args
            .frontend_endpoint_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:frontdoor/customHttpsConfiguration:CustomHttpsConfiguration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "customHttpsConfiguration".into(),
                    value: &custom_https_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "customHttpsProvisioningEnabled".into(),
                    value: &custom_https_provisioning_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "frontendEndpointId".into(),
                    value: &frontend_endpoint_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        CustomHttpsConfigurationResult {
            custom_https_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customHttpsConfiguration"),
            ),
            custom_https_provisioning_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customHttpsProvisioningEnabled"),
            ),
            frontend_endpoint_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("frontendEndpointId"),
            ),
        }
    }
}
