#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_repository {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRepositoryArgs {
        /// Name for the repository. This needs to be less than 100 characters.
        #[builder(into)]
        pub repository_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetRepositoryResult {
        /// ARN of the repository.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// URL to use for cloning the repository over HTTPS.
        pub clone_url_http: pulumi_gestalt_rust::Output<String>,
        /// URL to use for cloning the repository over SSH.
        pub clone_url_ssh: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the encryption key.
        pub kms_key_id: pulumi_gestalt_rust::Output<String>,
        /// ID of the repository.
        pub repository_id: pulumi_gestalt_rust::Output<String>,
        pub repository_name: pulumi_gestalt_rust::Output<String>,
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
        let repository_name_binding = args.repository_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:codecommit/getRepository:getRepository".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "repositoryName".into(),
                    value: &repository_name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetRepositoryResult {
            arn: o.get_field("arn"),
            clone_url_http: o.get_field("cloneUrlHttp"),
            clone_url_ssh: o.get_field("cloneUrlSsh"),
            id: o.get_field("id"),
            kms_key_id: o.get_field("kmsKeyId"),
            repository_id: o.get_field("repositoryId"),
            repository_name: o.get_field("repositoryName"),
        }
    }
}
