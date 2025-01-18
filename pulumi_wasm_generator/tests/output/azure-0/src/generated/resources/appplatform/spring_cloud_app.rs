/// Manage an Azure Spring Cloud Application.
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
///   exampleSpringCloudService:
///     type: azure:appplatform:SpringCloudService
///     name: example
///     properties:
///       name: example-springcloud
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///   exampleSpringCloudApp:
///     type: azure:appplatform:SpringCloudApp
///     name: example
///     properties:
///       name: example-springcloudapp
///       resourceGroupName: ${example.name}
///       serviceName: ${exampleSpringCloudService.name}
///       identity:
///         type: SystemAssigned
/// ```
///
/// ## Import
///
/// Spring Cloud Application can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appplatform/springCloudApp:SpringCloudApp example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/myresourcegroup/providers/Microsoft.AppPlatform/spring/myservice/apps/myapp
/// ```
///
pub mod spring_cloud_app {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpringCloudAppArgs {
        /// A JSON object that contains the addon configurations of the Spring Cloud Service.
        #[builder(into, default)]
        pub addon_json: pulumi_wasm_rust::Output<Option<String>>,
        /// A `custom_persistent_disk` block as defined below.
        #[builder(into, default)]
        pub custom_persistent_disks: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::appplatform::SpringCloudAppCustomPersistentDisk>,
            >,
        >,
        /// Is only HTTPS allowed? Defaults to `false`.
        #[builder(into, default)]
        pub https_only: pulumi_wasm_rust::Output<Option<bool>>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::appplatform::SpringCloudAppIdentity>,
        >,
        /// An `ingress_settings` block as defined below.
        #[builder(into, default)]
        pub ingress_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::appplatform::SpringCloudAppIngressSettings>,
        >,
        /// Does the Spring Cloud Application have public endpoint? Defaults to `false`.
        #[builder(into, default)]
        pub is_public: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the name of the Spring Cloud Application. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// An `persistent_disk` block as defined below.
        #[builder(into, default)]
        pub persistent_disk: pulumi_wasm_rust::Output<
            Option<super::super::types::appplatform::SpringCloudAppPersistentDisk>,
        >,
        /// Should the App in vnet injection instance exposes endpoint which could be accessed from Internet?
        #[builder(into, default)]
        pub public_endpoint_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the name of the resource group in which to create the Spring Cloud Application. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Spring Cloud Service resource. Changing this forces a new resource to be created.
        #[builder(into)]
        pub service_name: pulumi_wasm_rust::Output<String>,
        /// Is End to End TLS Enabled? Defaults to `false`.
        #[builder(into, default)]
        pub tls_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct SpringCloudAppResult {
        /// A JSON object that contains the addon configurations of the Spring Cloud Service.
        pub addon_json: pulumi_wasm_rust::Output<String>,
        /// A `custom_persistent_disk` block as defined below.
        pub custom_persistent_disks: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::appplatform::SpringCloudAppCustomPersistentDisk>,
            >,
        >,
        /// The Fully Qualified DNS Name of the Spring Application in the service.
        pub fqdn: pulumi_wasm_rust::Output<String>,
        /// Is only HTTPS allowed? Defaults to `false`.
        pub https_only: pulumi_wasm_rust::Output<Option<bool>>,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::appplatform::SpringCloudAppIdentity>,
        >,
        /// An `ingress_settings` block as defined below.
        pub ingress_settings: pulumi_wasm_rust::Output<
            super::super::types::appplatform::SpringCloudAppIngressSettings,
        >,
        /// Does the Spring Cloud Application have public endpoint? Defaults to `false`.
        pub is_public: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the name of the Spring Cloud Application. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// An `persistent_disk` block as defined below.
        pub persistent_disk: pulumi_wasm_rust::Output<
            super::super::types::appplatform::SpringCloudAppPersistentDisk,
        >,
        /// Should the App in vnet injection instance exposes endpoint which could be accessed from Internet?
        pub public_endpoint_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the name of the resource group in which to create the Spring Cloud Application. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Spring Cloud Service resource. Changing this forces a new resource to be created.
        pub service_name: pulumi_wasm_rust::Output<String>,
        /// Is End to End TLS Enabled? Defaults to `false`.
        pub tls_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The public endpoint of the Spring Cloud Application.
        pub url: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: SpringCloudAppArgs) -> SpringCloudAppResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let addon_json_binding = args.addon_json.get_inner();
        let custom_persistent_disks_binding = args.custom_persistent_disks.get_inner();
        let https_only_binding = args.https_only.get_inner();
        let identity_binding = args.identity.get_inner();
        let ingress_settings_binding = args.ingress_settings.get_inner();
        let is_public_binding = args.is_public.get_inner();
        let name_binding = args.name.get_inner();
        let persistent_disk_binding = args.persistent_disk.get_inner();
        let public_endpoint_enabled_binding = args.public_endpoint_enabled.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let service_name_binding = args.service_name.get_inner();
        let tls_enabled_binding = args.tls_enabled.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appplatform/springCloudApp:SpringCloudApp".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "addonJson".into(),
                    value: &addon_json_binding,
                },
                register_interface::ObjectField {
                    name: "customPersistentDisks".into(),
                    value: &custom_persistent_disks_binding,
                },
                register_interface::ObjectField {
                    name: "httpsOnly".into(),
                    value: &https_only_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "ingressSettings".into(),
                    value: &ingress_settings_binding,
                },
                register_interface::ObjectField {
                    name: "isPublic".into(),
                    value: &is_public_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "persistentDisk".into(),
                    value: &persistent_disk_binding,
                },
                register_interface::ObjectField {
                    name: "publicEndpointEnabled".into(),
                    value: &public_endpoint_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "serviceName".into(),
                    value: &service_name_binding,
                },
                register_interface::ObjectField {
                    name: "tlsEnabled".into(),
                    value: &tls_enabled_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "addonJson".into(),
                },
                register_interface::ResultField {
                    name: "customPersistentDisks".into(),
                },
                register_interface::ResultField {
                    name: "fqdn".into(),
                },
                register_interface::ResultField {
                    name: "httpsOnly".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "ingressSettings".into(),
                },
                register_interface::ResultField {
                    name: "isPublic".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "persistentDisk".into(),
                },
                register_interface::ResultField {
                    name: "publicEndpointEnabled".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "serviceName".into(),
                },
                register_interface::ResultField {
                    name: "tlsEnabled".into(),
                },
                register_interface::ResultField {
                    name: "url".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SpringCloudAppResult {
            addon_json: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("addonJson").unwrap(),
            ),
            custom_persistent_disks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customPersistentDisks").unwrap(),
            ),
            fqdn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fqdn").unwrap(),
            ),
            https_only: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("httpsOnly").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            ingress_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ingressSettings").unwrap(),
            ),
            is_public: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("isPublic").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            persistent_disk: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("persistentDisk").unwrap(),
            ),
            public_endpoint_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicEndpointEnabled").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            service_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceName").unwrap(),
            ),
            tls_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tlsEnabled").unwrap(),
            ),
            url: pulumi_wasm_rust::__private::into_domain(hashmap.remove("url").unwrap()),
        }
    }
}
