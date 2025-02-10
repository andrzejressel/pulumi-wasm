/// Provides a CodeDeploy application to be used as a basis for deployments
///
/// ## Example Usage
///
/// ### ECS Application
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = application::create(
///         "example",
///         ApplicationArgs::builder().compute_platform("ECS").name("example").build_struct(),
///     );
/// }
/// ```
///
/// ### Lambda Application
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = application::create(
///         "example",
///         ApplicationArgs::builder()
///             .compute_platform("Lambda")
///             .name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Server Application
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = application::create(
///         "example",
///         ApplicationArgs::builder()
///             .compute_platform("Server")
///             .name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CodeDeploy Applications using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:codedeploy/application:Application example my-application
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod application {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApplicationArgs {
        /// The compute platform can either be `ECS`, `Lambda`, or `Server`. Default is `Server`.
        #[builder(into, default)]
        pub compute_platform: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the application.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ApplicationResult {
        /// The application ID.
        pub application_id: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the CodeDeploy application.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The compute platform can either be `ECS`, `Lambda`, or `Server`. Default is `Server`.
        pub compute_platform: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name for a connection to a GitHub account.
        pub github_account_name: pulumi_gestalt_rust::Output<String>,
        /// Whether the user has authenticated with GitHub for the specified application.
        pub linked_to_github: pulumi_gestalt_rust::Output<bool>,
        /// The name of the application.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ApplicationArgs,
    ) -> ApplicationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let compute_platform_binding = args.compute_platform.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:codedeploy/application:Application".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "computePlatform".into(),
                    value: compute_platform_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ApplicationResult {
            application_id: o.get_field("applicationId"),
            arn: o.get_field("arn"),
            compute_platform: o.get_field("computePlatform"),
            github_account_name: o.get_field("githubAccountName"),
            linked_to_github: o.get_field("linkedToGithub"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
