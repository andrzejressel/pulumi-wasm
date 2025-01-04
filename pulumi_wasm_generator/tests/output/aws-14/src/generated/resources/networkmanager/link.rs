/// Creates a link for a site.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = link::create(
///         "example",
///         LinkArgs::builder()
///             .bandwidth(
///                 LinkBandwidth::builder().downloadSpeed(50).uploadSpeed(10).build_struct(),
///             )
///             .global_network_id("${exampleAwsNetworkmanagerGlobalNetwork.id}")
///             .provider_name("MegaCorp")
///             .site_id("${exampleAwsNetworkmanagerSite.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_networkmanager_link` using the link ARN. For example:
///
/// ```sh
/// $ pulumi import aws:networkmanager/link:Link example arn:aws:networkmanager::123456789012:link/global-network-0d47f6t230mz46dy4/link-444555aaabbb11223
/// ```
pub mod link {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LinkArgs {
        /// The upload speed and download speed in Mbps. Documented below.
        #[builder(into)]
        pub bandwidth: pulumi_wasm_rust::Output<
            super::super::types::networkmanager::LinkBandwidth,
        >,
        /// A description of the link.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the global network.
        #[builder(into)]
        pub global_network_id: pulumi_wasm_rust::Output<String>,
        /// The provider of the link.
        #[builder(into, default)]
        pub provider_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the site.
        #[builder(into)]
        pub site_id: pulumi_wasm_rust::Output<String>,
        /// Key-value tags for the link. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of the link.
        #[builder(into, default)]
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct LinkResult {
        /// Link Amazon Resource Name (ARN).
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The upload speed and download speed in Mbps. Documented below.
        pub bandwidth: pulumi_wasm_rust::Output<
            super::super::types::networkmanager::LinkBandwidth,
        >,
        /// A description of the link.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the global network.
        pub global_network_id: pulumi_wasm_rust::Output<String>,
        /// The provider of the link.
        pub provider_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the site.
        pub site_id: pulumi_wasm_rust::Output<String>,
        /// Key-value tags for the link. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The type of the link.
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: LinkArgs) -> LinkResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bandwidth_binding = args.bandwidth.get_inner();
        let description_binding = args.description.get_inner();
        let global_network_id_binding = args.global_network_id.get_inner();
        let provider_name_binding = args.provider_name.get_inner();
        let site_id_binding = args.site_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let type__binding = args.type_.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:networkmanager/link:Link".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bandwidth".into(),
                    value: &bandwidth_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "globalNetworkId".into(),
                    value: &global_network_id_binding,
                },
                register_interface::ObjectField {
                    name: "providerName".into(),
                    value: &provider_name_binding,
                },
                register_interface::ObjectField {
                    name: "siteId".into(),
                    value: &site_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "bandwidth".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "globalNetworkId".into(),
                },
                register_interface::ResultField {
                    name: "providerName".into(),
                },
                register_interface::ResultField {
                    name: "siteId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LinkResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            bandwidth: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bandwidth").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            global_network_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("globalNetworkId").unwrap(),
            ),
            provider_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("providerName").unwrap(),
            ),
            site_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("siteId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
