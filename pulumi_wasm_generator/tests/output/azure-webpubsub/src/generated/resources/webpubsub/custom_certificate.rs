/// Manages an Azure Web PubSub Custom Certificate.
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
///   exampleWebPubsubService:
///     type: azurerm:webPubsubService
///     name: example
///     properties:
///       name: example-webpubsub
///       location: ${testAzurermResourceGroup.location}
///       resourceGroupName: ${testAzurermResourceGroup.name}
///       sku:
///         - name: Premium_P1
///           capacity: 1
///       identity:
///         - type: SystemAssigned
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
///           objectId: ${testAzurermWebPubsubService.identity[0].principalId}
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
///       webPubsubId: ${exampleWebPubsubService.id}
///       customCertificateId: ${exampleCertificate.id}
///     options:
///       dependsOn:
///         - ${exampleAzurermKeyVaultAccessPolicy}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Custom Certificate for a Web PubSub service can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:webpubsub/customCertificate:CustomCertificate example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.SignalRService/webPubSub/WebPubsub1/customCertificates/cert1
/// ```
///
pub mod custom_certificate {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CustomCertificateArgs {
        /// The certificate ID of the Web PubSub Custom Certificate. Changing this forces a new resource to be created.
        ///
        /// > **Note:** Self assigned certificate is not supported and the provisioning status will fail.
        #[builder(into)]
        pub custom_certificate_id: pulumi_wasm_rust::Output<String>,
        /// The name of the Web PubSub Custom Certificate. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The Web PubSub ID of the Web PubSub Custom Certificate. Changing this forces a new resource to be created.
        ///
        /// > **Note:** custom certificate is only available for Web PubSub Premium tier. Please enable managed identity in the corresponding Web PubSub Service and give the managed identity access to the key vault, the required permission is Get Certificate and Secret.
        #[builder(into)]
        pub web_pubsub_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct CustomCertificateResult {
        /// The certificate version of the Web PubSub Custom Certificate.
        pub certificate_version: pulumi_wasm_rust::Output<String>,
        /// The certificate ID of the Web PubSub Custom Certificate. Changing this forces a new resource to be created.
        ///
        /// > **Note:** Self assigned certificate is not supported and the provisioning status will fail.
        pub custom_certificate_id: pulumi_wasm_rust::Output<String>,
        /// The name of the Web PubSub Custom Certificate. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The Web PubSub ID of the Web PubSub Custom Certificate. Changing this forces a new resource to be created.
        ///
        /// > **Note:** custom certificate is only available for Web PubSub Premium tier. Please enable managed identity in the corresponding Web PubSub Service and give the managed identity access to the key vault, the required permission is Get Certificate and Secret.
        pub web_pubsub_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: CustomCertificateArgs) -> CustomCertificateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let custom_certificate_id_binding = args.custom_certificate_id.get_inner();
        let name_binding = args.name.get_inner();
        let web_pubsub_id_binding = args.web_pubsub_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:webpubsub/customCertificate:CustomCertificate".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "customCertificateId".into(),
                    value: &custom_certificate_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "webPubsubId".into(),
                    value: &web_pubsub_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "certificateVersion".into(),
                },
                register_interface::ResultField {
                    name: "customCertificateId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "webPubsubId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CustomCertificateResult {
            certificate_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateVersion").unwrap(),
            ),
            custom_certificate_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customCertificateId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            web_pubsub_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("webPubsubId").unwrap(),
            ),
        }
    }
}