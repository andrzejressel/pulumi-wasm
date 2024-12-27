/// The [Cloudflare Managed Headers](https://developers.cloudflare.com/rules/transform/managed-transforms/)
/// allows you to add or remove some predefined headers to one's
/// requests or origin responses.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = managed_headers::create(
///         "example",
///         ManagedHeadersArgs::builder()
///             .managed_request_headers(
///                 vec![
///                     ManagedHeadersManagedRequestHeader::builder().enabled(true)
///                     .id("add_true_client_ip_headers").build_struct(),
///                 ],
///             )
///             .managed_response_headers(
///                 vec![
///                     ManagedHeadersManagedResponseHeader::builder().enabled(true)
///                     .id("remove_x-powered-by_header").build_struct(),
///                 ],
///             )
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
/// }
/// ```
pub mod managed_headers {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ManagedHeadersArgs {
        /// The list of managed request headers.
        #[builder(into, default)]
        pub managed_request_headers: pulumi_wasm_rust::Output<
            Option<Vec<super::types::ManagedHeadersManagedRequestHeader>>,
        >,
        /// The list of managed response headers.
        #[builder(into, default)]
        pub managed_response_headers: pulumi_wasm_rust::Output<
            Option<Vec<super::types::ManagedHeadersManagedResponseHeader>>,
        >,
        /// The zone identifier to target for the resource.
        #[builder(into)]
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ManagedHeadersResult {
        /// The list of managed request headers.
        pub managed_request_headers: pulumi_wasm_rust::Output<
            Option<Vec<super::types::ManagedHeadersManagedRequestHeader>>,
        >,
        /// The list of managed response headers.
        pub managed_response_headers: pulumi_wasm_rust::Output<
            Option<Vec<super::types::ManagedHeadersManagedResponseHeader>>,
        >,
        /// The zone identifier to target for the resource.
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ManagedHeadersArgs) -> ManagedHeadersResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let managed_request_headers_binding = args.managed_request_headers.get_inner();
        let managed_response_headers_binding = args.managed_response_headers.get_inner();
        let zone_id_binding = args.zone_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/managedHeaders:ManagedHeaders".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "managedRequestHeaders".into(),
                    value: &managed_request_headers_binding,
                },
                register_interface::ObjectField {
                    name: "managedResponseHeaders".into(),
                    value: &managed_response_headers_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "managedRequestHeaders".into(),
                },
                register_interface::ResultField {
                    name: "managedResponseHeaders".into(),
                },
                register_interface::ResultField {
                    name: "zoneId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ManagedHeadersResult {
            managed_request_headers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managedRequestHeaders").unwrap(),
            ),
            managed_response_headers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managedResponseHeaders").unwrap(),
            ),
            zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneId").unwrap(),
            ),
        }
    }
}
