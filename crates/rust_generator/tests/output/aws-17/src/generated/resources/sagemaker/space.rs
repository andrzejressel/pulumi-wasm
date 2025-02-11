/// Provides a SageMaker Space resource.
///
/// ## Example Usage
///
/// ### Basic usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = space::create(
///         "example",
///         SpaceArgs::builder().domain_id("${test.id}").space_name("example").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SageMaker Spaces using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:sagemaker/space:Space test_space arn:aws:sagemaker:us-west-2:123456789012:space/domain-id/space-name
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod space {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpaceArgs {
        /// The ID of the associated Domain.
        #[builder(into)]
        pub domain_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A collection of ownership settings. Required if `space_sharing_settings` is set. See `ownership_settings` Block below.
        #[builder(into, default)]
        pub ownership_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::sagemaker::SpaceOwnershipSettings>,
        >,
        /// The name of the space that appears in the SageMaker Studio UI.
        #[builder(into, default)]
        pub space_display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the space.
        #[builder(into)]
        pub space_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A collection of space settings. See `space_settings` Block below.
        #[builder(into, default)]
        pub space_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::sagemaker::SpaceSpaceSettings>,
        >,
        /// A collection of space sharing settings. Required if `ownership_settings` is set. See `space_sharing_settings` Block below.
        #[builder(into, default)]
        pub space_sharing_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::sagemaker::SpaceSpaceSharingSettings>,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct SpaceResult {
        /// The space's Amazon Resource Name (ARN).
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The ID of the associated Domain.
        pub domain_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the space's profile in the Amazon Elastic File System volume.
        pub home_efs_file_system_uid: pulumi_gestalt_rust::Output<String>,
        /// A collection of ownership settings. Required if `space_sharing_settings` is set. See `ownership_settings` Block below.
        pub ownership_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::sagemaker::SpaceOwnershipSettings>,
        >,
        /// The name of the space that appears in the SageMaker Studio UI.
        pub space_display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the space.
        pub space_name: pulumi_gestalt_rust::Output<String>,
        /// A collection of space settings. See `space_settings` Block below.
        pub space_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::sagemaker::SpaceSpaceSettings>,
        >,
        /// A collection of space sharing settings. Required if `ownership_settings` is set. See `space_sharing_settings` Block below.
        pub space_sharing_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::sagemaker::SpaceSpaceSharingSettings>,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Returns the URL of the space. If the space is created with Amazon Web Services IAM Identity Center (Successor to Amazon Web Services Single Sign-On) authentication, users can navigate to the URL after appending the respective redirect parameter for the application type to be federated through Amazon Web Services IAM Identity Center.
        pub url: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SpaceArgs,
    ) -> SpaceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let domain_id_binding = args.domain_id.get_output(context);
        let ownership_settings_binding = args.ownership_settings.get_output(context);
        let space_display_name_binding = args.space_display_name.get_output(context);
        let space_name_binding = args.space_name.get_output(context);
        let space_settings_binding = args.space_settings.get_output(context);
        let space_sharing_settings_binding = args
            .space_sharing_settings
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:sagemaker/space:Space".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainId".into(),
                    value: &domain_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ownershipSettings".into(),
                    value: &ownership_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "spaceDisplayName".into(),
                    value: &space_display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "spaceName".into(),
                    value: &space_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "spaceSettings".into(),
                    value: &space_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "spaceSharingSettings".into(),
                    value: &space_sharing_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SpaceResult {
            arn: o.get_field("arn"),
            domain_id: o.get_field("domainId"),
            home_efs_file_system_uid: o.get_field("homeEfsFileSystemUid"),
            ownership_settings: o.get_field("ownershipSettings"),
            space_display_name: o.get_field("spaceDisplayName"),
            space_name: o.get_field("spaceName"),
            space_settings: o.get_field("spaceSettings"),
            space_sharing_settings: o.get_field("spaceSharingSettings"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            url: o.get_field("url"),
        }
    }
}
