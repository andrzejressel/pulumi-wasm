/// Resource for managing an AWS CodeGuru Reviewer Repository Association.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = key::create("example", KeyArgs::builder().build_struct());
///     let exampleRepository = repository::create(
///         "exampleRepository",
///         RepositoryArgs::builder().repository_name("example-repo").build_struct(),
///     );
///     let exampleRepositoryAssociation = repository_association::create(
///         "exampleRepositoryAssociation",
///         RepositoryAssociationArgs::builder()
///             .kms_key_details(
///                 RepositoryAssociationKmsKeyDetails::builder()
///                     .encryptionOption("CUSTOMER_MANAGED_CMK")
///                     .kmsKeyId("${example.keyId}")
///                     .build_struct(),
///             )
///             .repository(
///                 RepositoryAssociationRepository::builder()
///                     .codecommit(
///                         RepositoryAssociationRepositoryCodecommit::builder()
///                             .name("${exampleRepository.repositoryName}")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
pub mod repository_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RepositoryAssociationArgs {
        /// An object describing the KMS key to asssociate. Block is documented below.
        #[builder(into, default)]
        pub kms_key_details: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::codegurureviewer::RepositoryAssociationKmsKeyDetails,
            >,
        >,
        /// An object describing the repository to associate. Valid values: `bitbucket`, `codecommit`, `github_enterprise_server`, or `s3_bucket`. Block is documented below. Note: for repositories that leverage CodeStar connections (ex. `bitbucket`, `github_enterprise_server`) the connection must be in `Available` status prior to creating this resource.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub repository: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::codegurureviewer::RepositoryAssociationRepository,
        >,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RepositoryAssociationResult {
        /// The Amazon Resource Name (ARN) identifying the repository association.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The ID of the repository association.
        pub association_id: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of an AWS CodeStar Connections connection.
        pub connection_arn: pulumi_gestalt_rust::Output<String>,
        /// An object describing the KMS key to asssociate. Block is documented below.
        pub kms_key_details: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::codegurureviewer::RepositoryAssociationKmsKeyDetails,
            >,
        >,
        /// The name of the repository.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The owner of the repository.
        pub owner: pulumi_gestalt_rust::Output<String>,
        /// The provider type of the repository association.
        pub provider_type: pulumi_gestalt_rust::Output<String>,
        /// An object describing the repository to associate. Valid values: `bitbucket`, `codecommit`, `github_enterprise_server`, or `s3_bucket`. Block is documented below. Note: for repositories that leverage CodeStar connections (ex. `bitbucket`, `github_enterprise_server`) the connection must be in `Available` status prior to creating this resource.
        ///
        /// The following arguments are optional:
        pub repository: pulumi_gestalt_rust::Output<
            super::super::types::codegurureviewer::RepositoryAssociationRepository,
        >,
        pub s3_repository_details: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::codegurureviewer::RepositoryAssociationS3RepositoryDetail,
            >,
        >,
        /// The state of the repository association.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// A description of why the repository association is in the current state.
        pub state_reason: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: RepositoryAssociationArgs,
    ) -> RepositoryAssociationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let kms_key_details_binding = args
            .kms_key_details
            .get_output(context)
            .get_inner();
        let repository_binding = args.repository.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:codegurureviewer/repositoryAssociation:RepositoryAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "kmsKeyDetails".into(),
                    value: &kms_key_details_binding,
                },
                register_interface::ObjectField {
                    name: "repository".into(),
                    value: &repository_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RepositoryAssociationResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            association_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("associationId"),
            ),
            connection_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("connectionArn"),
            ),
            kms_key_details: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsKeyDetails"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            owner: pulumi_gestalt_rust::__private::into_domain(o.extract_field("owner")),
            provider_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("providerType"),
            ),
            repository: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("repository"),
            ),
            s3_repository_details: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("s3RepositoryDetails"),
            ),
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
            state_reason: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("stateReason"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
