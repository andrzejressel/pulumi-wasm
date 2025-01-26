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
pub mod certificate {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CertificateArgs {
        /// The name of the automation account in which the Certificate is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub automation_account_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Base64 encoded value of the certificate. Changing this forces a new resource to be created.
        #[builder(into)]
        pub base64: pulumi_wasm_rust::InputOrOutput<String>,
        /// The description of this Automation Certificate.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The is exportable flag of the certificate.
        #[builder(into, default)]
        pub exportable: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Specifies the name of the Certificate. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which the Certificate is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct CertificateResult {
        /// The name of the automation account in which the Certificate is created. Changing this forces a new resource to be created.
        pub automation_account_name: pulumi_wasm_rust::Output<String>,
        /// Base64 encoded value of the certificate. Changing this forces a new resource to be created.
        pub base64: pulumi_wasm_rust::Output<String>,
        /// The description of this Automation Certificate.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The is exportable flag of the certificate.
        pub exportable: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the name of the Certificate. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which the Certificate is created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The thumbprint for the certificate.
        pub thumbprint: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: CertificateArgs,
    ) -> CertificateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let automation_account_name_binding = args
            .automation_account_name
            .get_output(context)
            .get_inner();
        let base64_binding = args.base64.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let exportable_binding = args.exportable.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:automation/certificate:Certificate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "automationAccountName".into(),
                    value: &automation_account_name_binding,
                },
                register_interface::ObjectField {
                    name: "base64".into(),
                    value: &base64_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "exportable".into(),
                    value: &exportable_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        CertificateResult {
            automation_account_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("automationAccountName"),
            ),
            base64: pulumi_wasm_rust::__private::into_domain(o.extract_field("base64")),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            exportable: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("exportable"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            thumbprint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("thumbprint"),
            ),
        }
    }
}
