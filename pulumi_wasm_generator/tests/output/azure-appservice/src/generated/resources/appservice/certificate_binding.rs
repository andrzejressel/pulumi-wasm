/// Manages an App Service Certificate Binding.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: webapp
///       location: West Europe
///   examplePlan:
///     type: azure:appservice:Plan
///     name: example
///     properties:
///       name: appserviceplan
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       sku:
///         tier: Premium
///         size: P1
///   exampleAppService:
///     type: azure:appservice:AppService
///     name: example
///     properties:
///       name: mywebapp
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       appServicePlanId: ${examplePlan.id}
///   exampleCNameRecord:
///     type: azure:dns:CNameRecord
///     name: example
///     properties:
///       name: www
///       zoneName: ${example.name}
///       resourceGroupName: ${example.resourceGroupName}
///       ttl: 300
///       record: ${exampleAppService.defaultSiteHostname}
///   exampleTxtRecord:
///     type: azure:dns:TxtRecord
///     name: example
///     properties:
///       name: asuid.${exampleCNameRecord.name}
///       zoneName: ${example.name}
///       resourceGroupName: ${example.resourceGroupName}
///       ttl: 300
///       records:
///         - value: ${exampleAppService.customDomainVerificationId}
///   exampleCustomHostnameBinding:
///     type: azure:appservice:CustomHostnameBinding
///     name: example
///     properties:
///       hostname:
///         fn::invoke:
///           function: std:trim
///           arguments:
///             input: ${exampleCNameRecord.fqdn}
///             cutset: .
///           return: result
///       appServiceName: ${exampleAppService.name}
///       resourceGroupName: ${exampleResourceGroup.name}
///     options:
///       dependsOn:
///         - ${exampleTxtRecord}
///   exampleManagedCertificate:
///     type: azure:appservice:ManagedCertificate
///     name: example
///     properties:
///       customHostnameBindingId: ${exampleCustomHostnameBinding.id}
///   exampleCertificateBinding:
///     type: azure:appservice:CertificateBinding
///     name: example
///     properties:
///       hostnameBindingId: ${exampleCustomHostnameBinding.id}
///       certificateId: ${exampleManagedCertificate.id}
///       sslState: SniEnabled
/// variables:
///   example:
///     fn::invoke:
///       function: azure:dns:getZone
///       arguments:
///         name: example.com
///         resourceGroupName: ${exampleResourceGroup.name}
/// ```
///
/// ## Import
///
/// App Service Certificate Bindings can be imported using the `hostname_binding_id` and the `app_service_certificate_id` , e.g.
///
/// ```sh
/// $ pulumi import azure:appservice/certificateBinding:CertificateBinding example "/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Web/sites/instance1/hostNameBindings/mywebsite.com|/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Web/certificates/mywebsite.com"
/// ```
///
pub mod certificate_binding {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CertificateBindingArgs {
        /// The ID of the certificate to bind to the custom domain. Changing this forces a new App Service Certificate Binding to be created.
        #[builder(into)]
        pub certificate_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the Custom Domain/Hostname Binding. Changing this forces a new App Service Certificate Binding to be created.
        #[builder(into)]
        pub hostname_binding_id: pulumi_wasm_rust::Output<String>,
        /// The type of certificate binding. Allowed values are `IpBasedEnabled` or `SniEnabled`. Changing this forces a new App Service Certificate Binding to be created.
        #[builder(into)]
        pub ssl_state: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct CertificateBindingResult {
        /// The name of the App Service to which the certificate was bound.
        pub app_service_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the certificate to bind to the custom domain. Changing this forces a new App Service Certificate Binding to be created.
        pub certificate_id: pulumi_wasm_rust::Output<String>,
        /// The hostname of the bound certificate.
        pub hostname: pulumi_wasm_rust::Output<String>,
        /// The ID of the Custom Domain/Hostname Binding. Changing this forces a new App Service Certificate Binding to be created.
        pub hostname_binding_id: pulumi_wasm_rust::Output<String>,
        /// The type of certificate binding. Allowed values are `IpBasedEnabled` or `SniEnabled`. Changing this forces a new App Service Certificate Binding to be created.
        pub ssl_state: pulumi_wasm_rust::Output<String>,
        /// The certificate thumbprint.
        pub thumbprint: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: CertificateBindingArgs) -> CertificateBindingResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let certificate_id_binding = args.certificate_id.get_inner();
        let hostname_binding_id_binding = args.hostname_binding_id.get_inner();
        let ssl_state_binding = args.ssl_state.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appservice/certificateBinding:CertificateBinding".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "certificateId".into(),
                    value: &certificate_id_binding,
                },
                register_interface::ObjectField {
                    name: "hostnameBindingId".into(),
                    value: &hostname_binding_id_binding,
                },
                register_interface::ObjectField {
                    name: "sslState".into(),
                    value: &ssl_state_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "appServiceName".into(),
                },
                register_interface::ResultField {
                    name: "certificateId".into(),
                },
                register_interface::ResultField {
                    name: "hostname".into(),
                },
                register_interface::ResultField {
                    name: "hostnameBindingId".into(),
                },
                register_interface::ResultField {
                    name: "sslState".into(),
                },
                register_interface::ResultField {
                    name: "thumbprint".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CertificateBindingResult {
            app_service_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appServiceName").unwrap(),
            ),
            certificate_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateId").unwrap(),
            ),
            hostname: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostname").unwrap(),
            ),
            hostname_binding_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostnameBindingId").unwrap(),
            ),
            ssl_state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sslState").unwrap(),
            ),
            thumbprint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("thumbprint").unwrap(),
            ),
        }
    }
}