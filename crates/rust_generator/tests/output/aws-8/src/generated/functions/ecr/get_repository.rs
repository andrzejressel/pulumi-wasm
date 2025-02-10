#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_repository {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRepositoryArgs {
        /// Name of the ECR Repository.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Registry ID where the repository was created.
        #[builder(into, default)]
        pub registry_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags assigned to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetRepositoryResult {
        /// Full ARN of the repository.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Encryption configuration for the repository. See Encryption Configuration below.
        pub encryption_configurations: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ecr::GetRepositoryEncryptionConfiguration>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Configuration block that defines image scanning configuration for the repository. See Image Scanning Configuration below.
        pub image_scanning_configurations: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ecr::GetRepositoryImageScanningConfiguration>,
        >,
        /// The tag mutability setting for the repository.
        pub image_tag_mutability: pulumi_gestalt_rust::Output<String>,
        /// List of image tags associated with the most recently pushed image in the repository.
        pub most_recent_image_tags: pulumi_gestalt_rust::Output<Vec<String>>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub registry_id: pulumi_gestalt_rust::Output<String>,
        /// URL of the repository (in the form `aws_account_id.dkr.ecr.region.amazonaws.com/repositoryName`).
        pub repository_url: pulumi_gestalt_rust::Output<String>,
        /// Map of tags assigned to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetRepositoryArgs,
    ) -> GetRepositoryResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let registry_id_binding = args.registry_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ecr/getRepository:getRepository".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "registryId".into(),
                    value: registry_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetRepositoryResult {
            arn: o.get_field("arn"),
            encryption_configurations: o.get_field("encryptionConfigurations"),
            id: o.get_field("id"),
            image_scanning_configurations: o.get_field("imageScanningConfigurations"),
            image_tag_mutability: o.get_field("imageTagMutability"),
            most_recent_image_tags: o.get_field("mostRecentImageTags"),
            name: o.get_field("name"),
            registry_id: o.get_field("registryId"),
            repository_url: o.get_field("repositoryUrl"),
            tags: o.get_field("tags"),
        }
    }
}
