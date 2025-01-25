/// Provides an AppSync Function.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = graph_ql_api::create(
///         "example",
///         GraphQlApiArgs::builder()
///             .authentication_type("API_KEY")
///             .name("example")
///             .schema(
///                 "type Mutation {\n  putPost(id: ID!, title: String!): Post\n}\n\ntype Post {\n  id: ID!\n  title: String!\n}\n\ntype Query {\n  singlePost(id: ID!): Post\n}\n\nschema {\n  query: Query\n  mutation: Mutation\n}\n",
///             )
///             .build_struct(),
///     );
///     let exampleDataSource = data_source::create(
///         "exampleDataSource",
///         DataSourceArgs::builder()
///             .api_id("${example.id}")
///             .http_config(
///                 DataSourceHttpConfig::builder()
///                     .endpoint("http://example.com")
///                     .build_struct(),
///             )
///             .name("example")
///             .type_("HTTP")
///             .build_struct(),
///     );
///     let exampleFunction = function::create(
///         "exampleFunction",
///         FunctionArgs::builder()
///             .api_id("${example.id}")
///             .data_source("${exampleDataSource.name}")
///             .name("example")
///             .request_mapping_template(
///                 "{\n    \"version\": \"2018-05-29\",\n    \"method\": \"GET\",\n    \"resourcePath\": \"/\",\n    \"params\":{\n        \"headers\": $utils.http.copyheaders($ctx.request.headers)\n    }\n}\n",
///             )
///             .response_mapping_template(
///                 "#if($ctx.result.statusCode == 200)\n    $ctx.result.body\n#else\n    $utils.appendError($ctx.result.body, $ctx.result.statusCode)\n#end",
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
///
/// ### With Code
///
/// ```yaml
/// resources:
///   example:
///     type: aws:appsync:Function
///     properties:
///       apiId: ${exampleAwsAppsyncGraphqlApi.id}
///       dataSource: ${exampleAwsAppsyncDatasource.name}
///       name: example
///       code:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: some-code-dir
///           return: result
///       runtime:
///         name: APPSYNC_JS
///         runtimeVersion: 1.0.0
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_appsync_function` using the AppSync API ID and Function ID separated by `-`. For example:
///
/// ```sh
/// $ pulumi import aws:appsync/function:Function example xxxxx-yyyyy
/// ```
pub mod function {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FunctionArgs {
        /// ID of the associated AppSync API.
        #[builder(into)]
        pub api_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The function code that contains the request and response functions. When code is used, the runtime is required. The runtime value must be APPSYNC_JS.
        #[builder(into, default)]
        pub code: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Function data source name.
        #[builder(into)]
        pub data_source: pulumi_wasm_rust::InputOrOutput<String>,
        /// Function description.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Version of the request mapping template. Currently the supported value is `2018-05-29`. Does not apply when specifying `code`.
        #[builder(into, default)]
        pub function_version: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Maximum batching size for a resolver. Valid values are between `0` and `2000`.
        #[builder(into, default)]
        pub max_batch_size: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Function name. The function name does not have to be unique.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Function request mapping template. Functions support only the 2018-05-29 version of the request mapping template.
        #[builder(into, default)]
        pub request_mapping_template: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Function response mapping template.
        #[builder(into, default)]
        pub response_mapping_template: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Describes a runtime used by an AWS AppSync pipeline resolver or AWS AppSync function. Specifies the name and version of the runtime to use. Note that if a runtime is specified, code must also be specified. See `runtime` Block for details.
        #[builder(into, default)]
        pub runtime: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::appsync::FunctionRuntime>,
        >,
        /// Describes a Sync configuration for a resolver. See `sync_config` Block for details.
        #[builder(into, default)]
        pub sync_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::appsync::FunctionSyncConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct FunctionResult {
        /// ID of the associated AppSync API.
        pub api_id: pulumi_wasm_rust::Output<String>,
        /// ARN of the Function object.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The function code that contains the request and response functions. When code is used, the runtime is required. The runtime value must be APPSYNC_JS.
        pub code: pulumi_wasm_rust::Output<Option<String>>,
        /// Function data source name.
        pub data_source: pulumi_wasm_rust::Output<String>,
        /// Function description.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Unique ID representing the Function object.
        pub function_id: pulumi_wasm_rust::Output<String>,
        /// Version of the request mapping template. Currently the supported value is `2018-05-29`. Does not apply when specifying `code`.
        pub function_version: pulumi_wasm_rust::Output<String>,
        /// Maximum batching size for a resolver. Valid values are between `0` and `2000`.
        pub max_batch_size: pulumi_wasm_rust::Output<Option<i32>>,
        /// Function name. The function name does not have to be unique.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Function request mapping template. Functions support only the 2018-05-29 version of the request mapping template.
        pub request_mapping_template: pulumi_wasm_rust::Output<Option<String>>,
        /// Function response mapping template.
        pub response_mapping_template: pulumi_wasm_rust::Output<Option<String>>,
        /// Describes a runtime used by an AWS AppSync pipeline resolver or AWS AppSync function. Specifies the name and version of the runtime to use. Note that if a runtime is specified, code must also be specified. See `runtime` Block for details.
        pub runtime: pulumi_wasm_rust::Output<
            Option<super::super::types::appsync::FunctionRuntime>,
        >,
        /// Describes a Sync configuration for a resolver. See `sync_config` Block for details.
        pub sync_config: pulumi_wasm_rust::Output<
            Option<super::super::types::appsync::FunctionSyncConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: FunctionArgs,
    ) -> FunctionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_id_binding = args.api_id.get_output(context).get_inner();
        let code_binding = args.code.get_output(context).get_inner();
        let data_source_binding = args.data_source.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let function_version_binding = args
            .function_version
            .get_output(context)
            .get_inner();
        let max_batch_size_binding = args.max_batch_size.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request_mapping_template_binding = args
            .request_mapping_template
            .get_output(context)
            .get_inner();
        let response_mapping_template_binding = args
            .response_mapping_template
            .get_output(context)
            .get_inner();
        let runtime_binding = args.runtime.get_output(context).get_inner();
        let sync_config_binding = args.sync_config.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:appsync/function:Function".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiId".into(),
                    value: &api_id_binding,
                },
                register_interface::ObjectField {
                    name: "code".into(),
                    value: &code_binding,
                },
                register_interface::ObjectField {
                    name: "dataSource".into(),
                    value: &data_source_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "functionVersion".into(),
                    value: &function_version_binding,
                },
                register_interface::ObjectField {
                    name: "maxBatchSize".into(),
                    value: &max_batch_size_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "requestMappingTemplate".into(),
                    value: &request_mapping_template_binding,
                },
                register_interface::ObjectField {
                    name: "responseMappingTemplate".into(),
                    value: &response_mapping_template_binding,
                },
                register_interface::ObjectField {
                    name: "runtime".into(),
                    value: &runtime_binding,
                },
                register_interface::ObjectField {
                    name: "syncConfig".into(),
                    value: &sync_config_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiId".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "code".into(),
                },
                register_interface::ResultField {
                    name: "dataSource".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "functionId".into(),
                },
                register_interface::ResultField {
                    name: "functionVersion".into(),
                },
                register_interface::ResultField {
                    name: "maxBatchSize".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "requestMappingTemplate".into(),
                },
                register_interface::ResultField {
                    name: "responseMappingTemplate".into(),
                },
                register_interface::ResultField {
                    name: "runtime".into(),
                },
                register_interface::ResultField {
                    name: "syncConfig".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FunctionResult {
            api_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiId").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            code: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("code").unwrap(),
            ),
            data_source: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataSource").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            function_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("functionId").unwrap(),
            ),
            function_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("functionVersion").unwrap(),
            ),
            max_batch_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxBatchSize").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            request_mapping_template: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("requestMappingTemplate").unwrap(),
            ),
            response_mapping_template: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("responseMappingTemplate").unwrap(),
            ),
            runtime: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("runtime").unwrap(),
            ),
            sync_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("syncConfig").unwrap(),
            ),
        }
    }
}
