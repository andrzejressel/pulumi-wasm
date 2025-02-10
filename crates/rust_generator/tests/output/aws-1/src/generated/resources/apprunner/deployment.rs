/// Manages an App Runner Deployment Operation.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = deployment::create(
///         "example",
///         DeploymentArgs::builder()
///             .service_arn("${exampleAwsApprunnerService.arn}")
///             .build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod deployment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DeploymentArgs {
        /// The Amazon Resource Name (ARN) of the App Runner service to start the deployment for.
        #[builder(into)]
        pub service_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apprunner::DeploymentTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct DeploymentResult {
        /// The unique ID of the operation associated with deployment.
        pub operation_id: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the App Runner service to start the deployment for.
        pub service_arn: pulumi_gestalt_rust::Output<String>,
        /// The current status of the App Runner service deployment.
        pub status: pulumi_gestalt_rust::Output<String>,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::apprunner::DeploymentTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DeploymentArgs,
    ) -> DeploymentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let service_arn_binding = args.service_arn.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:apprunner/deployment:Deployment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceArn".into(),
                    value: service_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: timeouts_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DeploymentResult {
            operation_id: o.get_field("operationId"),
            service_arn: o.get_field("serviceArn"),
            status: o.get_field("status"),
            timeouts: o.get_field("timeouts"),
        }
    }
}
