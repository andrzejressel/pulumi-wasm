/// Represents a sharedflow attachment to a flowhook point.
///
///
/// To get more information about Flowhook, see:
///
/// * [API documentation](https://cloud.google.com/apigee/docs/reference/apis/apigee/rest/v1/organizations.environments.flowhooks#FlowHook)
/// * How-to Guides
///     * [organizations.environments.flowhooks](https://cloud.google.com/apigee/docs/reference/apis/apigee/rest/v1/organizations.environments.flowhooks#FlowHook)
///
/// ## Import
///
/// Flowhook can be imported using any of these accepted formats:
///
/// * `organizations/{{org_id}}/environments/{{environment}}/flowhooks/{{flow_hook_point}}`
///
/// * `{{org_id}}/{{environment}}/{{flow_hook_point}}`
///
/// When using the `pulumi import` command, Flowhook can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:apigee/flowhook:Flowhook default organizations/{{org_id}}/environments/{{environment}}/flowhooks/{{flow_hook_point}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:apigee/flowhook:Flowhook default {{org_id}}/{{environment}}/{{flow_hook_point}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod flowhook {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FlowhookArgs {
        /// Flag that specifies whether execution should continue if the flow hook throws an exception. Set to true to continue execution. Set to false to stop execution if the flow hook throws an exception. Defaults to true.
        #[builder(into, default)]
        pub continue_on_error: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Description of the flow hook.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The resource ID of the environment.
        #[builder(into)]
        pub environment: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Where in the API call flow the flow hook is invoked. Must be one of PreProxyFlowHook, PostProxyFlowHook, PreTargetFlowHook, or PostTargetFlowHook.
        #[builder(into)]
        pub flow_hook_point: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Apigee Organization associated with the environment
        #[builder(into)]
        pub org_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Id of the Sharedflow attaching to a flowhook point.
        #[builder(into)]
        pub sharedflow: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct FlowhookResult {
        /// Flag that specifies whether execution should continue if the flow hook throws an exception. Set to true to continue execution. Set to false to stop execution if the flow hook throws an exception. Defaults to true.
        pub continue_on_error: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Description of the flow hook.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The resource ID of the environment.
        pub environment: pulumi_gestalt_rust::Output<String>,
        /// Where in the API call flow the flow hook is invoked. Must be one of PreProxyFlowHook, PostProxyFlowHook, PreTargetFlowHook, or PostTargetFlowHook.
        pub flow_hook_point: pulumi_gestalt_rust::Output<String>,
        /// The Apigee Organization associated with the environment
        pub org_id: pulumi_gestalt_rust::Output<String>,
        /// Id of the Sharedflow attaching to a flowhook point.
        pub sharedflow: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FlowhookArgs,
    ) -> FlowhookResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let continue_on_error_binding = args.continue_on_error.get_output(context);
        let description_binding = args.description.get_output(context);
        let environment_binding = args.environment.get_output(context);
        let flow_hook_point_binding = args.flow_hook_point.get_output(context);
        let org_id_binding = args.org_id.get_output(context);
        let sharedflow_binding = args.sharedflow.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:apigee/flowhook:Flowhook".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "continueOnError".into(),
                    value: &continue_on_error_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "environment".into(),
                    value: &environment_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "flowHookPoint".into(),
                    value: &flow_hook_point_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "orgId".into(),
                    value: &org_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sharedflow".into(),
                    value: &sharedflow_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        FlowhookResult {
            continue_on_error: o.get_field("continueOnError"),
            description: o.get_field("description"),
            environment: o.get_field("environment"),
            flow_hook_point: o.get_field("flowHookPoint"),
            org_id: o.get_field("orgId"),
            sharedflow: o.get_field("sharedflow"),
        }
    }
}
