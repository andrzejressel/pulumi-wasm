/// Manages a Certificate for an NGINX Deployment.
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
///       sku: publicpreview_Monthly_gmz7xq9ge3py
///       location: ${example.location}
///       managedResourceGroup: example
///       diagnoseSupportEnabled: true
///       frontendPublic:
///         ipAddresses:
///           - ${examplePublicIp.id}
///       networkInterfaces:
///         - subnetId: ${exampleSubnet.id}
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
///             - Delete
///             - DeleteIssuers
///             - Get
///             - GetIssuers
///             - Import
///             - List
///             - ListIssuers
///             - ManageContacts
///             - ManageIssuers
///             - SetIssuers
///             - Update
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
///   exampleCertificate2:
///     type: azure:nginx:Certificate
///     name: example
///     properties:
///       name: examplecert
///       nginxDeploymentId: ${exampleDeployment.id}
///       keyVirtualPath: /src/cert/soservermekey.key
///       certificateVirtualPath: /src/cert/server.cert
///       keyVaultSecretId: ${exampleCertificate.secretId}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// An NGINX Certificate can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:nginx/certificate:Certificate example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Nginx.NginxPlus/nginxDeployments/deploy1/certificates/cer1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod certificate {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CertificateArgs {
        /// Specify the path to the certificate file of this certificate.
        #[builder(into)]
        pub certificate_virtual_path: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specify the ID of the Key Vault Secret for this certificate.
        #[builder(into)]
        pub key_vault_secret_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specify the path to the key file of this certificate.
        #[builder(into)]
        pub key_virtual_path: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this NGINX Certificate. Changing this forces a new NGINX Certificate to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the NGINX Deployment that this Certificate should be associated with. Changing this forces a new NGINX Certificate to be created.
        #[builder(into)]
        pub nginx_deployment_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct CertificateResult {
        /// Specify the path to the certificate file of this certificate.
        pub certificate_virtual_path: pulumi_gestalt_rust::Output<String>,
        /// Specify the ID of the Key Vault Secret for this certificate.
        pub key_vault_secret_id: pulumi_gestalt_rust::Output<String>,
        /// Specify the path to the key file of this certificate.
        pub key_virtual_path: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this NGINX Certificate. Changing this forces a new NGINX Certificate to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the NGINX Deployment that this Certificate should be associated with. Changing this forces a new NGINX Certificate to be created.
        pub nginx_deployment_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CertificateArgs,
    ) -> CertificateResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let certificate_virtual_path_binding = args
            .certificate_virtual_path
            .get_output(context);
        let key_vault_secret_id_binding = args.key_vault_secret_id.get_output(context);
        let key_virtual_path_binding = args.key_virtual_path.get_output(context);
        let name_binding = args.name.get_output(context);
        let nginx_deployment_id_binding = args.nginx_deployment_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:nginx/certificate:Certificate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificateVirtualPath".into(),
                    value: &certificate_virtual_path_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyVaultSecretId".into(),
                    value: &key_vault_secret_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyVirtualPath".into(),
                    value: &key_virtual_path_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nginxDeploymentId".into(),
                    value: &nginx_deployment_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        CertificateResult {
            certificate_virtual_path: o.get_field("certificateVirtualPath"),
            key_vault_secret_id: o.get_field("keyVaultSecretId"),
            key_virtual_path: o.get_field("keyVirtualPath"),
            name: o.get_field("name"),
            nginx_deployment_id: o.get_field("nginxDeploymentId"),
        }
    }
}
