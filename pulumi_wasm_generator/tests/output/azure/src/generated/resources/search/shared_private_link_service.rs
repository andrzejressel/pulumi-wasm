/// Manages the Shared Private Link Service for an Azure Search Service.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = resource_group::create(
///         "test",
///         ResourceGroupArgs::builder()
///             .location("east us")
///             .name("example-resourceGroup")
///             .build_struct(),
///     );
///     let testAccount = account::create(
///         "testAccount",
///         AccountArgs::builder()
///             .account_replication_type("LRS")
///             .account_tier("Standard")
///             .location("${test.location}")
///             .name("xiaxintestsaforsearchspl")
///             .resource_group_name("${test.name}")
///             .build_struct(),
///     );
///     let testService = service::create(
///         "testService",
///         ServiceArgs::builder()
///             .location("${test.location}")
///             .name("example-search")
///             .resource_group_name("${test.name}")
///             .sku("standard")
///             .build_struct(),
///     );
///     let testSharedPrivateLinkService = shared_private_link_service::create(
///         "testSharedPrivateLinkService",
///         SharedPrivateLinkServiceArgs::builder()
///             .name("example-spl")
///             .request_message("please approve")
///             .search_service_id("${testService.id}")
///             .subresource_name("blob")
///             .target_resource_id("${testAccount.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Azure Search Shared Private Link Resource can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:search/sharedPrivateLinkService:SharedPrivateLinkService example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Search/searchServices/service1/sharedPrivateLinkResources/resource1
/// ```
///
pub mod shared_private_link_service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SharedPrivateLinkServiceArgs {
        /// Specify the name of the Azure Search Shared Private Link Resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specify the request message for requesting approval of the Shared Private Link Enabled Remote Resource.
        #[builder(into, default)]
        pub request_message: pulumi_wasm_rust::Output<Option<String>>,
        /// Specify the id of the Azure Search Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub search_service_id: pulumi_wasm_rust::Output<String>,
        /// Specify the sub resource name which the Azure Search Private Endpoint is able to connect to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub subresource_name: pulumi_wasm_rust::Output<String>,
        /// Specify the ID of the Shared Private Link Enabled Remote Resource which this Azure Search Private Endpoint should be connected to. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The sub resource name should match with the type of the target resource id that's being specified.
        #[builder(into)]
        pub target_resource_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct SharedPrivateLinkServiceResult {
        /// Specify the name of the Azure Search Shared Private Link Resource. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specify the request message for requesting approval of the Shared Private Link Enabled Remote Resource.
        pub request_message: pulumi_wasm_rust::Output<Option<String>>,
        /// Specify the id of the Azure Search Service. Changing this forces a new resource to be created.
        pub search_service_id: pulumi_wasm_rust::Output<String>,
        /// The status of a private endpoint connection. Possible values are Pending, Approved, Rejected or Disconnected.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Specify the sub resource name which the Azure Search Private Endpoint is able to connect to. Changing this forces a new resource to be created.
        pub subresource_name: pulumi_wasm_rust::Output<String>,
        /// Specify the ID of the Shared Private Link Enabled Remote Resource which this Azure Search Private Endpoint should be connected to. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The sub resource name should match with the type of the target resource id that's being specified.
        pub target_resource_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: SharedPrivateLinkServiceArgs,
    ) -> SharedPrivateLinkServiceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let request_message_binding = args.request_message.get_inner();
        let search_service_id_binding = args.search_service_id.get_inner();
        let subresource_name_binding = args.subresource_name.get_inner();
        let target_resource_id_binding = args.target_resource_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:search/sharedPrivateLinkService:SharedPrivateLinkService"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "requestMessage".into(),
                    value: &request_message_binding,
                },
                register_interface::ObjectField {
                    name: "searchServiceId".into(),
                    value: &search_service_id_binding,
                },
                register_interface::ObjectField {
                    name: "subresourceName".into(),
                    value: &subresource_name_binding,
                },
                register_interface::ObjectField {
                    name: "targetResourceId".into(),
                    value: &target_resource_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "requestMessage".into(),
                },
                register_interface::ResultField {
                    name: "searchServiceId".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "subresourceName".into(),
                },
                register_interface::ResultField {
                    name: "targetResourceId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SharedPrivateLinkServiceResult {
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            request_message: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("requestMessage").unwrap(),
            ),
            search_service_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("searchServiceId").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            subresource_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subresourceName").unwrap(),
            ),
            target_resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetResourceId").unwrap(),
            ),
        }
    }
}