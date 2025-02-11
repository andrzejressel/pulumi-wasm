/// Describes a view over log entries in a bucket.
///
///
/// To get more information about LogView, see:
///
/// * [API documentation](https://cloud.google.com/logging/docs/reference/v2/rest/v2/projects.locations.buckets.views)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/logging/docs/apis)
///
/// ## Example Usage
///
/// ### Logging Log View Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let loggingLogView = project_bucket_config::create(
///         "loggingLogView",
///         ProjectBucketConfigArgs::builder()
///             .bucket_id("_Default")
///             .location("global")
///             .project("my-project-name")
///             .retention_days(30)
///             .build_struct(),
///     );
///     let loggingLogViewLogView = log_view::create(
///         "loggingLogViewLogView",
///         LogViewArgs::builder()
///             .bucket("${loggingLogView.id}")
///             .description("A logging view configured with Terraform")
///             .filter(
///                 "SOURCE(\"projects/myproject\") AND resource.type = \"gce_instance\" AND LOG_ID(\"stdout\")",
///             )
///             .name("my-view")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// LogView can be imported using any of these accepted formats:
///
/// * `{{parent}}/locations/{{location}}/buckets/{{bucket}}/views/{{name}}`
///
/// When using the `pulumi import` command, LogView can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:logging/logView:LogView default {{parent}}/locations/{{location}}/buckets/{{bucket}}/views/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod log_view {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LogViewArgs {
        /// The bucket of the resource
        ///
        ///
        /// - - -
        #[builder(into)]
        pub bucket: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Describes this view.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Filter that restricts which log entries in a bucket are visible in this view. Filters are restricted to be a logical AND of ==/!= of any of the following: - originating project/folder/organization/billing account. - resource type - log id For example: SOURCE("projects/myproject") AND resource.type = "gce_instance" AND LOG_ID("stdout")
        #[builder(into, default)]
        pub filter: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The location of the resource. The supported locations are: global, us-central1, us-east1, us-west1, asia-east1, europe-west1.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The resource name of the view. For example: \`projects/my-project/locations/global/buckets/my-bucket/views/my-view\`
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The parent of the resource.
        #[builder(into, default)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct LogViewResult {
        /// The bucket of the resource
        ///
        ///
        /// - - -
        pub bucket: pulumi_gestalt_rust::Output<String>,
        /// Output only. The creation timestamp of the view.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Describes this view.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Filter that restricts which log entries in a bucket are visible in this view. Filters are restricted to be a logical AND of ==/!= of any of the following: - originating project/folder/organization/billing account. - resource type - log id For example: SOURCE("projects/myproject") AND resource.type = "gce_instance" AND LOG_ID("stdout")
        pub filter: pulumi_gestalt_rust::Output<Option<String>>,
        /// The location of the resource. The supported locations are: global, us-central1, us-east1, us-west1, asia-east1, europe-west1.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The resource name of the view. For example: \`projects/my-project/locations/global/buckets/my-bucket/views/my-view\`
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The parent of the resource.
        pub parent: pulumi_gestalt_rust::Output<String>,
        /// Output only. The last update timestamp of the view.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LogViewArgs,
    ) -> LogViewResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bucket_binding = args.bucket.get_output(context);
        let description_binding = args.description.get_output(context);
        let filter_binding = args.filter.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let parent_binding = args.parent.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:logging/logView:LogView".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bucket".into(),
                    value: &bucket_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filter".into(),
                    value: &filter_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parent".into(),
                    value: &parent_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        LogViewResult {
            bucket: o.get_field("bucket"),
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            filter: o.get_field("filter"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            parent: o.get_field("parent"),
            update_time: o.get_field("updateTime"),
        }
    }
}
