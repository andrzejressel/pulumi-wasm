/// Manages an association between Private Endpoint and Application Security Group.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-PEASGAsso
///       location: West Europe
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: examplevnet
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       addressSpaces:
///         - 10.5.0.0/16
///   service:
///     type: azure:network:Subnet
///     properties:
///       name: examplenetservice
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.5.1.0/24
///       enforcePrivateLinkServiceNetworkPolicies: true
///   endpoint:
///     type: azure:network:Subnet
///     properties:
///       name: examplenetendpoint
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.5.2.0/24
///       enforcePrivateLinkEndpointNetworkPolicies: true
///   examplePublicIp:
///     type: azure:network:PublicIp
///     name: example
///     properties:
///       name: examplepip
///       sku: Standard
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       allocationMethod: Static
///   exampleLoadBalancer:
///     type: azure:lb:LoadBalancer
///     name: example
///     properties:
///       name: examplelb
///       sku: Standard
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       frontendIpConfigurations:
///         - name: ${examplePublicIp.name}
///           publicIpAddressId: ${examplePublicIp.id}
///   exampleLinkService:
///     type: azure:privatedns:LinkService
///     name: example
///     properties:
///       name: examplePLS
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       autoApprovalSubscriptionIds:
///         - ${current.subscriptionId}
///       visibilitySubscriptionIds:
///         - ${current.subscriptionId}
///       natIpConfigurations:
///         - name: primaryIpConfiguration
///           primary: true
///           subnetId: ${service.id}
///       loadBalancerFrontendIpConfigurationIds:
///         - ${exampleLoadBalancer.frontendIpConfigurations[0].id}
///   exampleEndpoint:
///     type: azure:privatelink:Endpoint
///     name: example
///     properties:
///       name: example-privatelink
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       subnetId: ${endpoint.id}
///       privateServiceConnection:
///         name: ${exampleLinkService.name}
///         isManualConnection: false
///         privateConnectionResourceId: ${exampleLinkService.id}
///   exampleApplicationSecurityGroup:
///     type: azure:network:ApplicationSecurityGroup
///     name: example
///     properties:
///       name: example
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleApplicationSecurityGroupAssociation:
///     type: azure:privatelink:ApplicationSecurityGroupAssociation
///     name: example
///     properties:
///       privateEndpointId: ${exampleEndpoint.id}
///       applicationSecurityGroupId: ${exampleApplicationSecurityGroup.id}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getSubscription
///       arguments: {}
/// ```
///
/// ## Import
///
/// Associations between Private Endpoint and Application Security Group can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:privatelink/applicationSecurityGroupAssociation:ApplicationSecurityGroupAssociation association1 "/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Network/privateEndpoints/endpoints1|/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/applicationSecurityGroups/securityGroup1",
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod application_security_group_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApplicationSecurityGroupAssociationArgs {
        /// The id of application security group to associate. Changing this forces a new resource to be created.
        #[builder(into)]
        pub application_security_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The id of private endpoint to associate. Changing this forces a new resource to be created.
        #[builder(into)]
        pub private_endpoint_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ApplicationSecurityGroupAssociationResult {
        /// The id of application security group to associate. Changing this forces a new resource to be created.
        pub application_security_group_id: pulumi_gestalt_rust::Output<String>,
        /// The id of private endpoint to associate. Changing this forces a new resource to be created.
        pub private_endpoint_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ApplicationSecurityGroupAssociationArgs,
    ) -> ApplicationSecurityGroupAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let application_security_group_id_binding = args
            .application_security_group_id
            .get_output(context);
        let private_endpoint_id_binding = args.private_endpoint_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:privatelink/applicationSecurityGroupAssociation:ApplicationSecurityGroupAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationSecurityGroupId".into(),
                    value: &application_security_group_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "privateEndpointId".into(),
                    value: &private_endpoint_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ApplicationSecurityGroupAssociationResult {
            application_security_group_id: o.get_field("applicationSecurityGroupId"),
            private_endpoint_id: o.get_field("privateEndpointId"),
        }
    }
}
