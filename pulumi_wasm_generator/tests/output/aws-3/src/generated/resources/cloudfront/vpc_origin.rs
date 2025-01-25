/// Creates an Amazon CloudFront VPC origin.
///
/// For information about CloudFront VPC origins, see
/// [Amazon CloudFront Developer Guide - Restrict access with VPC origins][1].
///
/// ## Example Usage
///
/// ### Application Load Balancer
///
/// The following example below creates a CloudFront VPC origin for a Application Load Balancer.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let alb = vpc_origin::create(
///         "alb",
///         VpcOriginArgs::builder()
///             .vpc_origin_endpoint_config(
///                 VpcOriginVpcOriginEndpointConfig::builder()
///                     .arn("${this.arn}")
///                     .httpPort(8080)
///                     .httpsPort(8443)
///                     .name("Example VPC Origin")
///                     .originProtocolPolicy("https-only")
///                     .originSslProtocols(
///                         VpcOriginVpcOriginEndpointConfigOriginSslProtocols::builder()
///                             .items(vec!["TLSv1.2",])
///                             .quantity(1)
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// terraform
///
/// import {
///
///   to = aws_cloudfront_vpc_origin.origin
///
///   id = vo_JQEa410sssUFoY6wMkx69j
///
/// }
///
/// Using `pulumi import`, import Cloudfront VPC origins using the `id`. For example:
///
/// console
///
/// % pulumi import aws_cloudfront_vpc_origin vo_JQEa410sssUFoY6wMkx69j
///
pub mod vpc_origin {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcOriginArgs {
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::cloudfront::VpcOriginTimeouts>,
        >,
        #[builder(into, default)]
        pub vpc_origin_endpoint_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::cloudfront::VpcOriginVpcOriginEndpointConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct VpcOriginResult {
        /// The VPC origin ARN.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The current version of the origin.
        pub etag: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudfront::VpcOriginTimeouts>,
        >,
        pub vpc_origin_endpoint_config: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudfront::VpcOriginVpcOriginEndpointConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: VpcOriginArgs,
    ) -> VpcOriginResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let tags_binding = args.tags.get_output(context).get_inner();
        let timeouts_binding = args.timeouts.get_output(context).get_inner();
        let vpc_origin_endpoint_config_binding = args
            .vpc_origin_endpoint_config
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudfront/vpcOrigin:VpcOrigin".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
                register_interface::ObjectField {
                    name: "vpcOriginEndpointConfig".into(),
                    value: &vpc_origin_endpoint_config_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "timeouts".into(),
                },
                register_interface::ResultField {
                    name: "vpcOriginEndpointConfig".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VpcOriginResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeouts").unwrap(),
            ),
            vpc_origin_endpoint_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcOriginEndpointConfig").unwrap(),
            ),
        }
    }
}
