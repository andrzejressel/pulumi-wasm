/// TargetSite represents a URI pattern that the users want to confine their
/// search.
///
///
/// To get more information about TargetSite, see:
///
/// * [API documentation](https://cloud.google.com/generative-ai-app-builder/docs/reference/rest/v1/projects.locations.collections.dataStores.siteSearchEngine.targetSites)
///
/// ## Example Usage
///
/// ### Discoveryengine Targetsite Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let basic = target_site::create(
///         "basic",
///         TargetSiteArgs::builder()
///             .data_store_id("${basicDataStore.dataStoreId}")
///             .exact_match(false)
///             .location("${basicDataStore.location}")
///             .provided_uri_pattern("http://cloud.google.com/docs/*")
///             .type_("INCLUDE")
///             .build_struct(),
///     );
///     let basicDataStore = data_store::create(
///         "basicDataStore",
///         DataStoreArgs::builder()
///             .content_config("PUBLIC_WEBSITE")
///             .create_advanced_site_search(false)
///             .data_store_id("data-store-id")
///             .display_name("tf-test-basic-site-search-datastore")
///             .industry_vertical("GENERIC")
///             .location("global")
///             .skip_default_schema_creation(false)
///             .solution_types(vec!["SOLUTION_TYPE_SEARCH",])
///             .build_struct(),
///     );
/// }
/// ```
/// ### Discoveryengine Targetsite Advanced
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let advanced = target_site::create(
///         "advanced",
///         TargetSiteArgs::builder()
///             .data_store_id("${advancedDataStore.dataStoreId}")
///             .exact_match(false)
///             .location("${advancedDataStore.location}")
///             .provided_uri_pattern("http://cloud.google.com/docs/*")
///             .type_("INCLUDE")
///             .build_struct(),
///     );
///     let advancedDataStore = data_store::create(
///         "advancedDataStore",
///         DataStoreArgs::builder()
///             .content_config("PUBLIC_WEBSITE")
///             .create_advanced_site_search(true)
///             .data_store_id("data-store-id")
///             .display_name("tf-test-advanced-site-search-datastore")
///             .industry_vertical("GENERIC")
///             .location("global")
///             .skip_default_schema_creation(false)
///             .solution_types(vec!["SOLUTION_TYPE_SEARCH",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// TargetSite can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/collections/default_collection/dataStores/{{data_store_id}}/siteSearchEngine/targetSites/{{target_site_id}}`
///
/// * `{{project}}/{{location}}/{{data_store_id}}/{{target_site_id}}`
///
/// * `{{location}}/{{data_store_id}}/{{target_site_id}}`
///
/// When using the `pulumi import` command, TargetSite can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:discoveryengine/targetSite:TargetSite default projects/{{project}}/locations/{{location}}/collections/default_collection/dataStores/{{data_store_id}}/siteSearchEngine/targetSites/{{target_site_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:discoveryengine/targetSite:TargetSite default {{project}}/{{location}}/{{data_store_id}}/{{target_site_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:discoveryengine/targetSite:TargetSite default {{location}}/{{data_store_id}}/{{target_site_id}}
/// ```
///
pub mod target_site {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TargetSiteArgs {
        /// The unique id of the data store.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub data_store_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// If set to false, a uri_pattern is generated to include all pages whose
        /// address contains the provided_uri_pattern. If set to true, an uri_pattern
        /// is generated to try to be an exact match of the provided_uri_pattern or
        /// just the specific page if the provided_uri_pattern is a specific one.
        /// provided_uri_pattern is always normalized to generate the URI pattern to
        /// be used by the search engine.
        #[builder(into, default)]
        pub exact_match: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The geographic location where the data store should reside. The value can
        /// only be one of "global", "us" and "eu".
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The user provided URI pattern from which the `generated_uri_pattern` is
        /// generated.
        #[builder(into)]
        pub provided_uri_pattern: pulumi_wasm_rust::InputOrOutput<String>,
        /// The possible target site types.
        /// Possible values are: `INCLUDE`, `EXCLUDE`.
        #[builder(into, default)]
        pub type_: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct TargetSiteResult {
        /// The unique id of the data store.
        ///
        ///
        /// - - -
        pub data_store_id: pulumi_wasm_rust::Output<String>,
        /// If set to false, a uri_pattern is generated to include all pages whose
        /// address contains the provided_uri_pattern. If set to true, an uri_pattern
        /// is generated to try to be an exact match of the provided_uri_pattern or
        /// just the specific page if the provided_uri_pattern is a specific one.
        /// provided_uri_pattern is always normalized to generate the URI pattern to
        /// be used by the search engine.
        pub exact_match: pulumi_wasm_rust::Output<Option<bool>>,
        /// Site search indexing failure reasons.
        /// Structure is documented below.
        pub failure_reasons: pulumi_wasm_rust::Output<
            Vec<super::super::types::discoveryengine::TargetSiteFailureReason>,
        >,
        /// This is system-generated based on the `provided_uri_pattern`.
        pub generated_uri_pattern: pulumi_wasm_rust::Output<String>,
        /// The indexing status.
        pub indexing_status: pulumi_wasm_rust::Output<String>,
        /// The geographic location where the data store should reside. The value can
        /// only be one of "global", "us" and "eu".
        pub location: pulumi_wasm_rust::Output<String>,
        /// The unique full resource name of the target site. Values are of the format
        /// `projects/{project}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}/siteSearchEngine/targetSites/{target_site_id}`.
        /// This field must be a UTF-8 encoded string with a length limit of 1024
        /// characters.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The user provided URI pattern from which the `generated_uri_pattern` is
        /// generated.
        pub provided_uri_pattern: pulumi_wasm_rust::Output<String>,
        /// Root domain of the `provided_uri_pattern`.
        pub root_domain_uri: pulumi_wasm_rust::Output<String>,
        /// Site ownership and validity verification status.
        /// Structure is documented below.
        pub site_verification_infos: pulumi_wasm_rust::Output<
            Vec<super::super::types::discoveryengine::TargetSiteSiteVerificationInfo>,
        >,
        /// The unique id of the target site.
        pub target_site_id: pulumi_wasm_rust::Output<String>,
        /// The possible target site types.
        /// Possible values are: `INCLUDE`, `EXCLUDE`.
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
        /// The target site's last updated time.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: TargetSiteArgs,
    ) -> TargetSiteResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let data_store_id_binding = args.data_store_id.get_output(context).get_inner();
        let exact_match_binding = args.exact_match.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let provided_uri_pattern_binding = args
            .provided_uri_pattern
            .get_output(context)
            .get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:discoveryengine/targetSite:TargetSite".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dataStoreId".into(),
                    value: &data_store_id_binding,
                },
                register_interface::ObjectField {
                    name: "exactMatch".into(),
                    value: &exact_match_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "providedUriPattern".into(),
                    value: &provided_uri_pattern_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        TargetSiteResult {
            data_store_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dataStoreId"),
            ),
            exact_match: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("exactMatch"),
            ),
            failure_reasons: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("failureReasons"),
            ),
            generated_uri_pattern: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("generatedUriPattern"),
            ),
            indexing_status: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("indexingStatus"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            provided_uri_pattern: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("providedUriPattern"),
            ),
            root_domain_uri: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("rootDomainUri"),
            ),
            site_verification_infos: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("siteVerificationInfos"),
            ),
            target_site_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("targetSiteId"),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(o.extract_field("type")),
            update_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
