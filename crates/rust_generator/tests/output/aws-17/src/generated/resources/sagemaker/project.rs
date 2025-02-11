/// Provides a SageMaker Project resource.
///
///  > Note: If you are trying to use SageMaker projects with SageMaker studio you will need to add a tag with the key `sagemaker:studio-visibility` with value `true`. For more on requirements to use projects and permission needed see [AWS Docs](https://docs.aws.amazon.com/sagemaker/latest/dg/sagemaker-projects-templates-custom.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = project::create(
///         "example",
///         ProjectArgs::builder()
///             .project_name("example")
///             .service_catalog_provisioning_details(
///                 ProjectServiceCatalogProvisioningDetails::builder()
///                     .productId("${exampleAwsServicecatalogProduct.id}")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SageMaker Projects using the `project_name`. For example:
///
/// ```sh
/// $ pulumi import aws:sagemaker/project:Project example example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod project {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProjectArgs {
        /// A description for the project.
        #[builder(into, default)]
        pub project_description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Project.
        #[builder(into)]
        pub project_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The product ID and provisioning artifact ID to provision a service catalog. See Service Catalog Provisioning Details below.
        #[builder(into)]
        pub service_catalog_provisioning_details: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::sagemaker::ProjectServiceCatalogProvisioningDetails,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ProjectResult {
        /// The Amazon Resource Name (ARN) assigned by AWS to this Project.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A description for the project.
        pub project_description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the project.
        pub project_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the Project.
        pub project_name: pulumi_gestalt_rust::Output<String>,
        /// The product ID and provisioning artifact ID to provision a service catalog. See Service Catalog Provisioning Details below.
        pub service_catalog_provisioning_details: pulumi_gestalt_rust::Output<
            super::super::types::sagemaker::ProjectServiceCatalogProvisioningDetails,
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
        args: ProjectArgs,
    ) -> ProjectResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let project_description_binding = args.project_description.get_output(context);
        let project_name_binding = args.project_name.get_output(context);
        let service_catalog_provisioning_details_binding = args
            .service_catalog_provisioning_details
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:sagemaker/project:Project".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "projectDescription".into(),
                    value: &project_description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "projectName".into(),
                    value: &project_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceCatalogProvisioningDetails".into(),
                    value: &service_catalog_provisioning_details_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ProjectResult {
            arn: o.get_field("arn"),
            project_description: o.get_field("projectDescription"),
            project_id: o.get_field("projectId"),
            project_name: o.get_field("projectName"),
            service_catalog_provisioning_details: o
                .get_field("serviceCatalogProvisioningDetails"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
