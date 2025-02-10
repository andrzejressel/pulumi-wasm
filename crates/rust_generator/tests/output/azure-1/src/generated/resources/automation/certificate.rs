/// Manages an Automation Certificate.
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
///   exampleAccount:
///     type: azure:automation:Account
///     name: example
///     properties:
///       name: account1
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       skuName: Basic
///   exampleCertificate:
///     type: azure:automation:Certificate
///     name: example
///     properties:
///       name: certificate1
///       resourceGroupName: ${example.name}
///       automationAccountName: ${exampleAccount.name}
///       description: This is an example certificate
///       base64:
///         fn::invoke:
///           function: std:filebase64
///           arguments:
///             input: certificate.pfx
///           return: result
///       exportable: true
/// ```
///
/// ## Import
///
/// Automation Certificates can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:automation/certificate:Certificate certificate1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Automation/automationAccounts/account1/certificates/certificate1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod certificate {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CertificateArgs {
        /// The name of the automation account in which the Certificate is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub automation_account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Base64 encoded value of the certificate. Changing this forces a new resource to be created.
        #[builder(into)]
        pub base64: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The description of this Automation Certificate.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The is exportable flag of the certificate.
        #[builder(into, default)]
        pub exportable: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the name of the Certificate. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which the Certificate is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct CertificateResult {
        /// The name of the automation account in which the Certificate is created. Changing this forces a new resource to be created.
        pub automation_account_name: pulumi_gestalt_rust::Output<String>,
        /// Base64 encoded value of the certificate. Changing this forces a new resource to be created.
        pub base64: pulumi_gestalt_rust::Output<String>,
        /// The description of this Automation Certificate.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The is exportable flag of the certificate.
        pub exportable: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the name of the Certificate. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which the Certificate is created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
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
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let automation_account_name_binding = args
            .automation_account_name
            .get_output(context);
        let base64_binding = args.base64.get_output(context);
        let description_binding = args.description.get_output(context);
        let exportable_binding = args.exportable.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:automation/certificate:Certificate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "automationAccountName".into(),
                    value: automation_account_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "base64".into(),
                    value: base64_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "exportable".into(),
                    value: exportable_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        CertificateResult {
            automation_account_name: o.get_field("automationAccountName"),
            base64: o.get_field("base64"),
            description: o.get_field("description"),
            exportable: o.get_field("exportable"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            thumbprint: o.get_field("thumbprint"),
        }
    }
}
