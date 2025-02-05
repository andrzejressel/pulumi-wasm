/// Manages a Container App Environment Custom Domain Suffix.
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
///   exampleAnalyticsWorkspace:
///     type: azure:operationalinsights:AnalyticsWorkspace
///     name: example
///     properties:
///       name: acctest-01
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku: PerGB2018
///       retentionInDays: 30
///   exampleEnvironment:
///     type: azure:containerapp:Environment
///     name: example
///     properties:
///       name: my-environment
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       logAnalyticsWorkspaceId: ${exampleAnalyticsWorkspace.id}
///   exampleEnvironmentCustomDomain:
///     type: azure:containerapp:EnvironmentCustomDomain
///     name: example
///     properties:
///       containerAppEnvironmentId: ${exampleEnvironment.id}
///       certificateBlobBase64:
///         fn::invoke:
///           function: std:filebase64
///           arguments:
///             input: testacc.pfx
///           return: result
///       certificatePassword: TestAcc
///       dnsSuffix: acceptancetest.contoso.com
/// ```
///
/// ## Import
///
/// A Container App Environment Custom Domain Suffix can be imported using the `resource id` of its parent container ontainer App Environment , e.g.
///
/// ```sh
/// $ pulumi import azure:containerapp/environmentCustomDomain:EnvironmentCustomDomain example "/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resGroup1/providers/Microsoft.App/managedEnvironments/myEnvironment"
/// ```
///
pub mod environment_custom_domain {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnvironmentCustomDomainArgs {
        /// The bundle of Private Key and Certificate for the Custom DNS Suffix as a base64 encoded PFX or PEM.
        #[builder(into)]
        pub certificate_blob_base64: pulumi_wasm_rust::InputOrOutput<String>,
        /// The password for the Certificate bundle.
        #[builder(into)]
        pub certificate_password: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the Container Apps Managed Environment. Changing this forces a new resource to be created.
        #[builder(into)]
        pub container_app_environment_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Custom DNS Suffix for the Container App Environment.
        #[builder(into)]
        pub dns_suffix: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct EnvironmentCustomDomainResult {
        /// The bundle of Private Key and Certificate for the Custom DNS Suffix as a base64 encoded PFX or PEM.
        pub certificate_blob_base64: pulumi_wasm_rust::Output<String>,
        /// The password for the Certificate bundle.
        pub certificate_password: pulumi_wasm_rust::Output<String>,
        /// The ID of the Container Apps Managed Environment. Changing this forces a new resource to be created.
        pub container_app_environment_id: pulumi_wasm_rust::Output<String>,
        /// Custom DNS Suffix for the Container App Environment.
        pub dns_suffix: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: EnvironmentCustomDomainArgs,
    ) -> EnvironmentCustomDomainResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let certificate_blob_base64_binding = args
            .certificate_blob_base64
            .get_output(context)
            .get_inner();
        let certificate_password_binding = args
            .certificate_password
            .get_output(context)
            .get_inner();
        let container_app_environment_id_binding = args
            .container_app_environment_id
            .get_output(context)
            .get_inner();
        let dns_suffix_binding = args.dns_suffix.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:containerapp/environmentCustomDomain:EnvironmentCustomDomain"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "certificateBlobBase64".into(),
                    value: &certificate_blob_base64_binding,
                },
                register_interface::ObjectField {
                    name: "certificatePassword".into(),
                    value: &certificate_password_binding,
                },
                register_interface::ObjectField {
                    name: "containerAppEnvironmentId".into(),
                    value: &container_app_environment_id_binding,
                },
                register_interface::ObjectField {
                    name: "dnsSuffix".into(),
                    value: &dns_suffix_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        EnvironmentCustomDomainResult {
            certificate_blob_base64: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("certificateBlobBase64"),
            ),
            certificate_password: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("certificatePassword"),
            ),
            container_app_environment_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("containerAppEnvironmentId"),
            ),
            dns_suffix: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dnsSuffix"),
            ),
        }
    }
}
