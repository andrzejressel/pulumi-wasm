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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FormTypeArgs,
    ) -> FormTypeResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let domain_identifier_binding = args.domain_identifier.get_output(context);
        let model_binding = args.model.get_output(context);
        let name_binding = args.name.get_output(context);
        let owning_project_identifier_binding = args
            .owning_project_identifier
            .get_output(context);
        let status_binding = args.status.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:datazone/formType:FormType".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainIdentifier".into(),
                    value: domain_identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "model".into(),
                    value: model_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "owningProjectIdentifier".into(),
                    value: owning_project_identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "status".into(),
                    value: status_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: timeouts_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        FormTypeResult {
            created_at: o.get_field("createdAt"),
            created_by: o.get_field("createdBy"),
            description: o.get_field("description"),
            domain_identifier: o.get_field("domainIdentifier"),
            imports: o.get_field("imports"),
            model: o.get_field("model"),
            name: o.get_field("name"),
            origin_domain_id: o.get_field("originDomainId"),
            origin_project_id: o.get_field("originProjectId"),
            owning_project_identifier: o.get_field("owningProjectIdentifier"),
            revision: o.get_field("revision"),
            status: o.get_field("status"),
            timeouts: o.get_field("timeouts"),
        }
    }
}
