/// Resource for managing an AWS CodeGuru Reviewer Repository Association.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RepositoryAssociationArgs {
        /// An object describing the KMS key to asssociate. Block is documented below.
        #[builder(into, default)]
        pub kms_key_details: pulumi_wasm_rust::Output<
            Option<
                super::super::types::codegurureviewer::RepositoryAssociationKmsKeyDetails,
            >,
        >,
        /// An object describing the repository to associate. Valid values: `bitbucket`, `codecommit`, `github_enterprise_server`, or `s3_bucket`. Block is documented below. Note: for repositories that leverage CodeStar connections (ex. `bitbucket`, `github_enterprise_server`) the connection must be in `Available` status prior to creating this resource.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub repository: pulumi_wasm_rust::Output<
            super::super::types::codegurureviewer::RepositoryAssociationRepository,
        >,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RepositoryAssociationResult {
        /// The Amazon Resource Name (ARN) identifying the repository association.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The ID of the repository association.
        pub association_id: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of an AWS CodeStar Connections connection.
        pub connection_arn: pulumi_wasm_rust::Output<String>,
        /// An object describing the KMS key to asssociate. Block is documented below.
        pub kms_key_details: pulumi_wasm_rust::Output<
            Option<
                super::super::types::codegurureviewer::RepositoryAssociationKmsKeyDetails,
            >,
        >,
        /// The name of the repository.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The owner of the repository.
        pub owner: pulumi_wasm_rust::Output<String>,
        /// The provider type of the repository association.
        pub provider_type: pulumi_wasm_rust::Output<String>,
        /// An object describing the repository to associate. Valid values: `bitbucket`, `codecommit`, `github_enterprise_server`, or `s3_bucket`. Block is documented below. Note: for repositories that leverage CodeStar connections (ex. `bitbucket`, `github_enterprise_server`) the connection must be in `Available` status prior to creating this resource.
        ///
        /// The following arguments are optional:
        pub repository: pulumi_wasm_rust::Output<
            super::super::types::codegurureviewer::RepositoryAssociationRepository,
        >,
        pub s3_repository_details: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::codegurureviewer::RepositoryAssociationS3RepositoryDetail,
            >,
        >,
        /// The state of the repository association.
        pub state: pulumi_wasm_rust::Output<String>,
        /// A description of why the repository association is in the current state.
        pub state_reason: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: RepositoryAssociationArgs,
    ) -> RepositoryAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let kms_key_details_binding = args.kms_key_details.get_inner();
        let repository_binding = args.repository.get_inner();
        let tags_binding = args.tags.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "associationId".into(),
                },
                register_interface::ResultField {
                    name: "connectionArn".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyDetails".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "owner".into(),
                },
                register_interface::ResultField {
                    name: "providerType".into(),
                },
                register_interface::ResultField {
                    name: "repository".into(),
                },
                register_interface::ResultField {
                    name: "s3RepositoryDetails".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "stateReason".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RepositoryAssociationResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            association_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("associationId").unwrap(),
            ),
            connection_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionArn").unwrap(),
            ),
            kms_key_details: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyDetails").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            owner: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("owner").unwrap(),
            ),
            provider_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("providerType").unwrap(),
            ),
            repository: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("repository").unwrap(),
            ),
            s3_repository_details: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("s3RepositoryDetails").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            state_reason: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stateReason").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
