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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod url_list {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UrlListArgs {
        /// Free-text description of the resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The location of the url lists.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Short name of the UrlList resource to be created.
        /// This value should be 1-63 characters long, containing only letters, numbers, hyphens, and underscores, and should not start with a number. E.g. 'urlList'.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// FQDNs and URLs.
        #[builder(into)]
        pub values: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
    }
    #[allow(dead_code)]
    pub struct UrlListResult {
        /// Output only. Time when the security policy was created.
        /// A timestamp in RFC3339 UTC 'Zulu' format, with nanosecond resolution and up to nine fractional digits.
        /// Examples: '2014-10-02T15:01:23Z' and '2014-10-02T15:01:23.045123456Z'
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Free-text description of the resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The location of the url lists.
        ///
        ///
        /// - - -
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Short name of the UrlList resource to be created.
        /// This value should be 1-63 characters long, containing only letters, numbers, hyphens, and underscores, and should not start with a number. E.g. 'urlList'.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Output only. Time when the security policy was updated.
        /// A timestamp in RFC3339 UTC 'Zulu' format, with nanosecond resolution and up to nine fractional digits.
        /// Examples: '2014-10-02T15:01:23Z' and '2014-10-02T15:01:23.045123456Z'.
        pub update_time: pulumi_gestalt_rust::Output<String>,
        /// FQDNs and URLs.
        pub values: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: UrlListArgs,
    ) -> UrlListResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let values_binding = args.values.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:networksecurity/urlList:UrlList".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "values".into(),
                    value: values_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        UrlListResult {
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            update_time: o.get_field("updateTime"),
            values: o.get_field("values"),
        }
    }
}
