/// Manages an Azure Web PubSub Custom Domain.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleService:
///     type: azure:webpubsub:Service
///     name: example
///     properties:
///       name: example-webpubsub
///       location: ${testAzurermResourceGroup.location}
///       resourceGroupName: ${testAzurermResourceGroup.name}
///       sku:
///         - name: Premium_P1
///           capacity: 1
///       identity:
///         type: SystemAssigned
///   exampleKeyVault:
///     type: azure:keyvault:KeyVault
///     name: example
///     properties:
///       name: examplekeyvault
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       tenantId: ${current.tenantId}
///       skuName: premium
///       accessPolicies:
///         - tenantId: ${current.tenantId}
///           objectId: ${current.objectId}
///           certificatePermissions:
///             - Create
///             - Get
///             - List
///           secretPermissions:
///             - Get
///             - List
///         - tenantId: ${current.tenantId}
///           objectId: ${testAzurermWebPubsub.identity[0].principalId}
///           certificatePermissions:
///             - Create
///             - Get
///             - List
///           secretPermissions:
///             - Get
///             - List
///   exampleCertificate:
///     type: azure:keyvault:Certificate
///     name: example
///     properties:
///       name: imported-cert
///       keyVaultId: ${exampleKeyVault.id}
///       certificate:
///         contents:
///           fn::invoke:
///             function: std:filebase64
///             arguments:
///               input: certificate-to-import.pfx
///             return: result
///         password: ""
///   test:
///     type: azure:webpubsub:CustomCertificate
///     properties:
///       name: example-cert
///       webPubsubId: ${exampleService.id}
///       customCertificateId: ${exampleCertificate.id}
///     options:
///       dependsOn:
///         - ${exampleAzurermKeyVaultAccessPolicy}
///   testCustomDomain:
///     type: azure:webpubsub:CustomDomain
///     name: test
///     properties:
///       name: example-domain
///       domainName: tftest.com
///       webPubsubId: ${testAzurermWebPubsub.id}
///       webPubsubCustomCertificateId: ${test.id}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Custom Domain for a Web PubSub service can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:webpubsub/customDomain:CustomDomain example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.SignalRService/webPubSub/webpubsub1/customDomains/customDomain1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod custom_domain {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CustomDomainArgs {
        /// Specifies the custom domain name of the Web PubSub Custom Domain. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Please ensure the custom domain name is included in the Subject Alternative Names of the selected Web PubSub Custom Certificate.
        #[builder(into)]
        pub domain_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Web PubSub Custom Domain. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the Web PubSub Custom Certificate ID of the Web PubSub Custom Domain. Changing this forces a new resource to be created.
        #[builder(into)]
        pub web_pubsub_custom_certificate_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the Web PubSub ID of the Web PubSub Custom Domain. Changing this forces a new resource to be created.
        #[builder(into)]
        pub web_pubsub_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct CustomDomainResult {
        /// Specifies the custom domain name of the Web PubSub Custom Domain. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Please ensure the custom domain name is included in the Subject Alternative Names of the selected Web PubSub Custom Certificate.
        pub domain_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Web PubSub Custom Domain. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Web PubSub Custom Certificate ID of the Web PubSub Custom Domain. Changing this forces a new resource to be created.
        pub web_pubsub_custom_certificate_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Web PubSub ID of the Web PubSub Custom Domain. Changing this forces a new resource to be created.
        pub web_pubsub_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CustomDomainArgs,
    ) -> CustomDomainResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let domain_name_binding = args.domain_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let web_pubsub_custom_certificate_id_binding = args
            .web_pubsub_custom_certificate_id
            .get_output(context);
        let web_pubsub_id_binding = args.web_pubsub_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:webpubsub/customDomain:CustomDomain".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "webPubsubCustomCertificateId".into(),
                    value: &web_pubsub_custom_certificate_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "webPubsubId".into(),
                    value: &web_pubsub_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        CustomDomainResult {
            domain_name: o.get_field("domainName"),
            name: o.get_field("name"),
            web_pubsub_custom_certificate_id: o
                .get_field("webPubsubCustomCertificateId"),
            web_pubsub_id: o.get_field("webPubsubId"),
        }
    }
}
