/// Resource for managing an AWS Global Accelerator Cross Account Attachment.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = cross_account_attachment::create(
///         "example",
///         CrossAccountAttachmentArgs::builder()
///             .name("example-cross-account-attachment")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Usage with Optional Arguments
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = cross_account_attachment::create(
///         "example",
///         CrossAccountAttachmentArgs::builder()
///             .name("example-cross-account-attachment")
///             .principals(vec!["123456789012",])
///             .resources(
///                 vec![
///                     CrossAccountAttachmentResource::builder()
///                     .endpointId("arn:aws:elasticloadbalancing:us-west-2:123456789012:loadbalancer/app/my-load-balancer/50dc6c495c0c9188")
///                     .region("us-west-2").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Global Accelerator Cross Account Attachment using the `example_id_arg`. For example:
///
/// ```sh
/// $ pulumi import aws:globalaccelerator/crossAccountAttachment:CrossAccountAttachment example arn:aws:globalaccelerator::012345678910:attachment/01234567-abcd-8910-efgh-123456789012
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cross_account_attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CrossAccountAttachmentArgs {
        /// Name of the Cross Account Attachment.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of AWS account IDs that are allowed to associate resources with the accelerator.
        #[builder(into, default)]
        pub principals: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// List of resources to be associated with the accelerator.
        #[builder(into, default)]
        pub resources: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::globalaccelerator::CrossAccountAttachmentResource,
                >,
            >,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct CrossAccountAttachmentResult {
        /// ARN of the Cross Account Attachment.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Creation Time when the Cross Account Attachment.
        pub created_time: pulumi_gestalt_rust::Output<String>,
        /// Last modified time of the Cross Account Attachment.
        pub last_modified_time: pulumi_gestalt_rust::Output<String>,
        /// Name of the Cross Account Attachment.
        ///
        /// The following arguments are optional:
        pub name: pulumi_gestalt_rust::Output<String>,
        /// List of AWS account IDs that are allowed to associate resources with the accelerator.
        pub principals: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// List of resources to be associated with the accelerator.
        pub resources: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::globalaccelerator::CrossAccountAttachmentResource,
                >,
            >,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
        args: CrossAccountAttachmentArgs,
    ) -> CrossAccountAttachmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let principals_binding = args.principals.get_output(context);
        let resources_binding = args.resources.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:globalaccelerator/crossAccountAttachment:CrossAccountAttachment"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "principals".into(),
                    value: &principals_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resources".into(),
                    value: &resources_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        CrossAccountAttachmentResult {
            arn: o.get_field("arn"),
            created_time: o.get_field("createdTime"),
            last_modified_time: o.get_field("lastModifiedTime"),
            name: o.get_field("name"),
            principals: o.get_field("principals"),
            resources: o.get_field("resources"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
