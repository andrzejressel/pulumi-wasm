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
#[allow(clippy::doc_lazy_continuation)]
pub mod domain {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainArgs {
        /// Description of the Domain.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ARN of the role used by DataZone to configure the Domain.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub domain_execution_role: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ARN of the KMS key used to encrypt the Amazon DataZone domain, metadata and reporting data.
        #[builder(into, default)]
        pub kms_key_identifier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the Domain.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Single sign on options, used to [enable AWS IAM Identity Center](https://docs.aws.amazon.com/datazone/latest/userguide/enable-IAM-identity-center-for-datazone.html) for DataZone.
        #[builder(into, default)]
        pub single_sign_on: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::datazone::DomainSingleSignOn>,
        >,
        /// Whether to skip the deletion check for the Domain.
        #[builder(into, default)]
        pub skip_deletion_check: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::datazone::DomainTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct DomainResult {
        /// ARN of the Domain.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Description of the Domain.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// ARN of the role used by DataZone to configure the Domain.
        ///
        /// The following arguments are optional:
        pub domain_execution_role: pulumi_gestalt_rust::Output<String>,
        /// ARN of the KMS key used to encrypt the Amazon DataZone domain, metadata and reporting data.
        pub kms_key_identifier: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the Domain.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// URL of the data portal for the Domain.
        pub portal_url: pulumi_gestalt_rust::Output<String>,
        /// Single sign on options, used to [enable AWS IAM Identity Center](https://docs.aws.amazon.com/datazone/latest/userguide/enable-IAM-identity-center-for-datazone.html) for DataZone.
        pub single_sign_on: pulumi_gestalt_rust::Output<
            Option<super::super::types::datazone::DomainSingleSignOn>,
        >,
        /// Whether to skip the deletion check for the Domain.
        pub skip_deletion_check: pulumi_gestalt_rust::Output<Option<bool>>,
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::datazone::DomainTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: DomainArgs,
    ) -> DomainResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let domain_execution_role_binding = args
            .domain_execution_role
            .get_output(context)
            .get_inner();
        let kms_key_identifier_binding = args
            .kms_key_identifier
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let single_sign_on_binding = args.single_sign_on.get_output(context).get_inner();
        let skip_deletion_check_binding = args
            .skip_deletion_check
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let timeouts_binding = args.timeouts.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:datazone/domain:Domain".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        DomainResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            domain_execution_role: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("domainExecutionRole"),
            ),
            kms_key_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsKeyIdentifier"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            portal_url: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("portalUrl"),
            ),
            single_sign_on: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("singleSignOn"),
            ),
            skip_deletion_check: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("skipDeletionCheck"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            timeouts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timeouts"),
            ),
        }
    }
}
