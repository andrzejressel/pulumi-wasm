/// Manages an AWS CloudFront Origin Access Control, which is used by CloudFront Distributions with an Amazon S3 bucket as the origin.
///
/// Read more about Origin Access Control in the [CloudFront Developer Guide](https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/private-content-restricting-access-to-s3.html).
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
///     let example = origin_access_control::create(
///         "example",
///         OriginAccessControlArgs::builder()
///             .description("Example Policy")
///             .name("example")
///             .origin_access_control_origin_type("s3")
///             .signing_behavior("always")
///             .signing_protocol("sigv4")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CloudFront Origin Access Control using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudfront/originAccessControl:OriginAccessControl example E327GJI25M56DG
/// ```
pub mod origin_access_control {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OriginAccessControlArgs {
        /// The description of the Origin Access Control. Defaults to "Managed by Pulumi" if omitted.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A name that identifies the Origin Access Control.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The type of origin that this Origin Access Control is for. Valid values are `lambda`, `mediapackagev2`, `mediastore`, and `s3`.
        #[builder(into)]
        pub origin_access_control_origin_type: pulumi_wasm_rust::Output<String>,
        /// Specifies which requests CloudFront signs. Specify `always` for the most common use case. Allowed values: `always`, `never`, and `no-override`.
        #[builder(into)]
        pub signing_behavior: pulumi_wasm_rust::Output<String>,
        /// Determines how CloudFront signs (authenticates) requests. The only valid value is `sigv4`.
        #[builder(into)]
        pub signing_protocol: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct OriginAccessControlResult {
        /// The description of the Origin Access Control. Defaults to "Managed by Pulumi" if omitted.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The current version of this Origin Access Control.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// A name that identifies the Origin Access Control.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The type of origin that this Origin Access Control is for. Valid values are `lambda`, `mediapackagev2`, `mediastore`, and `s3`.
        pub origin_access_control_origin_type: pulumi_wasm_rust::Output<String>,
        /// Specifies which requests CloudFront signs. Specify `always` for the most common use case. Allowed values: `always`, `never`, and `no-override`.
        pub signing_behavior: pulumi_wasm_rust::Output<String>,
        /// Determines how CloudFront signs (authenticates) requests. The only valid value is `sigv4`.
        pub signing_protocol: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: OriginAccessControlArgs,
    ) -> OriginAccessControlResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let origin_access_control_origin_type_binding = args
            .origin_access_control_origin_type
            .get_inner();
        let signing_behavior_binding = args.signing_behavior.get_inner();
        let signing_protocol_binding = args.signing_protocol.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudfront/originAccessControl:OriginAccessControl".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "originAccessControlOriginType".into(),
                    value: &origin_access_control_origin_type_binding,
                },
                register_interface::ObjectField {
                    name: "signingBehavior".into(),
                    value: &signing_behavior_binding,
                },
                register_interface::ObjectField {
                    name: "signingProtocol".into(),
                    value: &signing_protocol_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "originAccessControlOriginType".into(),
                },
                register_interface::ResultField {
                    name: "signingBehavior".into(),
                },
                register_interface::ResultField {
                    name: "signingProtocol".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        OriginAccessControlResult {
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            origin_access_control_origin_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("originAccessControlOriginType").unwrap(),
            ),
            signing_behavior: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("signingBehavior").unwrap(),
            ),
            signing_protocol: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("signingProtocol").unwrap(),
            ),
        }
    }
}