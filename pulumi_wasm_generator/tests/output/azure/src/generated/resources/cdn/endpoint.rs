/// A CDN Endpoint is the entity within a CDN Profile containing configuration information regarding caching behaviours and origins. The CDN Endpoint is exposed using the URL format `<endpointname>.azureedge.net`.
///
/// !> **Be Aware:** Azure is rolling out a breaking change on Friday 9th April 2021 which may cause issues with the CDN/FrontDoor resources. More information is available in this GitHub issue - however unfortunately this may necessitate a breaking change to the CDN and FrontDoor resources, more information will be posted in the GitHub issue as the necessary changes are identified.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleEndpoint = endpoint::create(
///         "exampleEndpoint",
///         EndpointArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .origins(
///                 vec![
///                     EndpointOrigin::builder().hostName("www.contoso.com").name("example")
///                     .build_struct(),
///                 ],
///             )
///             .profile_name("${exampleProfile.name}")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleProfile = profile::create(
///         "exampleProfile",
///         ProfileArgs::builder()
///             .location("${example.location}")
///             .name("example-cdn")
///             .resource_group_name("${example.name}")
///             .sku("Standard_Verizon")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// CDN Endpoints can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:cdn/endpoint:Endpoint example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Cdn/profiles/myprofile1/endpoints/myendpoint1
/// ```
///
pub mod endpoint {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EndpointArgs {
        /// An array of strings that indicates a content types on which compression will be applied. The value for the elements should be MIME types.
        #[builder(into, default)]
        pub content_types_to_compresses: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Rules for the rules engine. An endpoint can contain up until 4 of those rules that consist of conditions and actions. A `delivery_rule` blocks as defined below.
        #[builder(into, default)]
        pub delivery_rules: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::cdn::EndpointDeliveryRule>>,
        >,
        /// A set of Geo Filters for this CDN Endpoint. Each `geo_filter` block supports fields documented below.
        #[builder(into, default)]
        pub geo_filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::cdn::EndpointGeoFilter>>,
        >,
        /// Actions that are valid for all resources regardless of any conditions. A `global_delivery_rule` block as defined below.
        #[builder(into, default)]
        pub global_delivery_rule: pulumi_wasm_rust::Output<
            Option<super::super::types::cdn::EndpointGlobalDeliveryRule>,
        >,
        /// Indicates whether compression is to be enabled.
        #[builder(into, default)]
        pub is_compression_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies if http allowed. Defaults to `true`.
        #[builder(into, default)]
        pub is_http_allowed: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies if https allowed. Defaults to `true`.
        #[builder(into, default)]
        pub is_https_allowed: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the CDN Endpoint. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// What types of optimization should this CDN Endpoint optimize for? Possible values include `DynamicSiteAcceleration`, `GeneralMediaStreaming`, `GeneralWebDelivery`, `LargeFileDownload` and `VideoOnDemandMediaStreaming`.
        #[builder(into, default)]
        pub optimization_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The host header CDN provider will send along with content requests to origins.
        #[builder(into, default)]
        pub origin_host_header: pulumi_wasm_rust::Output<Option<String>>,
        /// The path used at for origin requests.
        #[builder(into, default)]
        pub origin_path: pulumi_wasm_rust::Output<Option<String>>,
        /// The set of origins of the CDN endpoint. When multiple origins exist, the first origin will be used as primary and rest will be used as failover options. Each `origin` block supports fields documented below. Changing this forces a new resource to be created.
        #[builder(into)]
        pub origins: pulumi_wasm_rust::Output<
            Vec<super::super::types::cdn::EndpointOrigin>,
        >,
        /// the path to a file hosted on the origin which helps accelerate delivery of the dynamic content and calculate the most optimal routes for the CDN. This is relative to the `origin_path`.
        ///
        /// > **NOTE:** `global_delivery_rule` and `delivery_rule` are currently only available for `Microsoft_Standard` CDN profiles.
        #[builder(into, default)]
        pub probe_path: pulumi_wasm_rust::Output<Option<String>>,
        /// The CDN Profile to which to attach the CDN Endpoint. Changing this forces a new resource to be created.
        #[builder(into)]
        pub profile_name: pulumi_wasm_rust::Output<String>,
        /// Sets query string caching behavior. Allowed values are `IgnoreQueryString`, `BypassCaching` and `UseQueryString`. `NotSet` value can be used for `Premium Verizon` CDN profile. Defaults to `IgnoreQueryString`.
        #[builder(into, default)]
        pub querystring_caching_behaviour: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the CDN Endpoint. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct EndpointResult {
        /// An array of strings that indicates a content types on which compression will be applied. The value for the elements should be MIME types.
        pub content_types_to_compresses: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Rules for the rules engine. An endpoint can contain up until 4 of those rules that consist of conditions and actions. A `delivery_rule` blocks as defined below.
        pub delivery_rules: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::cdn::EndpointDeliveryRule>>,
        >,
        /// The Fully Qualified Domain Name of the CDN Endpoint.
        pub fqdn: pulumi_wasm_rust::Output<String>,
        /// A set of Geo Filters for this CDN Endpoint. Each `geo_filter` block supports fields documented below.
        pub geo_filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::cdn::EndpointGeoFilter>>,
        >,
        /// Actions that are valid for all resources regardless of any conditions. A `global_delivery_rule` block as defined below.
        pub global_delivery_rule: pulumi_wasm_rust::Output<
            Option<super::super::types::cdn::EndpointGlobalDeliveryRule>,
        >,
        /// Indicates whether compression is to be enabled.
        pub is_compression_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies if http allowed. Defaults to `true`.
        pub is_http_allowed: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies if https allowed. Defaults to `true`.
        pub is_https_allowed: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the CDN Endpoint. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// What types of optimization should this CDN Endpoint optimize for? Possible values include `DynamicSiteAcceleration`, `GeneralMediaStreaming`, `GeneralWebDelivery`, `LargeFileDownload` and `VideoOnDemandMediaStreaming`.
        pub optimization_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The host header CDN provider will send along with content requests to origins.
        pub origin_host_header: pulumi_wasm_rust::Output<Option<String>>,
        /// The path used at for origin requests.
        pub origin_path: pulumi_wasm_rust::Output<Option<String>>,
        /// The set of origins of the CDN endpoint. When multiple origins exist, the first origin will be used as primary and rest will be used as failover options. Each `origin` block supports fields documented below. Changing this forces a new resource to be created.
        pub origins: pulumi_wasm_rust::Output<
            Vec<super::super::types::cdn::EndpointOrigin>,
        >,
        /// the path to a file hosted on the origin which helps accelerate delivery of the dynamic content and calculate the most optimal routes for the CDN. This is relative to the `origin_path`.
        ///
        /// > **NOTE:** `global_delivery_rule` and `delivery_rule` are currently only available for `Microsoft_Standard` CDN profiles.
        pub probe_path: pulumi_wasm_rust::Output<Option<String>>,
        /// The CDN Profile to which to attach the CDN Endpoint. Changing this forces a new resource to be created.
        pub profile_name: pulumi_wasm_rust::Output<String>,
        /// Sets query string caching behavior. Allowed values are `IgnoreQueryString`, `BypassCaching` and `UseQueryString`. `NotSet` value can be used for `Premium Verizon` CDN profile. Defaults to `IgnoreQueryString`.
        pub querystring_caching_behaviour: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the CDN Endpoint. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EndpointArgs) -> EndpointResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let content_types_to_compresses_binding = args
            .content_types_to_compresses
            .get_inner();
        let delivery_rules_binding = args.delivery_rules.get_inner();
        let geo_filters_binding = args.geo_filters.get_inner();
        let global_delivery_rule_binding = args.global_delivery_rule.get_inner();
        let is_compression_enabled_binding = args.is_compression_enabled.get_inner();
        let is_http_allowed_binding = args.is_http_allowed.get_inner();
        let is_https_allowed_binding = args.is_https_allowed.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let optimization_type_binding = args.optimization_type.get_inner();
        let origin_host_header_binding = args.origin_host_header.get_inner();
        let origin_path_binding = args.origin_path.get_inner();
        let origins_binding = args.origins.get_inner();
        let probe_path_binding = args.probe_path.get_inner();
        let profile_name_binding = args.profile_name.get_inner();
        let querystring_caching_behaviour_binding = args
            .querystring_caching_behaviour
            .get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:cdn/endpoint:Endpoint".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "contentTypesToCompresses".into(),
                    value: &content_types_to_compresses_binding,
                },
                register_interface::ObjectField {
                    name: "deliveryRules".into(),
                    value: &delivery_rules_binding,
                },
                register_interface::ObjectField {
                    name: "geoFilters".into(),
                    value: &geo_filters_binding,
                },
                register_interface::ObjectField {
                    name: "globalDeliveryRule".into(),
                    value: &global_delivery_rule_binding,
                },
                register_interface::ObjectField {
                    name: "isCompressionEnabled".into(),
                    value: &is_compression_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "isHttpAllowed".into(),
                    value: &is_http_allowed_binding,
                },
                register_interface::ObjectField {
                    name: "isHttpsAllowed".into(),
                    value: &is_https_allowed_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "optimizationType".into(),
                    value: &optimization_type_binding,
                },
                register_interface::ObjectField {
                    name: "originHostHeader".into(),
                    value: &origin_host_header_binding,
                },
                register_interface::ObjectField {
                    name: "originPath".into(),
                    value: &origin_path_binding,
                },
                register_interface::ObjectField {
                    name: "origins".into(),
                    value: &origins_binding,
                },
                register_interface::ObjectField {
                    name: "probePath".into(),
                    value: &probe_path_binding,
                },
                register_interface::ObjectField {
                    name: "profileName".into(),
                    value: &profile_name_binding,
                },
                register_interface::ObjectField {
                    name: "querystringCachingBehaviour".into(),
                    value: &querystring_caching_behaviour_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "contentTypesToCompresses".into(),
                },
                register_interface::ResultField {
                    name: "deliveryRules".into(),
                },
                register_interface::ResultField {
                    name: "fqdn".into(),
                },
                register_interface::ResultField {
                    name: "geoFilters".into(),
                },
                register_interface::ResultField {
                    name: "globalDeliveryRule".into(),
                },
                register_interface::ResultField {
                    name: "isCompressionEnabled".into(),
                },
                register_interface::ResultField {
                    name: "isHttpAllowed".into(),
                },
                register_interface::ResultField {
                    name: "isHttpsAllowed".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "optimizationType".into(),
                },
                register_interface::ResultField {
                    name: "originHostHeader".into(),
                },
                register_interface::ResultField {
                    name: "originPath".into(),
                },
                register_interface::ResultField {
                    name: "origins".into(),
                },
                register_interface::ResultField {
                    name: "probePath".into(),
                },
                register_interface::ResultField {
                    name: "profileName".into(),
                },
                register_interface::ResultField {
                    name: "querystringCachingBehaviour".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EndpointResult {
            content_types_to_compresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contentTypesToCompresses").unwrap(),
            ),
            delivery_rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deliveryRules").unwrap(),
            ),
            fqdn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fqdn").unwrap(),
            ),
            geo_filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("geoFilters").unwrap(),
            ),
            global_delivery_rule: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("globalDeliveryRule").unwrap(),
            ),
            is_compression_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("isCompressionEnabled").unwrap(),
            ),
            is_http_allowed: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("isHttpAllowed").unwrap(),
            ),
            is_https_allowed: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("isHttpsAllowed").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            optimization_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("optimizationType").unwrap(),
            ),
            origin_host_header: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("originHostHeader").unwrap(),
            ),
            origin_path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("originPath").unwrap(),
            ),
            origins: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("origins").unwrap(),
            ),
            probe_path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("probePath").unwrap(),
            ),
            profile_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("profileName").unwrap(),
            ),
            querystring_caching_behaviour: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("querystringCachingBehaviour").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
