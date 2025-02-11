/// Manages a backend within an API Management Service.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod backend {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackendArgs {
        /// The Name of the API Management Service where this backend should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_management_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `credentials` block as documented below.
        #[builder(into, default)]
        pub credentials: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apimanagement::BackendCredentials>,
        >,
        /// The description of the backend.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the API Management backend. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The protocol used by the backend host. Possible values are `http` or `soap`.
        #[builder(into)]
        pub protocol: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `proxy` block as documented below.
        #[builder(into, default)]
        pub proxy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apimanagement::BackendProxy>,
        >,
        /// The Name of the Resource Group where the API Management Service exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The management URI of the backend host in an external system. This URI can be the ARM Resource ID of Logic Apps, Function Apps or API Apps, or the management endpoint of a Service Fabric cluster.
        #[builder(into, default)]
        pub resource_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `service_fabric_cluster` block as documented below.
        #[builder(into, default)]
        pub service_fabric_cluster: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apimanagement::BackendServiceFabricCluster>,
        >,
        /// The title of the backend.
        #[builder(into, default)]
        pub title: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `tls` block as documented below.
        #[builder(into, default)]
        pub tls: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apimanagement::BackendTls>,
        >,
        /// The backend host URL should be specified in the format `"https://backend.com/api"`, avoiding trailing slashes (/) to minimize misconfiguration risks. Azure API Management instance will append the backend resource name to this URL. This URL typically serves as the `base-url` in the [`set-backend-service`](https://learn.microsoft.com/azure/api-management/set-backend-service-policy) policy, enabling seamless transitions from frontend to backend.
        #[builder(into)]
        pub url: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct BackendResult {
        /// The Name of the API Management Service where this backend should be created. Changing this forces a new resource to be created.
        pub api_management_name: pulumi_gestalt_rust::Output<String>,
        /// A `credentials` block as documented below.
        pub credentials: pulumi_gestalt_rust::Output<
            Option<super::super::types::apimanagement::BackendCredentials>,
        >,
        /// The description of the backend.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the API Management backend. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The protocol used by the backend host. Possible values are `http` or `soap`.
        pub protocol: pulumi_gestalt_rust::Output<String>,
        /// A `proxy` block as documented below.
        pub proxy: pulumi_gestalt_rust::Output<
            Option<super::super::types::apimanagement::BackendProxy>,
        >,
        /// The Name of the Resource Group where the API Management Service exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The management URI of the backend host in an external system. This URI can be the ARM Resource ID of Logic Apps, Function Apps or API Apps, or the management endpoint of a Service Fabric cluster.
        pub resource_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `service_fabric_cluster` block as documented below.
        pub service_fabric_cluster: pulumi_gestalt_rust::Output<
            Option<super::super::types::apimanagement::BackendServiceFabricCluster>,
        >,
        /// The title of the backend.
        pub title: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `tls` block as documented below.
        pub tls: pulumi_gestalt_rust::Output<
            Option<super::super::types::apimanagement::BackendTls>,
        >,
        /// The backend host URL should be specified in the format `"https://backend.com/api"`, avoiding trailing slashes (/) to minimize misconfiguration risks. Azure API Management instance will append the backend resource name to this URL. This URL typically serves as the `base-url` in the [`set-backend-service`](https://learn.microsoft.com/azure/api-management/set-backend-service-policy) policy, enabling seamless transitions from frontend to backend.
        pub url: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BackendArgs,
    ) -> BackendResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_management_name_binding = args.api_management_name.get_output(context);
        let credentials_binding = args.credentials.get_output(context);
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let protocol_binding = args.protocol.get_output(context);
        let proxy_binding = args.proxy.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let resource_id_binding = args.resource_id.get_output(context);
        let service_fabric_cluster_binding = args
            .service_fabric_cluster
            .get_output(context);
        let title_binding = args.title.get_output(context);
        let tls_binding = args.tls.get_output(context);
        let url_binding = args.url.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:apimanagement/backend:Backend".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiManagementName".into(),
                    value: &api_management_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "credentials".into(),
                    value: &credentials_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "protocol".into(),
                    value: &protocol_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "proxy".into(),
                    value: &proxy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceId".into(),
                    value: &resource_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceFabricCluster".into(),
                    value: &service_fabric_cluster_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "title".into(),
                    value: &title_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tls".into(),
                    value: &tls_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "url".into(),
                    value: &url_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        BackendResult {
            api_management_name: o.get_field("apiManagementName"),
            credentials: o.get_field("credentials"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            protocol: o.get_field("protocol"),
            proxy: o.get_field("proxy"),
            resource_group_name: o.get_field("resourceGroupName"),
            resource_id: o.get_field("resourceId"),
            service_fabric_cluster: o.get_field("serviceFabricCluster"),
            title: o.get_field("title"),
            tls: o.get_field("tls"),
            url: o.get_field("url"),
        }
    }
}
