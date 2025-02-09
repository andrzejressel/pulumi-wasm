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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetRepositoryArgs,
    ) -> GetRepositoryResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let repository_name_binding_1 = args.repository_name.get_output(context);
        let repository_name_binding = repository_name_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:codecommit/getRepository:getRepository".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "repositoryName".into(),
                    value: &repository_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetRepositoryResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            clone_url_http: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cloneUrlHttp"),
            ),
            clone_url_ssh: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cloneUrlSsh"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            kms_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsKeyId"),
            ),
            repository_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("repositoryId"),
            ),
            repository_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("repositoryName"),
            ),
        }
    }
}
