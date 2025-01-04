/// Manages an IoT Central Application
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resource
///       location: West Europe
///   exampleApplication:
///     type: azure:iotcentral:Application
///     name: example
///     properties:
///       name: example-iotcentral-app
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       subDomain: example-iotcentral-app-subdomain
///       displayName: example-iotcentral-app-display-name
///       sku: ST1
///       template: iotc-default@1.0.0
///       tags:
///         Foo: Bar
/// ```
///
/// ## Import
///
/// The IoT Central Application can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:iotcentral/application:Application example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.IoTCentral/iotApps/app1
/// ```
///
pub mod application {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApplicationArgs {
        /// A `display_name` name. Custom display name for the IoT Central application. Default is resource name.
        ///
        /// > **NOTE:** Due to a bug in the provider, the default value of `display_name` of a newly created IoT Central App will be the Resource Group Name, it will be fixed and use resource name in 4.0. For an existing IoT Central App, this could be fixed by specifying the `display_name` explicitly.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::iotcentral::ApplicationIdentity>,
        >,
        /// Specifies the supported Azure location where the resource has to be create. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the IotHub resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether public network access is allowed for the IoT Central Application. Defaults to `true`.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the resource group under which the IotHub resource has to be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `sku` name. Possible values is `ST0`, `ST1`, `ST2`, Default value is `ST1`
        #[builder(into, default)]
        pub sku: pulumi_wasm_rust::Output<Option<String>>,
        /// A `sub_domain` name. Subdomain for the IoT Central URL. Each application must have a unique subdomain.
        #[builder(into)]
        pub sub_domain: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `template` name. IoT Central application template name. Defaults to `iotc-pnp-preview@1.0.0`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub template: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ApplicationResult {
        /// A `display_name` name. Custom display name for the IoT Central application. Default is resource name.
        ///
        /// > **NOTE:** Due to a bug in the provider, the default value of `display_name` of a newly created IoT Central App will be the Resource Group Name, it will be fixed and use resource name in 4.0. For an existing IoT Central App, this could be fixed by specifying the `display_name` explicitly.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::iotcentral::ApplicationIdentity>,
        >,
        /// Specifies the supported Azure location where the resource has to be create. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the IotHub resource. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Whether public network access is allowed for the IoT Central Application. Defaults to `true`.
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the resource group under which the IotHub resource has to be created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `sku` name. Possible values is `ST0`, `ST1`, `ST2`, Default value is `ST1`
        pub sku: pulumi_wasm_rust::Output<Option<String>>,
        /// A `sub_domain` name. Subdomain for the IoT Central URL. Each application must have a unique subdomain.
        pub sub_domain: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `template` name. IoT Central application template name. Defaults to `iotc-pnp-preview@1.0.0`. Changing this forces a new resource to be created.
        pub template: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ApplicationArgs) -> ApplicationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let display_name_binding = args.display_name.get_inner();
        let identity_binding = args.identity.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let sku_binding = args.sku.get_inner();
        let sub_domain_binding = args.sub_domain.get_inner();
        let tags_binding = args.tags.get_inner();
        let template_binding = args.template.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:iotcentral/application:Application".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "publicNetworkAccessEnabled".into(),
                    value: &public_network_access_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "sku".into(),
                    value: &sku_binding,
                },
                register_interface::ObjectField {
                    name: "subDomain".into(),
                    value: &sub_domain_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "template".into(),
                    value: &template_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "publicNetworkAccessEnabled".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "sku".into(),
                },
                register_interface::ResultField {
                    name: "subDomain".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "template".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ApplicationResult {
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            public_network_access_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicNetworkAccessEnabled").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            sku: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sku").unwrap(),
            ),
            sub_domain: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subDomain").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            template: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("template").unwrap(),
            ),
        }
    }
}
