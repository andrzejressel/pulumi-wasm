/// Manages a Custom Domain for a CDN Endpoint.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: example-rg
///       location: west europe
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${exampleResourceGroup.name}
///       location: ${exampleResourceGroup.location}
///       accountTier: Standard
///       accountReplicationType: GRS
///   exampleProfile:
///     type: azure:cdn:Profile
///     name: example
///     properties:
///       name: example-profile
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       sku: Standard_Verizon
///   exampleEndpoint:
///     type: azure:cdn:Endpoint
///     name: example
///     properties:
///       name: example-endpoint
///       profileName: ${exampleProfile.name}
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       origins:
///         - name: example
///           hostName: ${exampleAccount.primaryBlobHost}
///   exampleCNameRecord:
///     type: azure:dns:CNameRecord
///     name: example
///     properties:
///       name: example
///       zoneName: ${example.name}
///       resourceGroupName: ${example.resourceGroupName}
///       ttl: 3600
///       targetResourceId: ${exampleEndpoint.id}
///   exampleEndpointCustomDomain:
///     type: azure:cdn:EndpointCustomDomain
///     name: example
///     properties:
///       name: example-domain
///       cdnEndpointId: ${exampleEndpoint.id}
///       hostName: ${exampleCNameRecord.name}.${example.name}
/// variables:
///   example:
///     fn::invoke:
///       function: azure:dns:getZone
///       arguments:
///         name: example-domain.com
///         resourceGroupName: domain-rg
/// ```
///
/// ## Import
///
/// CDN Endpoint Custom Domains can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:cdn/endpointCustomDomain:EndpointCustomDomain example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Cdn/profiles/profile1/endpoints/endpoint1/customDomains/domain1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod endpoint_custom_domain {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EndpointCustomDomainArgs {
        /// The ID of the CDN Endpoint. Changing this forces a new CDN Endpoint Custom Domain to be created.
        #[builder(into)]
        pub cdn_endpoint_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `cdn_managed_https` block as defined below.
        #[builder(into, default)]
        pub cdn_managed_https: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cdn::EndpointCustomDomainCdnManagedHttps>,
        >,
        /// The host name of the custom domain. Changing this forces a new CDN Endpoint Custom Domain to be created.
        #[builder(into)]
        pub host_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this CDN Endpoint Custom Domain. Changing this forces a new CDN Endpoint Custom Domain to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `user_managed_https` block as defined below.
        ///
        /// > **NOTE** Only one of `cdn_managed_https` and `user_managed_https` can be specified.
        #[builder(into, default)]
        pub user_managed_https: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cdn::EndpointCustomDomainUserManagedHttps>,
        >,
    }
    #[allow(dead_code)]
    pub struct EndpointCustomDomainResult {
        /// The ID of the CDN Endpoint. Changing this forces a new CDN Endpoint Custom Domain to be created.
        pub cdn_endpoint_id: pulumi_gestalt_rust::Output<String>,
        /// A `cdn_managed_https` block as defined below.
        pub cdn_managed_https: pulumi_gestalt_rust::Output<
            Option<super::super::types::cdn::EndpointCustomDomainCdnManagedHttps>,
        >,
        /// The host name of the custom domain. Changing this forces a new CDN Endpoint Custom Domain to be created.
        pub host_name: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this CDN Endpoint Custom Domain. Changing this forces a new CDN Endpoint Custom Domain to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `user_managed_https` block as defined below.
        ///
        /// > **NOTE** Only one of `cdn_managed_https` and `user_managed_https` can be specified.
        pub user_managed_https: pulumi_gestalt_rust::Output<
            Option<super::super::types::cdn::EndpointCustomDomainUserManagedHttps>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EndpointCustomDomainArgs,
    ) -> EndpointCustomDomainResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cdn_endpoint_id_binding = args.cdn_endpoint_id.get_output(context);
        let cdn_managed_https_binding = args.cdn_managed_https.get_output(context);
        let host_name_binding = args.host_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let user_managed_https_binding = args.user_managed_https.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:cdn/endpointCustomDomain:EndpointCustomDomain".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cdnEndpointId".into(),
                    value: cdn_endpoint_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cdnManagedHttps".into(),
                    value: cdn_managed_https_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hostName".into(),
                    value: host_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userManagedHttps".into(),
                    value: user_managed_https_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        EndpointCustomDomainResult {
            cdn_endpoint_id: o.get_field("cdnEndpointId"),
            cdn_managed_https: o.get_field("cdnManagedHttps"),
            host_name: o.get_field("hostName"),
            name: o.get_field("name"),
            user_managed_https: o.get_field("userManagedHttps"),
        }
    }
}
