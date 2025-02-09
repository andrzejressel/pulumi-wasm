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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SinkPolicyArgs,
    ) -> SinkPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let policy_binding_1 = args.policy.get_output(context);
        let policy_binding = policy_binding_1.get_inner();
        let sink_identifier_binding_1 = args.sink_identifier.get_output(context);
        let sink_identifier_binding = sink_identifier_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:oam/sinkPolicy:SinkPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "policy".into(),
                    value: &policy_binding,
                },
                register_interface::ObjectField {
                    name: "sinkIdentifier".into(),
                    value: &sink_identifier_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SinkPolicyResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("policy"),
            ),
            sink_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sinkId"),
            ),
            sink_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sinkIdentifier"),
            ),
        }
    }
}
