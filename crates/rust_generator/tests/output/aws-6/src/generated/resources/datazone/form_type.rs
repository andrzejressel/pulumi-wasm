/// Resource for managing an AWS DataZone Form Type.
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
///       name: example-role
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
///         - name: example-policy
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
///   test:
///     type: aws:datazone:Domain
///     properties:
///       name: example
///       domainExecutionRole: ${domainExecutionRole.arn}
///   testSecurityGroup:
///     type: aws:ec2:SecurityGroup
///     name: test
///     properties:
///       name: example
///   testProject:
///     type: aws:datazone:Project
///     name: test
///     properties:
///       domainIdentifier: ${test.id}
///       glossaryTerms:
///         - 2N8w6XJCwZf
///       name: example name
///       description: desc
///       skipDeletionCheck: true
///   testFormType:
///     type: aws:datazone:FormType
///     name: test
///     properties:
///       description: desc
///       name: SageMakerModelFormType
///       domainIdentifier: ${test.id}
///       owningProjectIdentifier: ${testProject.id}
///       status: DISABLED
///       model:
///         smithy: |
///           	structure SageMakerModelFormType {
///           			@required
///           			@amazon.datazone#searchable
///           			modelName: String
///
///           			@required
///           			modelArn: String
///
///           			@required
///           			creationTime: String
///           			}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import DataZone Form Type using a comma separated value of `domain_identifier`,`name`,`revision`. For example:
///
/// ```sh
/// $ pulumi import aws:datazone/formType:FormType example domain_identifier,name,revision
/// ```
#[allow(clippy::doc_lazy_continuation)]
pub mod form_type {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FormTypeArgs {
        /// Description of form type. Must have a length of between 1 and 2048 characters.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Identifier of the domain.
        #[builder(into)]
        pub domain_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Object of the model of the form type that contains the following attributes.
        #[builder(into, default)]
        pub model: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::datazone::FormTypeModel>,
        >,
        /// Name of the form type. Must be the name of the structure in smithy document.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Identifier of project that owns the form type. Must follow regex of ^[a-zA-Z0-9_-]{1,36}.
        #[builder(into)]
        pub owning_project_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub status: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::datazone::FormTypeTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct FormTypeResult {
        /// Creation time of the Form Type.
        pub created_at: pulumi_gestalt_rust::Output<String>,
        /// Creator of the Form Type.
        pub created_by: pulumi_gestalt_rust::Output<String>,
        /// Description of form type. Must have a length of between 1 and 2048 characters.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Identifier of the domain.
        pub domain_identifier: pulumi_gestalt_rust::Output<String>,
        pub imports: pulumi_gestalt_rust::Output<
            Vec<super::super::types::datazone::FormTypeImport>,
        >,
        /// Object of the model of the form type that contains the following attributes.
        pub model: pulumi_gestalt_rust::Output<
            Option<super::super::types::datazone::FormTypeModel>,
        >,
        /// Name of the form type. Must be the name of the structure in smithy document.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Origin domain id of the Form Type.
        pub origin_domain_id: pulumi_gestalt_rust::Output<String>,
        /// Origin project id of the Form Type.
        pub origin_project_id: pulumi_gestalt_rust::Output<String>,
        /// Identifier of project that owns the form type. Must follow regex of ^[a-zA-Z0-9_-]{1,36}.
        pub owning_project_identifier: pulumi_gestalt_rust::Output<String>,
        /// Revision of the Form Type.
        pub revision: pulumi_gestalt_rust::Output<String>,
        pub status: pulumi_gestalt_rust::Output<String>,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::datazone::FormTypeTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: FormTypeArgs,
    ) -> FormTypeResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let domain_identifier_binding = args
            .domain_identifier
            .get_output(context)
            .get_inner();
        let model_binding = args.model.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let owning_project_identifier_binding = args
            .owning_project_identifier
            .get_output(context)
            .get_inner();
        let status_binding = args.status.get_output(context).get_inner();
        let timeouts_binding = args.timeouts.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:datazone/formType:FormType".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "domainIdentifier".into(),
                    value: &domain_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "model".into(),
                    value: &model_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "owningProjectIdentifier".into(),
                    value: &owning_project_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "status".into(),
                    value: &status_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        FormTypeResult {
            created_at: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createdAt"),
            ),
            created_by: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createdBy"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            domain_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("domainIdentifier"),
            ),
            imports: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("imports"),
            ),
            model: pulumi_gestalt_rust::__private::into_domain(o.extract_field("model")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            origin_domain_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("originDomainId"),
            ),
            origin_project_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("originProjectId"),
            ),
            owning_project_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("owningProjectIdentifier"),
            ),
            revision: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("revision"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            timeouts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timeouts"),
            ),
        }
    }
}
