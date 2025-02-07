/// Manages a JavaScript UDF Function within Stream Analytics Streaming Job.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleFunctionJavaScriptUDF:
///     type: azure:streamanalytics:FunctionJavaScriptUDF
///     name: example
///     properties:
///       name: example-javascript-function
///       streamAnalyticsJobName: ${exampleGetJob.name}
///       resourceGroupName: ${exampleGetJob.resourceGroupName}
///       script: |
///         function getRandomNumber(in) {
///           return in;
///         }
///       inputs:
///         - type: bigint
///       output:
///         type: bigint
/// variables:
///   example:
///     fn::invoke:
///       function: azure:core:getResourceGroup
///       arguments:
///         name: example-resources
///   exampleGetJob:
///     fn::invoke:
///       function: azure:streamanalytics:getJob
///       arguments:
///         name: example-job
///         resourceGroupName: ${example.name}
/// ```
///
/// ## Import
///
/// Stream Analytics JavaScript UDF Functions can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:streamanalytics/functionJavaScriptUDF:FunctionJavaScriptUDF example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.StreamAnalytics/streamingJobs/job1/functions/func1
/// ```
///
pub mod function_java_script_udf {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FunctionJavaScriptUDFArgs {
        /// One or more `input` blocks as defined below.
        #[builder(into)]
        pub inputs: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::streamanalytics::FunctionJavaScriptUdfInput>,
        >,
        /// The name of the JavaScript UDF Function. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An `output` blocks as defined below.
        #[builder(into)]
        pub output: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::streamanalytics::FunctionJavaScriptUdfOutput,
        >,
        /// The name of the Resource Group where the Stream Analytics Job exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The JavaScript of this UDF Function.
        #[builder(into)]
        pub script: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Stream Analytics Job where this Function should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub stream_analytics_job_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct FunctionJavaScriptUDFResult {
        /// One or more `input` blocks as defined below.
        pub inputs: pulumi_gestalt_rust::Output<
            Vec<super::super::types::streamanalytics::FunctionJavaScriptUdfInput>,
        >,
        /// The name of the JavaScript UDF Function. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// An `output` blocks as defined below.
        pub output: pulumi_gestalt_rust::Output<
            super::super::types::streamanalytics::FunctionJavaScriptUdfOutput,
        >,
        /// The name of the Resource Group where the Stream Analytics Job exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The JavaScript of this UDF Function.
        pub script: pulumi_gestalt_rust::Output<String>,
        /// The name of the Stream Analytics Job where this Function should be created. Changing this forces a new resource to be created.
        pub stream_analytics_job_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: FunctionJavaScriptUDFArgs,
    ) -> FunctionJavaScriptUDFResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let inputs_binding = args.inputs.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let output_binding = args.output.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let script_binding = args.script.get_output(context).get_inner();
        let stream_analytics_job_name_binding = args
            .stream_analytics_job_name
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:streamanalytics/functionJavaScriptUDF:FunctionJavaScriptUDF"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "inputs".into(),
                    value: &inputs_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "output".into(),
                    value: &output_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "script".into(),
                    value: &script_binding,
                },
                register_interface::ObjectField {
                    name: "streamAnalyticsJobName".into(),
                    value: &stream_analytics_job_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        FunctionJavaScriptUDFResult {
            inputs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("inputs"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            output: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("output"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            script: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("script"),
            ),
            stream_analytics_job_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("streamAnalyticsJobName"),
            ),
        }
    }
}
