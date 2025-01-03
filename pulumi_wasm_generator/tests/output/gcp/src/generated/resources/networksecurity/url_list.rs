/// UrlList proto helps users to set reusable, independently manageable lists of hosts, host patterns, URLs, URL patterns.
///
///
/// To get more information about UrlLists, see:
///
/// * [API documentation](https://cloud.google.com/secure-web-proxy/docs/reference/network-security/rest/v1/projects.locations.urlLists)
/// * How-to Guides
///     * Use UrlLists
///
/// ## Example Usage
///
/// ### Network Security Url Lists Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = url_list::create(
///         "default",
///         UrlListArgs::builder()
///             .location("us-central1")
///             .name("my-url-lists")
///             .values(vec!["www.example.com",])
///             .build_struct(),
///     );
/// }
/// ```
/// ### Network Security Url Lists Advanced
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = url_list::create(
///         "default",
///         UrlListArgs::builder()
///             .description("my description")
///             .location("us-central1")
///             .name("my-url-lists")
///             .values(
///                 vec!["www.example.com", "about.example.com", "github.com/example-org/*",],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// UrlLists can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/urlLists/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, UrlLists can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:networksecurity/urlList:UrlList default projects/{{project}}/locations/{{location}}/urlLists/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networksecurity/urlList:UrlList default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networksecurity/urlList:UrlList default {{location}}/{{name}}
/// ```
///
pub mod url_list {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UrlListArgs {
        /// Free-text description of the resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The location of the url lists.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// Short name of the UrlList resource to be created.
        /// This value should be 1-63 characters long, containing only letters, numbers, hyphens, and underscores, and should not start with a number. E.g. 'urlList'.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// FQDNs and URLs.
        #[builder(into)]
        pub values: pulumi_wasm_rust::Output<Vec<String>>,
    }
    #[allow(dead_code)]
    pub struct UrlListResult {
        /// Output only. Time when the security policy was created.
        /// A timestamp in RFC3339 UTC 'Zulu' format, with nanosecond resolution and up to nine fractional digits.
        /// Examples: '2014-10-02T15:01:23Z' and '2014-10-02T15:01:23.045123456Z'
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Free-text description of the resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The location of the url lists.
        ///
        ///
        /// - - -
        pub location: pulumi_wasm_rust::Output<String>,
        /// Short name of the UrlList resource to be created.
        /// This value should be 1-63 characters long, containing only letters, numbers, hyphens, and underscores, and should not start with a number. E.g. 'urlList'.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// Output only. Time when the security policy was updated.
        /// A timestamp in RFC3339 UTC 'Zulu' format, with nanosecond resolution and up to nine fractional digits.
        /// Examples: '2014-10-02T15:01:23Z' and '2014-10-02T15:01:23.045123456Z'.
        pub update_time: pulumi_wasm_rust::Output<String>,
        /// FQDNs and URLs.
        pub values: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: UrlListArgs) -> UrlListResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let values_binding = args.values.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:networksecurity/urlList:UrlList".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
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
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "values".into(),
                    value: &values_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
                register_interface::ResultField {
                    name: "values".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        UrlListResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
            values: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("values").unwrap(),
            ),
        }
    }
}
