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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod function_app_function {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FunctionAppFunctionArgs {
        /// The config for this Function in JSON format.
        #[builder(into)]
        pub config_json: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Should this function be enabled. Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A `file` block as detailed below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub files: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::appservice::FunctionAppFunctionFile>>,
        >,
        /// The ID of the Function App in which this function should reside. Changing this forces a new resource to be created.
        #[builder(into)]
        pub function_app_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The language the Function is written in. Possible values are `CSharp`, `Custom`, `Java`, `Javascript`, `Python`, `PowerShell`, and `TypeScript`.
        ///
        /// > **NOTE:** when using `Custom` language, you must specify the code handler in the `host.json` file for your function. See the [official docs](https://docs.microsoft.com/azure/azure-functions/functions-custom-handlers#hostjson) for more information.
        #[builder(into, default)]
        pub language: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the function. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The test data for the function.
        #[builder(into, default)]
        pub test_data: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct FunctionAppFunctionResult {
        /// The config for this Function in JSON format.
        pub config_json: pulumi_gestalt_rust::Output<String>,
        /// The URL of the configuration JSON.
        pub config_url: pulumi_gestalt_rust::Output<String>,
        /// Should this function be enabled. Defaults to `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A `file` block as detailed below. Changing this forces a new resource to be created.
        pub files: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::appservice::FunctionAppFunctionFile>>,
        >,
        /// The ID of the Function App in which this function should reside. Changing this forces a new resource to be created.
        pub function_app_id: pulumi_gestalt_rust::Output<String>,
        /// The invocation URL.
        pub invocation_url: pulumi_gestalt_rust::Output<String>,
        /// The language the Function is written in. Possible values are `CSharp`, `Custom`, `Java`, `Javascript`, `Python`, `PowerShell`, and `TypeScript`.
        ///
        /// > **NOTE:** when using `Custom` language, you must specify the code handler in the `host.json` file for your function. See the [official docs](https://docs.microsoft.com/azure/azure-functions/functions-custom-handlers#hostjson) for more information.
        pub language: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the function. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Script root path URL.
        pub script_root_path_url: pulumi_gestalt_rust::Output<String>,
        /// The script URL.
        pub script_url: pulumi_gestalt_rust::Output<String>,
        /// The URL for the Secrets File.
        pub secrets_file_url: pulumi_gestalt_rust::Output<String>,
        /// The test data for the function.
        pub test_data: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Test data URL.
        pub test_data_url: pulumi_gestalt_rust::Output<String>,
        /// The function URL.
        pub url: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FunctionAppFunctionArgs,
    ) -> FunctionAppFunctionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let config_json_binding = args.config_json.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let files_binding = args.files.get_output(context);
        let function_app_id_binding = args.function_app_id.get_output(context);
        let language_binding = args.language.get_output(context);
        let name_binding = args.name.get_output(context);
        let test_data_binding = args.test_data.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:appservice/functionAppFunction:FunctionAppFunction".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configJson".into(),
                    value: config_json_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "files".into(),
                    value: files_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "functionAppId".into(),
                    value: function_app_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "language".into(),
                    value: language_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "testData".into(),
                    value: test_data_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        FunctionAppFunctionResult {
            config_json: o.get_field("configJson"),
            config_url: o.get_field("configUrl"),
            enabled: o.get_field("enabled"),
            files: o.get_field("files"),
            function_app_id: o.get_field("functionAppId"),
            invocation_url: o.get_field("invocationUrl"),
            language: o.get_field("language"),
            name: o.get_field("name"),
            script_root_path_url: o.get_field("scriptRootPathUrl"),
            script_url: o.get_field("scriptUrl"),
            secrets_file_url: o.get_field("secretsFileUrl"),
            test_data: o.get_field("testData"),
            test_data_url: o.get_field("testDataUrl"),
            url: o.get_field("url"),
        }
    }
}
