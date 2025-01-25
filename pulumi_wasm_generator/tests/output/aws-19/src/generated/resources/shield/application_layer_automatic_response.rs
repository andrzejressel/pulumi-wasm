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
pub mod application_layer_automatic_response {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApplicationLayerAutomaticResponseArgs {
        /// One of `COUNT` or `BLOCK`
        #[builder(into)]
        pub action: pulumi_wasm_rust::InputOrOutput<String>,
        /// ARN of the resource to protect (Cloudfront Distributions and ALBs only at this time).
        #[builder(into)]
        pub resource_arn: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::shield::ApplicationLayerAutomaticResponseTimeouts,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct ApplicationLayerAutomaticResponseResult {
        /// One of `COUNT` or `BLOCK`
        pub action: pulumi_wasm_rust::Output<String>,
        /// ARN of the resource to protect (Cloudfront Distributions and ALBs only at this time).
        pub resource_arn: pulumi_wasm_rust::Output<String>,
        pub timeouts: pulumi_wasm_rust::Output<
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
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ApplicationLayerAutomaticResponseArgs,
    ) -> ApplicationLayerAutomaticResponseResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let action_binding = args.action.get_output(context).get_inner();
        let resource_arn_binding = args.resource_arn.get_output(context).get_inner();
        let timeouts_binding = args.timeouts.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:shield/applicationLayerAutomaticResponse:ApplicationLayerAutomaticResponse"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "action".into(),
                    value: &action_binding,
                },
                register_interface::ObjectField {
                    name: "resourceArn".into(),
                    value: &resource_arn_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "action".into(),
                },
                register_interface::ResultField {
                    name: "resourceArn".into(),
                },
                register_interface::ResultField {
                    name: "timeouts".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ApplicationLayerAutomaticResponseResult {
            action: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("action").unwrap(),
            ),
            resource_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceArn").unwrap(),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeouts").unwrap(),
            ),
        }
    }
}
