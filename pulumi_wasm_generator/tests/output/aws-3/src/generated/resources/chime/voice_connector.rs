/// Enables you to connect your phone system to the telephone network at a substantial cost savings by using SIP trunking.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = voice_connector::create(
///         "test",
///         VoiceConnectorArgs::builder()
///             .aws_region("us-east-1")
///             .name("connector-test-1")
///             .require_encryption(true)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Configuration Recorder using the name. For example:
///
/// ```sh
/// $ pulumi import aws:chime/voiceConnector:VoiceConnector test example
/// ```
pub mod voice_connector {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VoiceConnectorArgs {
        /// The AWS Region in which the Amazon Chime Voice Connector is created. Default value: `us-east-1`
        #[builder(into, default)]
        pub aws_region: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Amazon Chime Voice Connector.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// When enabled, requires encryption for the Amazon Chime Voice Connector.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub require_encryption: pulumi_wasm_rust::InputOrOutput<bool>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct VoiceConnectorResult {
        /// ARN (Amazon Resource Name) of the Amazon Chime Voice Connector.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The AWS Region in which the Amazon Chime Voice Connector is created. Default value: `us-east-1`
        pub aws_region: pulumi_wasm_rust::Output<String>,
        /// The name of the Amazon Chime Voice Connector.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The outbound host name for the Amazon Chime Voice Connector.
        pub outbound_host_name: pulumi_wasm_rust::Output<String>,
        /// When enabled, requires encryption for the Amazon Chime Voice Connector.
        ///
        /// The following arguments are optional:
        pub require_encryption: pulumi_wasm_rust::Output<bool>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: VoiceConnectorArgs,
    ) -> VoiceConnectorResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let aws_region_binding = args.aws_region.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let require_encryption_binding = args
            .require_encryption
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:chime/voiceConnector:VoiceConnector".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "awsRegion".into(),
                    value: &aws_region_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "requireEncryption".into(),
                    value: &require_encryption_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "awsRegion".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "outboundHostName".into(),
                },
                register_interface::ResultField {
                    name: "requireEncryption".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VoiceConnectorResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            aws_region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("awsRegion").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            outbound_host_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outboundHostName").unwrap(),
            ),
            require_encryption: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("requireEncryption").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
