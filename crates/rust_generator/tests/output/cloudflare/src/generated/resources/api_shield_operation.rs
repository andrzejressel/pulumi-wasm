/// Provides a resource to manage an operation in API Shield Endpoint Management.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = api_shield_operation::create(
///         "example",
///         ApiShieldOperationArgs::builder()
///             .endpoint("/path")
///             .host("api.example.com")
///             .method("GET")
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod api_shield_operation {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApiShieldOperationArgs {
        /// The endpoint which can contain path parameter templates in curly braces, each will be replaced from left to right with `{varN}`, starting with `{var1}`. This will then be [Cloudflare-normalized](https://developers.cloudflare.com/rules/normalization/how-it-works/). **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub endpoint: pulumi_gestalt_rust::InputOrOutput<String>,
        /// RFC3986-compliant host. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub host: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The HTTP method used to access the endpoint. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub method: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ApiShieldOperationResult {
        /// The endpoint which can contain path parameter templates in curly braces, each will be replaced from left to right with `{varN}`, starting with `{var1}`. This will then be [Cloudflare-normalized](https://developers.cloudflare.com/rules/normalization/how-it-works/). **Modifying this attribute will force creation of a new resource.**
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        /// RFC3986-compliant host. **Modifying this attribute will force creation of a new resource.**
        pub host: pulumi_gestalt_rust::Output<String>,
        /// The HTTP method used to access the endpoint. **Modifying this attribute will force creation of a new resource.**
        pub method: pulumi_gestalt_rust::Output<String>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ApiShieldOperationArgs,
    ) -> ApiShieldOperationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let endpoint_binding = args.endpoint.get_output(context);
        let host_binding = args.host.get_output(context);
        let method_binding = args.method.get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/apiShieldOperation:ApiShieldOperation".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endpoint".into(),
                    value: endpoint_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "host".into(),
                    value: host_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "method".into(),
                    value: method_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: zone_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ApiShieldOperationResult {
            endpoint: o.get_field("endpoint"),
            host: o.get_field("host"),
            method: o.get_field("method"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
