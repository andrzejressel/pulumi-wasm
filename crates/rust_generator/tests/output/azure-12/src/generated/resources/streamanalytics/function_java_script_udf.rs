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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FunctionJavaScriptUDFArgs,
    ) -> FunctionJavaScriptUDFResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let inputs_binding = args.inputs.get_output(context);
        let name_binding = args.name.get_output(context);
        let output_binding = args.output.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let script_binding = args.script.get_output(context);
        let stream_analytics_job_name_binding = args
            .stream_analytics_job_name
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:streamanalytics/functionJavaScriptUDF:FunctionJavaScriptUDF"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "inputs".into(),
                    value: &inputs_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "output".into(),
                    value: &output_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "script".into(),
                    value: &script_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "streamAnalyticsJobName".into(),
                    value: &stream_analytics_job_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        FunctionJavaScriptUDFResult {
            inputs: o.get_field("inputs"),
            name: o.get_field("name"),
            output: o.get_field("output"),
            resource_group_name: o.get_field("resourceGroupName"),
            script: o.get_field("script"),
            stream_analytics_job_name: o.get_field("streamAnalyticsJobName"),
        }
    }
}
