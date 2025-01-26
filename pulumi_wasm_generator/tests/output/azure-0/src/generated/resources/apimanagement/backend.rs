/// Manages a backend within an API Management Service.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleBackend = backend::create(
///         "exampleBackend",
///         BackendArgs::builder()
///             .api_management_name("${exampleService.name}")
///             .name("example-backend")
///             .protocol("http")
///             .resource_group_name("${example.name}")
///             .url("https://backend.com/api")
///             .build_struct(),
///     );
///     let exampleService = service::create(
///         "exampleService",
///         ServiceArgs::builder()
///             .location("${example.location}")
///             .name("example-apim")
///             .publisher_email("company@exmaple.com")
///             .publisher_name("My Company")
///             .resource_group_name("${example.name}")
///             .sku_name("Developer_1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// API Management backends can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:apimanagement/backend:Backend example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.ApiManagement/service/instance1/backends/backend1
/// ```
///
pub mod backend {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackendArgs {
        /// The Name of the API Management Service where this backend should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_management_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A `credentials` block as documented below.
        #[builder(into, default)]
        pub credentials: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::apimanagement::BackendCredentials>,
        >,
        /// The description of the backend.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the API Management backend. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The protocol used by the backend host. Possible values are `http` or `soap`.
        #[builder(into)]
        pub protocol: pulumi_wasm_rust::InputOrOutput<String>,
        /// A `proxy` block as documented below.
        #[builder(into, default)]
        pub proxy: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::apimanagement::BackendProxy>,
        >,
        /// The Name of the Resource Group where the API Management Service exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The management URI of the backend host in an external system. This URI can be the ARM Resource ID of Logic Apps, Function Apps or API Apps, or the management endpoint of a Service Fabric cluster.
        #[builder(into, default)]
        pub resource_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A `service_fabric_cluster` block as documented below.
        #[builder(into, default)]
        pub service_fabric_cluster: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::apimanagement::BackendServiceFabricCluster>,
        >,
        /// The title of the backend.
        #[builder(into, default)]
        pub title: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A `tls` block as documented below.
        #[builder(into, default)]
        pub tls: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::apimanagement::BackendTls>,
        >,
        /// The backend host URL should be specified in the format `"https://backend.com/api"`, avoiding trailing slashes (/) to minimize misconfiguration risks. Azure API Management instance will append the backend resource name to this URL. This URL typically serves as the `base-url` in the [`set-backend-service`](https://learn.microsoft.com/azure/api-management/set-backend-service-policy) policy, enabling seamless transitions from frontend to backend.
        #[builder(into)]
        pub url: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct BackendResult {
        /// The Name of the API Management Service where this backend should be created. Changing this forces a new resource to be created.
        pub api_management_name: pulumi_wasm_rust::Output<String>,
        /// A `credentials` block as documented below.
        pub credentials: pulumi_wasm_rust::Output<
            Option<super::super::types::apimanagement::BackendCredentials>,
        >,
        /// The description of the backend.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the API Management backend. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The protocol used by the backend host. Possible values are `http` or `soap`.
        pub protocol: pulumi_wasm_rust::Output<String>,
        /// A `proxy` block as documented below.
        pub proxy: pulumi_wasm_rust::Output<
            Option<super::super::types::apimanagement::BackendProxy>,
        >,
        /// The Name of the Resource Group where the API Management Service exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The management URI of the backend host in an external system. This URI can be the ARM Resource ID of Logic Apps, Function Apps or API Apps, or the management endpoint of a Service Fabric cluster.
        pub resource_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A `service_fabric_cluster` block as documented below.
        pub service_fabric_cluster: pulumi_wasm_rust::Output<
            Option<super::super::types::apimanagement::BackendServiceFabricCluster>,
        >,
        /// The title of the backend.
        pub title: pulumi_wasm_rust::Output<Option<String>>,
        /// A `tls` block as documented below.
        pub tls: pulumi_wasm_rust::Output<
            Option<super::super::types::apimanagement::BackendTls>,
        >,
        /// The backend host URL should be specified in the format `"https://backend.com/api"`, avoiding trailing slashes (/) to minimize misconfiguration risks. Azure API Management instance will append the backend resource name to this URL. This URL typically serves as the `base-url` in the [`set-backend-service`](https://learn.microsoft.com/azure/api-management/set-backend-service-policy) policy, enabling seamless transitions from frontend to backend.
        pub url: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: BackendArgs,
    ) -> BackendResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_management_name_binding = args
            .api_management_name
            .get_output(context)
            .get_inner();
        let credentials_binding = args.credentials.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let protocol_binding = args.protocol.get_output(context).get_inner();
        let proxy_binding = args.proxy.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let resource_id_binding = args.resource_id.get_output(context).get_inner();
        let service_fabric_cluster_binding = args
            .service_fabric_cluster
            .get_output(context)
            .get_inner();
        let title_binding = args.title.get_output(context).get_inner();
        let tls_binding = args.tls.get_output(context).get_inner();
        let url_binding = args.url.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:apimanagement/backend:Backend".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiManagementName".into(),
                    value: &api_management_name_binding,
                },
                register_interface::ObjectField {
                    name: "credentials".into(),
                    value: &credentials_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "protocol".into(),
                    value: &protocol_binding,
                },
                register_interface::ObjectField {
                    name: "proxy".into(),
                    value: &proxy_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceId".into(),
                    value: &resource_id_binding,
                },
                register_interface::ObjectField {
                    name: "serviceFabricCluster".into(),
                    value: &service_fabric_cluster_binding,
                },
                register_interface::ObjectField {
                    name: "title".into(),
                    value: &title_binding,
                },
                register_interface::ObjectField {
                    name: "tls".into(),
                    value: &tls_binding,
                },
                register_interface::ObjectField {
                    name: "url".into(),
                    value: &url_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiManagementName".into(),
                },
                register_interface::ResultField {
                    name: "credentials".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "protocol".into(),
                },
                register_interface::ResultField {
                    name: "proxy".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "resourceId".into(),
                },
                register_interface::ResultField {
                    name: "serviceFabricCluster".into(),
                },
                register_interface::ResultField {
                    name: "title".into(),
                },
                register_interface::ResultField {
                    name: "tls".into(),
                },
                register_interface::ResultField {
                    name: "url".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        BackendResult {
            api_management_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiManagementName").unwrap(),
            ),
            credentials: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("credentials").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            protocol: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protocol").unwrap(),
            ),
            proxy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("proxy").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceId").unwrap(),
            ),
            service_fabric_cluster: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceFabricCluster").unwrap(),
            ),
            title: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("title").unwrap(),
            ),
            tls: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tls").unwrap(),
            ),
            url: pulumi_wasm_rust::__private::into_domain(hashmap.remove("url").unwrap()),
        }
    }
}
