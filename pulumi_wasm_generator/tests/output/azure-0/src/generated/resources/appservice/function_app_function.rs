/// Manages a Function App Function.
///
/// ## Example Usage
///
/// ### Basic HTTP Trigger
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-group
///       location: West Europe
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: examplesa
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///   exampleServicePlan:
///     type: azure:appservice:ServicePlan
///     name: example
///     properties:
///       name: example-service-plan
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       osType: Linux
///       skuName: S1
///   exampleLinuxFunctionApp:
///     type: azure:appservice:LinuxFunctionApp
///     name: example
///     properties:
///       name: example-function-app
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       servicePlanId: ${exampleServicePlan.id}
///       storageAccountName: ${exampleAccount.name}
///       storageAccountAccessKey: ${exampleAccount.primaryAccessKey}
///       siteConfig:
///         applicationStack:
///           pythonVersion: '3.9'
///   exampleFunctionAppFunction:
///     type: azure:appservice:FunctionAppFunction
///     name: example
///     properties:
///       name: example-function-app-function
///       functionAppId: ${exampleLinuxFunctionApp.id}
///       language: Python
///       testData:
///         fn::toJSON:
///           name: Azure
///       configJson:
///         fn::toJSON:
///           bindings:
///             - authLevel: function
///               direction: in
///               methods:
///                 - get
///                 - post
///               name: req
///               type: httpTrigger
///             - direction: out
///               name: $return
///               type: http
/// ```
///
/// ### HTTP Trigger With Code Upload
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-group
///       location: West Europe
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: examplesa
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///   exampleServicePlan:
///     type: azure:appservice:ServicePlan
///     name: example
///     properties:
///       name: example-service-plan
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       osType: Windows
///       skuName: S1
///   exampleWindowsFunctionApp:
///     type: azure:appservice:WindowsFunctionApp
///     name: example
///     properties:
///       name: example-function-app
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       servicePlanId: ${exampleServicePlan.id}
///       storageAccountName: ${exampleAccount.name}
///       storageAccountAccessKey: ${exampleAccount.primaryAccessKey}
///       siteConfig:
///         applicationStack:
///           dotnetVersion: '6'
///   exampleFunctionAppFunction:
///     type: azure:appservice:FunctionAppFunction
///     name: example
///     properties:
///       name: example-function-app-function
///       functionAppId: ${exampleWindowsFunctionApp.id}
///       language: CSharp
///       files:
///         - name: run.csx
///           content:
///             fn::invoke:
///               function: std:file
///               arguments:
///                 input: exampledata/run.csx
///               return: result
///       testData:
///         fn::toJSON:
///           name: Azure
///       configJson:
///         fn::toJSON:
///           bindings:
///             - authLevel: function
///               direction: in
///               methods:
///                 - get
///                 - post
///               name: req
///               type: httpTrigger
///             - direction: out
///               name: $return
///               type: http
/// ```
///
/// ## Import
///
/// a Function App Function can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appservice/functionAppFunction:FunctionAppFunction example "/subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resGroup1/providers/Microsoft.Web/sites/site1/functions/function1"
/// ```
///
pub mod function_app_function {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FunctionAppFunctionArgs {
        /// The config for this Function in JSON format.
        #[builder(into)]
        pub config_json: pulumi_wasm_rust::Output<String>,
        /// Should this function be enabled. Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A `file` block as detailed below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub files: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::appservice::FunctionAppFunctionFile>>,
        >,
        /// The ID of the Function App in which this function should reside. Changing this forces a new resource to be created.
        #[builder(into)]
        pub function_app_id: pulumi_wasm_rust::Output<String>,
        /// The language the Function is written in. Possible values are `CSharp`, `Custom`, `Java`, `Javascript`, `Python`, `PowerShell`, and `TypeScript`.
        ///
        /// > **NOTE:** when using `Custom` language, you must specify the code handler in the `host.json` file for your function. See the [official docs](https://docs.microsoft.com/azure/azure-functions/functions-custom-handlers#hostjson) for more information.
        #[builder(into, default)]
        pub language: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the function. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The test data for the function.
        #[builder(into, default)]
        pub test_data: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct FunctionAppFunctionResult {
        /// The config for this Function in JSON format.
        pub config_json: pulumi_wasm_rust::Output<String>,
        /// The URL of the configuration JSON.
        pub config_url: pulumi_wasm_rust::Output<String>,
        /// Should this function be enabled. Defaults to `true`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A `file` block as detailed below. Changing this forces a new resource to be created.
        pub files: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::appservice::FunctionAppFunctionFile>>,
        >,
        /// The ID of the Function App in which this function should reside. Changing this forces a new resource to be created.
        pub function_app_id: pulumi_wasm_rust::Output<String>,
        /// The invocation URL.
        pub invocation_url: pulumi_wasm_rust::Output<String>,
        /// The language the Function is written in. Possible values are `CSharp`, `Custom`, `Java`, `Javascript`, `Python`, `PowerShell`, and `TypeScript`.
        ///
        /// > **NOTE:** when using `Custom` language, you must specify the code handler in the `host.json` file for your function. See the [official docs](https://docs.microsoft.com/azure/azure-functions/functions-custom-handlers#hostjson) for more information.
        pub language: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the function. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The Script root path URL.
        pub script_root_path_url: pulumi_wasm_rust::Output<String>,
        /// The script URL.
        pub script_url: pulumi_wasm_rust::Output<String>,
        /// The URL for the Secrets File.
        pub secrets_file_url: pulumi_wasm_rust::Output<String>,
        /// The test data for the function.
        pub test_data: pulumi_wasm_rust::Output<Option<String>>,
        /// The Test data URL.
        pub test_data_url: pulumi_wasm_rust::Output<String>,
        /// The function URL.
        pub url: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: FunctionAppFunctionArgs,
    ) -> FunctionAppFunctionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let config_json_binding = args.config_json.get_inner();
        let enabled_binding = args.enabled.get_inner();
        let files_binding = args.files.get_inner();
        let function_app_id_binding = args.function_app_id.get_inner();
        let language_binding = args.language.get_inner();
        let name_binding = args.name.get_inner();
        let test_data_binding = args.test_data.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appservice/functionAppFunction:FunctionAppFunction".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "configJson".into(),
                    value: &config_json_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "files".into(),
                    value: &files_binding,
                },
                register_interface::ObjectField {
                    name: "functionAppId".into(),
                    value: &function_app_id_binding,
                },
                register_interface::ObjectField {
                    name: "language".into(),
                    value: &language_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "testData".into(),
                    value: &test_data_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "configJson".into(),
                },
                register_interface::ResultField {
                    name: "configUrl".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "files".into(),
                },
                register_interface::ResultField {
                    name: "functionAppId".into(),
                },
                register_interface::ResultField {
                    name: "invocationUrl".into(),
                },
                register_interface::ResultField {
                    name: "language".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "scriptRootPathUrl".into(),
                },
                register_interface::ResultField {
                    name: "scriptUrl".into(),
                },
                register_interface::ResultField {
                    name: "secretsFileUrl".into(),
                },
                register_interface::ResultField {
                    name: "testData".into(),
                },
                register_interface::ResultField {
                    name: "testDataUrl".into(),
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
        FunctionAppFunctionResult {
            config_json: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configJson").unwrap(),
            ),
            config_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configUrl").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            files: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("files").unwrap(),
            ),
            function_app_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("functionAppId").unwrap(),
            ),
            invocation_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("invocationUrl").unwrap(),
            ),
            language: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("language").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            script_root_path_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scriptRootPathUrl").unwrap(),
            ),
            script_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scriptUrl").unwrap(),
            ),
            secrets_file_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secretsFileUrl").unwrap(),
            ),
            test_data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("testData").unwrap(),
            ),
            test_data_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("testDataUrl").unwrap(),
            ),
            url: pulumi_wasm_rust::__private::into_domain(hashmap.remove("url").unwrap()),
        }
    }
}
