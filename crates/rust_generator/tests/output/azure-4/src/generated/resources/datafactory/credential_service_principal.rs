/// Manage a Data Factory Service Principal credential resource. These resources are used by Data Factory to access data sources.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: westeurope
///   exampleFactory:
///     type: azure:datafactory:Factory
///     name: example
///     properties:
///       name: example
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleKeyVault:
///     type: azure:keyvault:KeyVault
///     name: example
///     properties:
///       name: example
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       tenantId: ${current.tenantId}
///       skuName: premium
///       softDeleteRetentionDays: 7
///       accessPolicies:
///         - tenantId: ${current.tenantId}
///           objectId: ${current.objectId}
///           keyPermissions:
///             - Create
///             - Get
///           secretPermissions:
///             - Set
///             - Get
///             - Delete
///             - Purge
///             - Recover
///   exampleSecret:
///     type: azure:keyvault:Secret
///     name: example
///     properties:
///       name: example
///       value: example-secret
///       keyVaultId: ${exampleKeyVault.id}
///   exampleLinkedServiceKeyVault:
///     type: azure:datafactory:LinkedServiceKeyVault
///     name: example
///     properties:
///       name: example
///       dataFactoryId: ${exampleFactory.id}
///       keyVaultId: ${exampleKeyVault.id}
///   exampleCredentialServicePrincipal:
///     type: azure:datafactory:CredentialServicePrincipal
///     name: example
///     properties:
///       name: example
///       description: example description
///       dataFactoryId: ${exampleFactory.id}
///       tenantId: ${current.tenantId}
///       servicePrincipalId: ${current.clientId}
///       servicePrincipalKey:
///         linkedServiceName: ${exampleLinkedServiceKeyVault.name}
///         secretName: ${exampleSecret.name}
///         secretVersion: ${exampleSecret.version}
///       annotations:
///         - '1'
///         - '2'
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Data Factory Credentials can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datafactory/credentialServicePrincipal:CredentialServicePrincipal example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example-resources/providers/Microsoft.DataFactory/factories/example/credentials/credential1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod credential_service_principal {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CredentialServicePrincipalArgs {
        /// List of tags that can be used for describing the Data Factory Credential.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The Data Factory ID in which to associate the Credential with. Changing this forces a new resource.
        #[builder(into)]
        pub data_factory_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The description for the Data Factory Credential.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Credential. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Client ID of the Service Principal.
        #[builder(into)]
        pub service_principal_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `service_principal_key` block as defined below.
        #[builder(into, default)]
        pub service_principal_key: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::datafactory::CredentialServicePrincipalServicePrincipalKey,
            >,
        >,
        /// The Tenant ID of the Service Principal.
        #[builder(into)]
        pub tenant_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct CredentialServicePrincipalResult {
        /// List of tags that can be used for describing the Data Factory Credential.
        pub annotations: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The Data Factory ID in which to associate the Credential with. Changing this forces a new resource.
        pub data_factory_id: pulumi_gestalt_rust::Output<String>,
        /// The description for the Data Factory Credential.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the name of the Credential. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Client ID of the Service Principal.
        pub service_principal_id: pulumi_gestalt_rust::Output<String>,
        /// A `service_principal_key` block as defined below.
        pub service_principal_key: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::datafactory::CredentialServicePrincipalServicePrincipalKey,
            >,
        >,
        /// The Tenant ID of the Service Principal.
        pub tenant_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CredentialServicePrincipalArgs,
    ) -> CredentialServicePrincipalResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let annotations_binding = args.annotations.get_output(context);
        let data_factory_id_binding = args.data_factory_id.get_output(context);
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let service_principal_id_binding = args.service_principal_id.get_output(context);
        let service_principal_key_binding = args
            .service_principal_key
            .get_output(context);
        let tenant_id_binding = args.tenant_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:datafactory/credentialServicePrincipal:CredentialServicePrincipal"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "annotations".into(),
                    value: annotations_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataFactoryId".into(),
                    value: data_factory_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "servicePrincipalId".into(),
                    value: service_principal_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "servicePrincipalKey".into(),
                    value: service_principal_key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tenantId".into(),
                    value: tenant_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        CredentialServicePrincipalResult {
            annotations: o.get_field("annotations"),
            data_factory_id: o.get_field("dataFactoryId"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            service_principal_id: o.get_field("servicePrincipalId"),
            service_principal_key: o.get_field("servicePrincipalKey"),
            tenant_id: o.get_field("tenantId"),
        }
    }
}
