/// Manages status (recording / stopped) of an AWS Config Configuration Recorder.
///
/// > **Note:** Starting Configuration Recorder requires a Delivery Channel to be present. Use of `depends_on` (as shown below) is recommended to avoid race conditions.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   foo:
///     type: aws:cfg:RecorderStatus
///     properties:
///       name: ${fooRecorder.name}
///       isEnabled: true
///     options:
///       dependsOn:
///         - ${fooDeliveryChannel}
///   a:
///     type: aws:iam:RolePolicyAttachment
///     properties:
///       role: ${r.name}
///       policyArn: arn:aws:iam::aws:policy/service-role/AWS_ConfigRole
///   b:
///     type: aws:s3:BucketV2
///     properties:
///       bucket: awsconfig-example
///   fooDeliveryChannel:
///     type: aws:cfg:DeliveryChannel
///     name: foo
///     properties:
///       name: example
///       s3BucketName: ${b.bucket}
///   fooRecorder:
///     type: aws:cfg:Recorder
///     name: foo
///     properties:
///       name: example
///       roleArn: ${r.arn}
///   r:
///     type: aws:iam:Role
///     properties:
///       name: example-awsconfig
///       assumeRolePolicy: ${assumeRole.json}
///   pRolePolicy:
///     type: aws:iam:RolePolicy
///     name: p
///     properties:
///       name: awsconfig-example
///       role: ${r.id}
///       policy: ${p.json}
/// variables:
///   assumeRole:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - config.amazonaws.com
///             actions:
///               - sts:AssumeRole
///   p:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             actions:
///               - s3:*
///             resources:
///               - ${b.arn}
///               - ${b.arn}/*
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Configuration Recorder Status using the name of the Configuration Recorder. For example:
///
/// ```sh
/// $ pulumi import aws:cfg/recorderStatus:RecorderStatus foo example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod recorder_status {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RecorderStatusArgs {
        /// Whether the configuration recorder should be enabled or disabled.
        #[builder(into)]
        pub is_enabled: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// The name of the recorder
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct RecorderStatusResult {
        /// Whether the configuration recorder should be enabled or disabled.
        pub is_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The name of the recorder
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RecorderStatusArgs,
    ) -> RecorderStatusResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let is_enabled_binding = args.is_enabled.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cfg/recorderStatus:RecorderStatus".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "isEnabled".into(),
                    value: is_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RecorderStatusResult {
            is_enabled: o.get_field("isEnabled"),
            name: o.get_field("name"),
        }
    }
}
