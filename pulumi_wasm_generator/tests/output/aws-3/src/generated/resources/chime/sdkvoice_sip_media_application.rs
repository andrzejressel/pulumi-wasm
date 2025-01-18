/// A ChimeSDKVoice SIP Media Application is a managed object that passes values from a SIP rule to a target AWS Lambda function.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = sdkvoice_sip_media_application::create(
///         "example",
///         SdkvoiceSipMediaApplicationArgs::builder()
///             .aws_region("us-east-1")
///             .endpoints(
///                 SdkvoiceSipMediaApplicationEndpoints::builder()
///                     .lambdaArn("${test.arn}")
///                     .build_struct(),
///             )
///             .name("example-sip-media-application")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import a ChimeSDKVoice SIP Media Application using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:chime/sdkvoiceSipMediaApplication:SdkvoiceSipMediaApplication example abcdef123456
/// ```
pub mod sdkvoice_sip_media_application {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SdkvoiceSipMediaApplicationArgs {
        /// The AWS Region in which the AWS Chime SDK Voice Sip Media Application is created.
        #[builder(into)]
        pub aws_region: pulumi_wasm_rust::Output<String>,
        /// List of endpoints (Lambda Amazon Resource Names) specified for the SIP media application. Currently, only one endpoint is supported. See `endpoints`.
        #[builder(into)]
        pub endpoints: pulumi_wasm_rust::Output<
            super::super::types::chime::SdkvoiceSipMediaApplicationEndpoints,
        >,
        /// The name of the AWS Chime SDK Voice Sip Media Application.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct SdkvoiceSipMediaApplicationResult {
        /// ARN (Amazon Resource Name) of the AWS Chime SDK Voice Sip Media Application
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The AWS Region in which the AWS Chime SDK Voice Sip Media Application is created.
        pub aws_region: pulumi_wasm_rust::Output<String>,
        /// List of endpoints (Lambda Amazon Resource Names) specified for the SIP media application. Currently, only one endpoint is supported. See `endpoints`.
        pub endpoints: pulumi_wasm_rust::Output<
            super::super::types::chime::SdkvoiceSipMediaApplicationEndpoints,
        >,
        /// The name of the AWS Chime SDK Voice Sip Media Application.
        ///
        /// The following arguments are optional:
        pub name: pulumi_wasm_rust::Output<String>,
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
        name: &str,
        args: SdkvoiceSipMediaApplicationArgs,
    ) -> SdkvoiceSipMediaApplicationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let aws_region_binding = args.aws_region.get_inner();
        let endpoints_binding = args.endpoints.get_inner();
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:chime/sdkvoiceSipMediaApplication:SdkvoiceSipMediaApplication"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "awsRegion".into(),
                    value: &aws_region_binding,
                },
                register_interface::ObjectField {
                    name: "endpoints".into(),
                    value: &endpoints_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
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
                    name: "endpoints".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SdkvoiceSipMediaApplicationResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            aws_region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("awsRegion").unwrap(),
            ),
            endpoints: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpoints").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
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
