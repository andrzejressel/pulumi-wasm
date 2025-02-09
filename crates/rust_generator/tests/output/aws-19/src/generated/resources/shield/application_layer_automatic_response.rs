/// Resource for managing an AWS Shield Application Layer Automatic Response for automatic DDoS mitigation.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// configuration:
///   distributionId:
///     type: string
/// resources:
///   example:
///     type: aws:shield:ApplicationLayerAutomaticResponse
///     properties:
///       resourceArn: arn:${currentGetPartition.partition}:cloudfront:${currentGetCallerIdentity.accountId}:distribution/${distributionId}
///       action: COUNT
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getRegion
///       arguments: {}
///   currentGetCallerIdentity:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
///   currentGetPartition:
///     fn::invoke:
///       function: aws:getPartition
///       arguments: {}
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod application_layer_automatic_response {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApplicationLayerAutomaticResponseArgs {
        /// One of `COUNT` or `BLOCK`
        #[builder(into)]
        pub action: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ARN of the resource to protect (Cloudfront Distributions and ALBs only at this time).
        #[builder(into)]
        pub resource_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::shield::ApplicationLayerAutomaticResponseTimeouts,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct ApplicationLayerAutomaticResponseResult {
        /// One of `COUNT` or `BLOCK`
        pub action: pulumi_gestalt_rust::Output<String>,
        /// ARN of the resource to protect (Cloudfront Distributions and ALBs only at this time).
        pub resource_arn: pulumi_gestalt_rust::Output<String>,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::shield::ApplicationLayerAutomaticResponseTimeouts,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ApplicationLayerAutomaticResponseArgs,
    ) -> ApplicationLayerAutomaticResponseResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let action_binding = args.action.get_output(context);
        let resource_arn_binding = args.resource_arn.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:shield/applicationLayerAutomaticResponse:ApplicationLayerAutomaticResponse"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "action".into(),
                    value: action_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceArn".into(),
                    value: resource_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: timeouts_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ApplicationLayerAutomaticResponseResult {
            action: o.get_field("action"),
            resource_arn: o.get_field("resourceArn"),
            timeouts: o.get_field("timeouts"),
        }
    }
}
