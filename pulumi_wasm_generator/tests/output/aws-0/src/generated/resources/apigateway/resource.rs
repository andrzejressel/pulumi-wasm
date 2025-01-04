/// Provides an API Gateway Resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let myDemoAPI = rest_api::create(
///         "myDemoAPI",
///         RestApiArgs::builder()
///             .description("This is my API for demonstration purposes")
///             .name("MyDemoAPI")
///             .build_struct(),
///     );
///     let myDemoResource = resource::create(
///         "myDemoResource",
///         ResourceArgs::builder()
///             .parent_id("${myDemoAPI.rootResourceId}")
///             .path_part("mydemoresource")
///             .rest_api("${myDemoAPI.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_api_gateway_resource` using `REST-API-ID/RESOURCE-ID`. For example:
///
/// ```sh
/// $ pulumi import aws:apigateway/resource:Resource example 12345abcde/67890fghij
/// ```
pub mod resource {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourceArgs {
        /// ID of the parent API resource
        #[builder(into)]
        pub parent_id: pulumi_wasm_rust::Output<String>,
        /// Last path segment of this API resource.
        #[builder(into)]
        pub path_part: pulumi_wasm_rust::Output<String>,
        /// ID of the associated REST API
        #[builder(into)]
        pub rest_api: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ResourceResult {
        /// ID of the parent API resource
        pub parent_id: pulumi_wasm_rust::Output<String>,
        /// Complete path for this API resource, including all parent paths.
        pub path: pulumi_wasm_rust::Output<String>,
        /// Last path segment of this API resource.
        pub path_part: pulumi_wasm_rust::Output<String>,
        /// ID of the associated REST API
        pub rest_api: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ResourceArgs) -> ResourceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let parent_id_binding = args.parent_id.get_inner();
        let path_part_binding = args.path_part.get_inner();
        let rest_api_binding = args.rest_api.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:apigateway/resource:Resource".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "parentId".into(),
                    value: &parent_id_binding,
                },
                register_interface::ObjectField {
                    name: "pathPart".into(),
                    value: &path_part_binding,
                },
                register_interface::ObjectField {
                    name: "restApi".into(),
                    value: &rest_api_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "parentId".into(),
                },
                register_interface::ResultField {
                    name: "path".into(),
                },
                register_interface::ResultField {
                    name: "pathPart".into(),
                },
                register_interface::ResultField {
                    name: "restApi".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ResourceResult {
            parent_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parentId").unwrap(),
            ),
            path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("path").unwrap(),
            ),
            path_part: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pathPart").unwrap(),
            ),
            rest_api: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("restApi").unwrap(),
            ),
        }
    }
}
