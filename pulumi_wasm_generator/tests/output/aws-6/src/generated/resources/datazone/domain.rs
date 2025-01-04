/// Resource for managing an AWS DataZone Domain.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   domainExecutionRole:
///     type: aws:iam:Role
///     name: domain_execution_role
///     properties:
///       name: my_domain_execution_role
///       assumeRolePolicy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Action:
///                 - sts:AssumeRole
///                 - sts:TagSession
///               Effect: Allow
///               Principal:
///                 Service: datazone.amazonaws.com
///             - Action:
///                 - sts:AssumeRole
///                 - sts:TagSession
///               Effect: Allow
///               Principal:
///                 Service: cloudformation.amazonaws.com
///       inlinePolicies:
///         - name: domain_execution_policy
///           policy:
///             fn::toJSON:
///               Version: 2012-10-17
///               Statement:
///                 - Action:
///                     - datazone:*
///                     - ram:*
///                     - sso:*
///                     - kms:*
///                   Effect: Allow
///                   Resource: '*'
///   example:
///     type: aws:datazone:Domain
///     properties:
///       name: example
///       domainExecutionRole: ${domainExecutionRole.arn}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import DataZone Domain using the `domain_id`. For example:
///
/// ```sh
/// $ pulumi import aws:datazone/domain:Domain example domain-id-12345678
/// ```
pub mod domain {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainArgs {
        /// Description of the Domain.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of the role used by DataZone to configure the Domain.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub domain_execution_role: pulumi_wasm_rust::Output<String>,
        /// ARN of the KMS key used to encrypt the Amazon DataZone domain, metadata and reporting data.
        #[builder(into, default)]
        pub kms_key_identifier: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the Domain.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Single sign on options, used to [enable AWS IAM Identity Center](https://docs.aws.amazon.com/datazone/latest/userguide/enable-IAM-identity-center-for-datazone.html) for DataZone.
        #[builder(into, default)]
        pub single_sign_on: pulumi_wasm_rust::Output<
            Option<super::super::types::datazone::DomainSingleSignOn>,
        >,
        /// Whether to skip the deletion check for the Domain.
        #[builder(into, default)]
        pub skip_deletion_check: pulumi_wasm_rust::Output<Option<bool>>,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::datazone::DomainTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct DomainResult {
        /// ARN of the Domain.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Description of the Domain.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of the role used by DataZone to configure the Domain.
        ///
        /// The following arguments are optional:
        pub domain_execution_role: pulumi_wasm_rust::Output<String>,
        /// ARN of the KMS key used to encrypt the Amazon DataZone domain, metadata and reporting data.
        pub kms_key_identifier: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the Domain.
        pub name: pulumi_wasm_rust::Output<String>,
        /// URL of the data portal for the Domain.
        pub portal_url: pulumi_wasm_rust::Output<String>,
        /// Single sign on options, used to [enable AWS IAM Identity Center](https://docs.aws.amazon.com/datazone/latest/userguide/enable-IAM-identity-center-for-datazone.html) for DataZone.
        pub single_sign_on: pulumi_wasm_rust::Output<
            Option<super::super::types::datazone::DomainSingleSignOn>,
        >,
        /// Whether to skip the deletion check for the Domain.
        pub skip_deletion_check: pulumi_wasm_rust::Output<Option<bool>>,
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::datazone::DomainTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DomainArgs) -> DomainResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let domain_execution_role_binding = args.domain_execution_role.get_inner();
        let kms_key_identifier_binding = args.kms_key_identifier.get_inner();
        let name_binding = args.name.get_inner();
        let single_sign_on_binding = args.single_sign_on.get_inner();
        let skip_deletion_check_binding = args.skip_deletion_check.get_inner();
        let tags_binding = args.tags.get_inner();
        let timeouts_binding = args.timeouts.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:datazone/domain:Domain".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "domainExecutionRole".into(),
                    value: &domain_execution_role_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyIdentifier".into(),
                    value: &kms_key_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "singleSignOn".into(),
                    value: &single_sign_on_binding,
                },
                register_interface::ObjectField {
                    name: "skipDeletionCheck".into(),
                    value: &skip_deletion_check_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "domainExecutionRole".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "portalUrl".into(),
                },
                register_interface::ResultField {
                    name: "singleSignOn".into(),
                },
                register_interface::ResultField {
                    name: "skipDeletionCheck".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "timeouts".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DomainResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            domain_execution_role: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainExecutionRole").unwrap(),
            ),
            kms_key_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyIdentifier").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            portal_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("portalUrl").unwrap(),
            ),
            single_sign_on: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("singleSignOn").unwrap(),
            ),
            skip_deletion_check: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skipDeletionCheck").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeouts").unwrap(),
            ),
        }
    }
}
