/// Manages regional EC2 instance metadata default settings.
/// More information can be found in the [Configure instance metadata options for new instances](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/configuring-IMDS-new-instances.html) user guide.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   enforce-imdsv2:
///     type: aws:ec2:InstanceMetadataDefaults
///     properties:
///       httpTokens: required
///       httpPutResponseHopLimit: 1
/// ```
///
/// ## Import
///
/// You cannot import this resource.
///
pub mod instance_metadata_defaults {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceMetadataDefaultsArgs {
        /// Whether the metadata service is available. Can be `"enabled"`, `"disabled"`, or `"no-preference"`. Default: `"no-preference"`.
        #[builder(into, default)]
        pub http_endpoint: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The desired HTTP PUT response hop limit for instance metadata requests. The larger the number, the further instance metadata requests can travel. Can be an integer from `1` to `64`, or `-1` to indicate no preference. Default: `-1`.
        #[builder(into, default)]
        pub http_put_response_hop_limit: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Whether the metadata service requires session tokens, also referred to as _Instance Metadata Service Version 2 (IMDSv2)_. Can be `"optional"`, `"required"`, or `"no-preference"`. Default: `"no-preference"`.
        #[builder(into, default)]
        pub http_tokens: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Enables or disables access to instance tags from the instance metadata service. Can be `"enabled"`, `"disabled"`, or `"no-preference"`. Default: `"no-preference"`.
        #[builder(into, default)]
        pub instance_metadata_tags: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct InstanceMetadataDefaultsResult {
        /// Whether the metadata service is available. Can be `"enabled"`, `"disabled"`, or `"no-preference"`. Default: `"no-preference"`.
        pub http_endpoint: pulumi_wasm_rust::Output<String>,
        /// The desired HTTP PUT response hop limit for instance metadata requests. The larger the number, the further instance metadata requests can travel. Can be an integer from `1` to `64`, or `-1` to indicate no preference. Default: `-1`.
        pub http_put_response_hop_limit: pulumi_wasm_rust::Output<i32>,
        /// Whether the metadata service requires session tokens, also referred to as _Instance Metadata Service Version 2 (IMDSv2)_. Can be `"optional"`, `"required"`, or `"no-preference"`. Default: `"no-preference"`.
        pub http_tokens: pulumi_wasm_rust::Output<String>,
        /// Enables or disables access to instance tags from the instance metadata service. Can be `"enabled"`, `"disabled"`, or `"no-preference"`. Default: `"no-preference"`.
        pub instance_metadata_tags: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: InstanceMetadataDefaultsArgs,
    ) -> InstanceMetadataDefaultsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let http_endpoint_binding = args.http_endpoint.get_output(context).get_inner();
        let http_put_response_hop_limit_binding = args
            .http_put_response_hop_limit
            .get_output(context)
            .get_inner();
        let http_tokens_binding = args.http_tokens.get_output(context).get_inner();
        let instance_metadata_tags_binding = args
            .instance_metadata_tags
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/instanceMetadataDefaults:InstanceMetadataDefaults".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "httpEndpoint".into(),
                    value: &http_endpoint_binding,
                },
                register_interface::ObjectField {
                    name: "httpPutResponseHopLimit".into(),
                    value: &http_put_response_hop_limit_binding,
                },
                register_interface::ObjectField {
                    name: "httpTokens".into(),
                    value: &http_tokens_binding,
                },
                register_interface::ObjectField {
                    name: "instanceMetadataTags".into(),
                    value: &instance_metadata_tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        InstanceMetadataDefaultsResult {
            http_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("httpEndpoint"),
            ),
            http_put_response_hop_limit: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("httpPutResponseHopLimit"),
            ),
            http_tokens: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("httpTokens"),
            ),
            instance_metadata_tags: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("instanceMetadataTags"),
            ),
        }
    }
}
