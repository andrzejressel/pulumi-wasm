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
pub mod sink_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SinkPolicyArgs {
        /// JSON policy to use. If you are updating an existing policy, the entire existing policy is replaced by what you specify here.
        #[builder(into)]
        pub policy: pulumi_wasm_rust::Output<String>,
        /// ARN of the sink to attach this policy to.
        #[builder(into)]
        pub sink_identifier: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct SinkPolicyResult {
        /// ARN of the Sink.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// JSON policy to use. If you are updating an existing policy, the entire existing policy is replaced by what you specify here.
        pub policy: pulumi_wasm_rust::Output<String>,
        /// ID string that AWS generated as part of the sink ARN.
        pub sink_id: pulumi_wasm_rust::Output<String>,
        /// ARN of the sink to attach this policy to.
        pub sink_identifier: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: SinkPolicyArgs) -> SinkPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let policy_binding = args.policy.get_inner();
        let sink_identifier_binding = args.sink_identifier.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "policy".into(),
                },
                register_interface::ResultField {
                    name: "sinkId".into(),
                },
                register_interface::ResultField {
                    name: "sinkIdentifier".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SinkPolicyResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policy").unwrap(),
            ),
            sink_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sinkId").unwrap(),
            ),
            sink_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sinkIdentifier").unwrap(),
            ),
        }
    }
}
