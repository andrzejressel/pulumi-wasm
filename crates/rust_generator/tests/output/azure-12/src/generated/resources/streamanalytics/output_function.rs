/// Manages a Stream Analytics Output Function.
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
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .account_replication_type("LRS")
///             .account_tier("Standard")
///             .location("${example.location}")
///             .name("examplestorageaccount")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleFunctionApp = function_app::create(
///         "exampleFunctionApp",
///         FunctionAppArgs::builder()
///             .app_service_plan_id("${examplePlan.id}")
///             .location("${example.location}")
///             .name("examplefunctionapp")
///             .os_type("linux")
///             .resource_group_name("${example.name}")
///             .storage_account_access_key("${exampleAccount.primaryAccessKey}")
///             .storage_account_name("${exampleAccount.name}")
///             .version("~3")
///             .build_struct(),
///     );
///     let exampleJob = job::create(
///         "exampleJob",
///         JobArgs::builder()
///             .location("${example.location}")
///             .name("examplestreamanalyticsjob")
///             .resource_group_name("${example.name}")
///             .streaming_units(3)
///             .transformation_query(
///                 "    SELECT *\n    INTO [YourOutputAlias]\n    FROM [YourInputAlias]\n",
///             )
///             .build_struct(),
///     );
///     let exampleOutputFunction = output_function::create(
///         "exampleOutputFunction",
///         OutputFunctionArgs::builder()
///             .api_key("exampleapikey")
///             .function_app("${exampleFunctionApp.name}")
///             .function_name("examplefunctionname")
///             .name("exampleoutput")
///             .resource_group_name("${exampleJob.resourceGroupName}")
///             .stream_analytics_job_name("${exampleJob.name}")
///             .build_struct(),
///     );
///     let examplePlan = plan::create(
///         "examplePlan",
///         PlanArgs::builder()
///             .kind("FunctionApp")
///             .location("${example.location}")
///             .name("exampleappserviceplan")
///             .reserved(true)
///             .resource_group_name("${example.name}")
///             .sku(PlanSku::builder().size("Y1").tier("Dynamic").build_struct())
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Stream Analytics Output Functions can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:streamanalytics/outputFunction:OutputFunction example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.StreamAnalytics/streamingJobs/job1/outputs/output1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod output_function {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OutputFunctionArgs {
        /// The API key for the Function.
        #[builder(into)]
        pub api_key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The maximum number of events in each batch that's sent to the function. Defaults to `100`.
        #[builder(into, default)]
        pub batch_max_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The maximum batch size in bytes that's sent to the function. Defaults to `262144` (256 kB).
        #[builder(into, default)]
        pub batch_max_in_bytes: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The name of the Function App.
        #[builder(into)]
        pub function_app: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the function in the Function App.
        #[builder(into)]
        pub function_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this Stream Analytics Output. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Stream Analytics Output should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Stream Analytics Job. Changing this forces a new resource to be created.
        #[builder(into)]
        pub stream_analytics_job_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct OutputFunctionResult {
        /// The API key for the Function.
        pub api_key: pulumi_gestalt_rust::Output<String>,
        /// The maximum number of events in each batch that's sent to the function. Defaults to `100`.
        pub batch_max_count: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The maximum batch size in bytes that's sent to the function. Defaults to `262144` (256 kB).
        pub batch_max_in_bytes: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The name of the Function App.
        pub function_app: pulumi_gestalt_rust::Output<String>,
        /// The name of the function in the Function App.
        pub function_name: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Stream Analytics Output. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Stream Analytics Output should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Stream Analytics Job. Changing this forces a new resource to be created.
        pub stream_analytics_job_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: OutputFunctionArgs,
    ) -> OutputFunctionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let api_key_binding_1 = args.api_key.get_output(context);
        let api_key_binding = api_key_binding_1.get_inner();
        let batch_max_count_binding_1 = args.batch_max_count.get_output(context);
        let batch_max_count_binding = batch_max_count_binding_1.get_inner();
        let batch_max_in_bytes_binding_1 = args.batch_max_in_bytes.get_output(context);
        let batch_max_in_bytes_binding = batch_max_in_bytes_binding_1.get_inner();
        let function_app_binding_1 = args.function_app.get_output(context);
        let function_app_binding = function_app_binding_1.get_inner();
        let function_name_binding_1 = args.function_name.get_output(context);
        let function_name_binding = function_name_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let stream_analytics_job_name_binding_1 = args
            .stream_analytics_job_name
            .get_output(context);
        let stream_analytics_job_name_binding = stream_analytics_job_name_binding_1
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:streamanalytics/outputFunction:OutputFunction".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiKey".into(),
                    value: &api_key_binding,
                },
                register_interface::ObjectField {
                    name: "batchMaxCount".into(),
                    value: &batch_max_count_binding,
                },
                register_interface::ObjectField {
                    name: "batchMaxInBytes".into(),
                    value: &batch_max_in_bytes_binding,
                },
                register_interface::ObjectField {
                    name: "functionApp".into(),
                    value: &function_app_binding,
                },
                register_interface::ObjectField {
                    name: "functionName".into(),
                    value: &function_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "streamAnalyticsJobName".into(),
                    value: &stream_analytics_job_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        OutputFunctionResult {
            api_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("apiKey"),
            ),
            batch_max_count: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("batchMaxCount"),
            ),
            batch_max_in_bytes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("batchMaxInBytes"),
            ),
            function_app: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("functionApp"),
            ),
            function_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("functionName"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            stream_analytics_job_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("streamAnalyticsJobName"),
            ),
        }
    }
}
