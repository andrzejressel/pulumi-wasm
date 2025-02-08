/// Manages a JavaScript UDA Function within a Stream Analytics Streaming Job.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleFunctionJavascriptUda:
///     type: azure:streamanalytics:FunctionJavascriptUda
///     name: example
///     properties:
///       name: example-javascript-function
///       streamAnalyticsJobId: ${exampleGetJob.id}
///       script: |
///         function main() {
///             this.init = function () {
///                 this.state = 0;
///             }
///
///             this.accumulate = function (value, timestamp) {
///                 this.state += value;
///             }
///
///             this.computeResult = function () {
///                 return this.state;
///             }
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
/// Stream Analytics JavaScript UDA Functions can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:streamanalytics/functionJavascriptUda:FunctionJavascriptUda example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.StreamAnalytics/streamingJobs/job1/functions/func1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod function_javascript_uda {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FunctionJavascriptUdaArgs {
        /// One or more `input` blocks as defined below.
        #[builder(into)]
        pub inputs: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::streamanalytics::FunctionJavascriptUdaInput>,
        >,
        /// The name of the JavaScript UDA Function. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An `output` block as defined below.
        #[builder(into)]
        pub output: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::streamanalytics::FunctionJavascriptUdaOutput,
        >,
        /// The JavaScript of this UDA Function.
        #[builder(into)]
        pub script: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The resource ID of the Stream Analytics Job where this Function should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub stream_analytics_job_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct FunctionJavascriptUdaResult {
        /// One or more `input` blocks as defined below.
        pub inputs: pulumi_gestalt_rust::Output<
            Vec<super::super::types::streamanalytics::FunctionJavascriptUdaInput>,
        >,
        /// The name of the JavaScript UDA Function. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// An `output` block as defined below.
        pub output: pulumi_gestalt_rust::Output<
            super::super::types::streamanalytics::FunctionJavascriptUdaOutput,
        >,
        /// The JavaScript of this UDA Function.
        pub script: pulumi_gestalt_rust::Output<String>,
        /// The resource ID of the Stream Analytics Job where this Function should be created. Changing this forces a new resource to be created.
        pub stream_analytics_job_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: FunctionJavascriptUdaArgs,
    ) -> FunctionJavascriptUdaResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let inputs_binding = args.inputs.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let output_binding = args.output.get_output(context).get_inner();
        let script_binding = args.script.get_output(context).get_inner();
        let stream_analytics_job_id_binding = args
            .stream_analytics_job_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:streamanalytics/functionJavascriptUda:FunctionJavascriptUda"
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
                    name: "script".into(),
                    value: &script_binding,
                },
                register_interface::ObjectField {
                    name: "streamAnalyticsJobId".into(),
                    value: &stream_analytics_job_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        FunctionJavascriptUdaResult {
            inputs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("inputs"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            output: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("output"),
            ),
            script: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("script"),
            ),
            stream_analytics_job_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("streamAnalyticsJobId"),
            ),
        }
    }
}
