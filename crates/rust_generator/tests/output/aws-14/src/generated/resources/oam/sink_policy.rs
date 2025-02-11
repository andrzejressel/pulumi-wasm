/// Resource for managing an AWS CloudWatch Observability Access Manager Sink Policy.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:oam:Sink
///     properties:
///       name: ExampleSink
///   exampleSinkPolicy:
///     type: aws:oam:SinkPolicy
///     name: example
///     properties:
///       sinkIdentifier: ${example.id}
///       policy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Action:
///                 - oam:CreateLink
///                 - oam:UpdateLink
///               Effect: Allow
///               Resource: '*'
///               Principal:
///                 AWS:
///                   - '1111111111111'
///                   - '222222222222'
///               Condition:
///                 ForAllValues:StringEquals:
///                   oam:ResourceTypes:
///                     - AWS::CloudWatch::Metric
///                     - AWS::Logs::LogGroup
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CloudWatch Observability Access Manager Sink Policy using the `sink_identifier`. For example:
///
/// ```sh
/// $ pulumi import aws:oam/sinkPolicy:SinkPolicy example arn:aws:oam:us-west-2:123456789012:sink/sink-id
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod sink_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SinkPolicyArgs {
        /// JSON policy to use. If you are updating an existing policy, the entire existing policy is replaced by what you specify here.
        #[builder(into)]
        pub policy: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ARN of the sink to attach this policy to.
        #[builder(into)]
        pub sink_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SinkPolicyResult {
        /// ARN of the Sink.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// JSON policy to use. If you are updating an existing policy, the entire existing policy is replaced by what you specify here.
        pub policy: pulumi_gestalt_rust::Output<String>,
        /// ID string that AWS generated as part of the sink ARN.
        pub sink_id: pulumi_gestalt_rust::Output<String>,
        /// ARN of the sink to attach this policy to.
        pub sink_identifier: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SinkPolicyArgs,
    ) -> SinkPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let policy_binding = args.policy.get_output(context);
        let sink_identifier_binding = args.sink_identifier.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:oam/sinkPolicy:SinkPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policy".into(),
                    value: &policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sinkIdentifier".into(),
                    value: &sink_identifier_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SinkPolicyResult {
            arn: o.get_field("arn"),
            policy: o.get_field("policy"),
            sink_id: o.get_field("sinkId"),
            sink_identifier: o.get_field("sinkIdentifier"),
        }
    }
}
