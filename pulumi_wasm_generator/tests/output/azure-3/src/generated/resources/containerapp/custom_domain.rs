/// Manages a Container App Custom Domain.
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
///   exampleZone:
///     type: azure:dns:Zone
///     name: example
///     properties:
///       name: contoso.com
///       resourceGroupName: ${example.name}
///   exampleTxtRecord:
///     type: azure:dns:TxtRecord
///     name: example
///     properties:
///       name: asuid.example
///       resourceGroupName: ${exampleZone.resourceGroupName}
///       zoneName: ${exampleZone.name}
///       ttl: 300
///       records:
///         - value: ${exampleApp.customDomainVerificationId}
///   exampleAnalyticsWorkspace:
///     type: azure:operationalinsights:AnalyticsWorkspace
///     name: example
///     properties:
///       name: example
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku: PerGB2018
///       retentionInDays: 30
///   exampleEnvironment:
///     type: azure:containerapp:Environment
///     name: example
///     properties:
///       name: Example-Environment
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       logAnalyticsWorkspaceId: ${exampleAnalyticsWorkspace.id}
///   exampleEnvironmentCertificate:
///     type: azure:containerapp:EnvironmentCertificate
///     name: example
///     properties:
///       name: myfriendlyname
///       containerAppEnvironmentId: ${exampleEnvironment.id}
///       certificateBlob:
///         fn::invoke:
///           function: std:filebase64
///           arguments:
///             input: path/to/certificate_file.pfx
///           return: result
///       certificatePassword: $3cretSqu1rreL
///   exampleApp:
///     type: azure:containerapp:App
///     name: example
///     properties:
///       name: example-app
///       containerAppEnvironmentId: ${exampleEnvironment.id}
///       resourceGroupName: ${example.name}
///       revisionMode: Single
///       template:
///         containers:
///           - name: examplecontainerapp
///             image: mcr.microsoft.com/k8se/quickstart:latest
///             cpu: 0.25
///             memory: 0.5Gi
///       ingress:
///         allowInsecureConnections: false
///         externalEnabled: true
///         targetPort: 5000
///         transport: http
///         trafficWeights:
///           - latestRevision: true
///             percentage: 100
///   exampleCustomDomain:
///     type: azure:containerapp:CustomDomain
///     name: example
///     properties:
///       name:
///         fn::invoke:
///           function: std:trimsuffix
///           arguments:
///             input:
///               fn::invoke:
///                 function: std:trimprefix
///                 arguments:
///                   input: ${api.fqdn}
///                   prefix: asuid.
///                 return: result
///             suffix: .
///           return: result
///       containerAppId: ${exampleApp.id}
///       containerAppEnvironmentCertificateId: ${exampleEnvironmentCertificate.id}
///       certificateBindingType: SniEnabled
/// ```
///
/// ### Managed Certificate
///
/// ```yaml
/// resources:
///   example:
///     type: azure:containerapp:CustomDomain
///     properties:
///       name:
///         fn::invoke:
///           function: std:trimsuffix
///           arguments:
///             input:
///               fn::invoke:
///                 function: std:trimprefix
///                 arguments:
///                   input: ${api.fqdn}
///                   prefix: asuid.
///                 return: result
///             suffix: .
///           return: result
///       containerAppId: ${exampleAzurermContainerApp.id}
/// ```
///
/// ## Import
///
/// A Container App Custom Domain can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:containerapp/customDomain:CustomDomain example "/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resGroup1/providers/Microsoft.App/containerApps/myContainerApp/customDomainName/mycustomdomain.example.com"
/// ```
///
pub mod custom_domain {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CustomDomainArgs {
        /// The Binding type. Possible values include `Disabled` and `SniEnabled`.
        #[builder(into, default)]
        pub certificate_binding_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Container App Environment Certificate to use. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Omit this value if you wish to use an Azure Managed certificate. You must create the relevant DNS verification steps before this process will be successful.
        #[builder(into, default)]
        pub container_app_environment_certificate_id: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// The ID of the Container App to which this Custom Domain should be bound. Changing this forces a new resource to be created.
        #[builder(into)]
        pub container_app_id: pulumi_wasm_rust::Output<String>,
        /// The fully qualified name of the Custom Domain. Must be the CN or a named SAN in the certificate specified by the `container_app_environment_certificate_id`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** The Custom Domain verification TXT record requires a prefix of `asuid.`, however, this must be trimmed from the `name` property here. See the [official docs](https://learn.microsoft.com/en-us/azure/container-apps/custom-domains-certificates) for more information.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct CustomDomainResult {
        /// The Binding type. Possible values include `Disabled` and `SniEnabled`.
        pub certificate_binding_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Container App Environment Certificate to use. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Omit this value if you wish to use an Azure Managed certificate. You must create the relevant DNS verification steps before this process will be successful.
        pub container_app_environment_certificate_id: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// The ID of the Container App Environment Managed Certificate to use.
        pub container_app_environment_managed_certificate_id: pulumi_wasm_rust::Output<
            String,
        >,
        /// The ID of the Container App to which this Custom Domain should be bound. Changing this forces a new resource to be created.
        pub container_app_id: pulumi_wasm_rust::Output<String>,
        /// The fully qualified name of the Custom Domain. Must be the CN or a named SAN in the certificate specified by the `container_app_environment_certificate_id`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** The Custom Domain verification TXT record requires a prefix of `asuid.`, however, this must be trimmed from the `name` property here. See the [official docs](https://learn.microsoft.com/en-us/azure/container-apps/custom-domains-certificates) for more information.
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: CustomDomainArgs) -> CustomDomainResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let certificate_binding_type_binding = args.certificate_binding_type.get_inner();
        let container_app_environment_certificate_id_binding = args
            .container_app_environment_certificate_id
            .get_inner();
        let container_app_id_binding = args.container_app_id.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:containerapp/customDomain:CustomDomain".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "certificateBindingType".into(),
                    value: &certificate_binding_type_binding,
                },
                register_interface::ObjectField {
                    name: "containerAppEnvironmentCertificateId".into(),
                    value: &container_app_environment_certificate_id_binding,
                },
                register_interface::ObjectField {
                    name: "containerAppId".into(),
                    value: &container_app_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "certificateBindingType".into(),
                },
                register_interface::ResultField {
                    name: "containerAppEnvironmentCertificateId".into(),
                },
                register_interface::ResultField {
                    name: "containerAppEnvironmentManagedCertificateId".into(),
                },
                register_interface::ResultField {
                    name: "containerAppId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CustomDomainResult {
            certificate_binding_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateBindingType").unwrap(),
            ),
            container_app_environment_certificate_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("containerAppEnvironmentCertificateId").unwrap(),
            ),
            container_app_environment_managed_certificate_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("containerAppEnvironmentManagedCertificateId").unwrap(),
            ),
            container_app_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("containerAppId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
        }
    }
}
