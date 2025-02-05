/// Provides a CodeArtifact Domains Permissions Policy Resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:kms:Key
///     properties:
///       description: domain key
///   exampleDomain:
///     type: aws:codeartifact:Domain
///     name: example
///     properties:
///       domain: example
///       encryptionKey: ${example.arn}
///   testDomainPermissions:
///     type: aws:codeartifact:DomainPermissions
///     name: test
///     properties:
///       domain: ${exampleDomain.domain}
///       policyDocument: ${test.json}
/// variables:
///   test:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: '*'
///                 identifiers:
///                   - '*'
///             actions:
///               - codeartifact:CreateRepository
///             resources:
///               - ${exampleDomain.arn}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CodeArtifact Domain Permissions Policies using the CodeArtifact Domain ARN. For example:
///
/// ```sh
/// $ pulumi import aws:codeartifact/domainPermissions:DomainPermissions example arn:aws:codeartifact:us-west-2:012345678912:domain/tf-acc-test-1928056699409417367
/// ```
pub mod domain_permissions {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainPermissionsArgs {
        /// The name of the domain on which to set the resource policy.
        #[builder(into)]
        pub domain: pulumi_wasm_rust::InputOrOutput<String>,
        /// The account number of the AWS account that owns the domain.
        #[builder(into, default)]
        pub domain_owner: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A JSON policy string to be set as the access control resource policy on the provided domain.
        #[builder(into)]
        pub policy_document: pulumi_wasm_rust::InputOrOutput<String>,
        /// The current revision of the resource policy to be set. This revision is used for optimistic locking, which prevents others from overwriting your changes to the domain's resource policy.
        #[builder(into, default)]
        pub policy_revision: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DomainPermissionsResult {
        /// The name of the domain on which to set the resource policy.
        pub domain: pulumi_wasm_rust::Output<String>,
        /// The account number of the AWS account that owns the domain.
        pub domain_owner: pulumi_wasm_rust::Output<String>,
        /// A JSON policy string to be set as the access control resource policy on the provided domain.
        pub policy_document: pulumi_wasm_rust::Output<String>,
        /// The current revision of the resource policy to be set. This revision is used for optimistic locking, which prevents others from overwriting your changes to the domain's resource policy.
        pub policy_revision: pulumi_wasm_rust::Output<String>,
        /// The ARN of the resource associated with the resource policy.
        pub resource_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: DomainPermissionsArgs,
    ) -> DomainPermissionsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let domain_binding = args.domain.get_output(context).get_inner();
        let domain_owner_binding = args.domain_owner.get_output(context).get_inner();
        let policy_document_binding = args
            .policy_document
            .get_output(context)
            .get_inner();
        let policy_revision_binding = args
            .policy_revision
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:codeartifact/domainPermissions:DomainPermissions".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "domain".into(),
                    value: &domain_binding,
                },
                register_interface::ObjectField {
                    name: "domainOwner".into(),
                    value: &domain_owner_binding,
                },
                register_interface::ObjectField {
                    name: "policyDocument".into(),
                    value: &policy_document_binding,
                },
                register_interface::ObjectField {
                    name: "policyRevision".into(),
                    value: &policy_revision_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DomainPermissionsResult {
            domain: pulumi_wasm_rust::__private::into_domain(o.extract_field("domain")),
            domain_owner: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("domainOwner"),
            ),
            policy_document: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("policyDocument"),
            ),
            policy_revision: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("policyRevision"),
            ),
            resource_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceArn"),
            ),
        }
    }
}
