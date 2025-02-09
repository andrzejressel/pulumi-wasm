/// Manages an App Service certificate.
///
/// ## Example Usage
///
/// This example provisions an App Service Certificate from a Local File.
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleCertificate:
///     type: azure:appservice:Certificate
///     name: example
///     properties:
///       name: example-cert
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       pfxBlob:
///         fn::invoke:
///           function: std:filebase64
///           arguments:
///             input: certificate.pfx
///           return: result
///       password: password123!
/// ```
///
/// ## Import
///
/// App Service Certificates can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appservice/certificate:Certificate example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Web/certificates/certificate1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod certificate {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CertificateArgs {
        /// The ID of the associated App Service plan. Must be specified when the certificate is used inside an App Service Environment hosted App Service or with Premium App Service plans. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub app_service_plan_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub key_vault_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Key Vault secret. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Exactly one of `key_vault_secret_id` or `pfx_blob` must be specified.
        #[builder(into, default)]
        pub key_vault_secret_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the certificate. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The password to access the certificate's private key. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub password: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The base64-encoded contents of the certificate. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Exactly one of `key_vault_secret_id` or `pfx_blob` must be specified.
        #[builder(into, default)]
        pub pfx_blob: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which to create the certificate. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The resource group must be the same as that which the app service plan is defined in - otherwise the certificate will not show as available for the app services.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct CertificateResult {
        /// The ID of the associated App Service plan. Must be specified when the certificate is used inside an App Service Environment hosted App Service or with Premium App Service plans. Changing this forces a new resource to be created.
        pub app_service_plan_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The expiration date for the certificate.
        pub expiration_date: pulumi_gestalt_rust::Output<String>,
        /// The friendly name of the certificate.
        pub friendly_name: pulumi_gestalt_rust::Output<String>,
        /// List of host names the certificate applies to.
        pub host_names: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The ID of the App Service Environment where the certificate is in use.
        pub hosting_environment_profile_id: pulumi_gestalt_rust::Output<String>,
        /// The issue date for the certificate.
        pub issue_date: pulumi_gestalt_rust::Output<String>,
        /// The name of the certificate issuer.
        pub issuer: pulumi_gestalt_rust::Output<String>,
        pub key_vault_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the Key Vault secret. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Exactly one of `key_vault_secret_id` or `pfx_blob` must be specified.
        pub key_vault_secret_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the certificate. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The password to access the certificate's private key. Changing this forces a new resource to be created.
        pub password: pulumi_gestalt_rust::Output<Option<String>>,
        /// The base64-encoded contents of the certificate. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Exactly one of `key_vault_secret_id` or `pfx_blob` must be specified.
        pub pfx_blob: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the certificate. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The resource group must be the same as that which the app service plan is defined in - otherwise the certificate will not show as available for the app services.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The subject name of the certificate.
        pub subject_name: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The thumbprint for the certificate.
        pub thumbprint: pulumi_gestalt_rust::Output<String>,
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
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let app_service_plan_id_binding = args.app_service_plan_id.get_output(context);
        let key_vault_id_binding = args.key_vault_id.get_output(context);
        let key_vault_secret_id_binding = args.key_vault_secret_id.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let password_binding = args.password.get_output(context);
        let pfx_blob_binding = args.pfx_blob.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:appservice/certificate:Certificate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "appServicePlanId".into(),
                    value: app_service_plan_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyVaultId".into(),
                    value: key_vault_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyVaultSecretId".into(),
                    value: key_vault_secret_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "password".into(),
                    value: password_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pfxBlob".into(),
                    value: pfx_blob_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        CertificateResult {
            app_service_plan_id: o.get_field("appServicePlanId"),
            expiration_date: o.get_field("expirationDate"),
            friendly_name: o.get_field("friendlyName"),
            host_names: o.get_field("hostNames"),
            hosting_environment_profile_id: o.get_field("hostingEnvironmentProfileId"),
            issue_date: o.get_field("issueDate"),
            issuer: o.get_field("issuer"),
            key_vault_id: o.get_field("keyVaultId"),
            key_vault_secret_id: o.get_field("keyVaultSecretId"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            password: o.get_field("password"),
            pfx_blob: o.get_field("pfxBlob"),
            resource_group_name: o.get_field("resourceGroupName"),
            subject_name: o.get_field("subjectName"),
            tags: o.get_field("tags"),
            thumbprint: o.get_field("thumbprint"),
        }
    }
}
