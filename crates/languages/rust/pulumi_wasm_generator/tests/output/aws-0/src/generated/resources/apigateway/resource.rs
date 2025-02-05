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
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourceArgs {
        /// ID of the parent API resource
        #[builder(into)]
        pub parent_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Last path segment of this API resource.
        #[builder(into)]
        pub path_part: pulumi_wasm_rust::InputOrOutput<String>,
        /// ID of the associated REST API
        #[builder(into)]
        pub rest_api: pulumi_wasm_rust::InputOrOutput<String>,
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
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ResourceArgs,
    ) -> ResourceResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let parent_id_binding = args.parent_id.get_output(context).get_inner();
        let path_part_binding = args.path_part.get_output(context).get_inner();
        let rest_api_binding = args.rest_api.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:apigateway/resource:Resource".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        ResourceResult {
            parent_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("parentId"),
            ),
            path: pulumi_wasm_rust::__private::into_domain(o.extract_field("path")),
            path_part: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pathPart"),
            ),
            rest_api: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("restApi"),
            ),
        }
    }
}
