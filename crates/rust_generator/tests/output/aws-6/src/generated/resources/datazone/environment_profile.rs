/// Resource for managing an AWS DataZone Environment Profile.
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
///       name: example-name
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
///         - name: example-name
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
///   testDomain:
///     type: aws:datazone:Domain
///     name: test
///     properties:
///       name: example-name
///       domainExecutionRole: ${domainExecutionRole.arn}
///   testSecurityGroup:
///     type: aws:ec2:SecurityGroup
///     name: test
///     properties:
///       name: example-name
///   testProject:
///     type: aws:datazone:Project
///     name: test
///     properties:
///       domainIdentifier: ${testDomain.id}
///       glossaryTerms:
///         - 2N8w6XJCwZf
///       name: example-name
///       description: desc
///       skipDeletionCheck: true
///   testEnvironmentBlueprintConfiguration:
///     type: aws:datazone:EnvironmentBlueprintConfiguration
///     name: test
///     properties:
///       domainId: ${testDomain.id}
///       environmentBlueprintId: ${testGetEnvironmentBlueprint.id}
///       provisioningRoleArn: ${domainExecutionRole.arn}
///       enabledRegions:
///         - ${testGetRegion.name}
///   testEnvironmentProfile:
///     type: aws:datazone:EnvironmentProfile
///     name: test
///     properties:
///       awsAccountId: ${test.accountId}
///       awsAccountRegion: ${testGetRegion.name}
///       description: description
///       environmentBlueprintIdentifier: ${testGetEnvironmentBlueprint.id}
///       name: example-name
///       projectIdentifier: ${testProject.id}
///       domainIdentifier: ${testDomain.id}
///       userParameters:
///         - name: consumerGlueDbName
///           value: value
/// variables:
///   test:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
///   testGetRegion:
///     fn::invoke:
///       function: aws:getRegion
///       arguments: {}
///   testGetEnvironmentBlueprint:
///     fn::invoke:
///       function: aws:datazone:getEnvironmentBlueprint
///       arguments:
///         domainId: ${testDomain.id}
///         name: DefaultDataLake
///         managed: true
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import DataZone Environment Profile using a comma-delimited string combining `id` and `domain_identifier`. For example:
///
/// ```sh
/// $ pulumi import aws:datazone/environmentProfile:EnvironmentProfile example environment_profile-id-12345678,domain-id-12345678
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod environment_profile {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnvironmentProfileArgs {
        /// Id of the AWS account being used.
        #[builder(into, default)]
        pub aws_account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Desired region for environment profile.
        #[builder(into)]
        pub aws_account_region: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Description of environment profile.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Domain Identifier for environment profile.
        #[builder(into)]
        pub domain_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ID of the blueprint which the environment will be created with.
        #[builder(into)]
        pub environment_blueprint_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the environment profile.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Project identifier for environment profile.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub project_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Array of user parameters of the environment profile with the following attributes:
        #[builder(into, default)]
        pub user_parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::datazone::EnvironmentProfileUserParameter>>,
        >,
    }
    #[allow(dead_code)]
    pub struct EnvironmentProfileResult {
        /// Id of the AWS account being used.
        pub aws_account_id: pulumi_gestalt_rust::Output<String>,
        /// Desired region for environment profile.
        pub aws_account_region: pulumi_gestalt_rust::Output<String>,
        /// Creation time of environment profile.
        pub created_at: pulumi_gestalt_rust::Output<String>,
        /// Creator of environment profile.
        pub created_by: pulumi_gestalt_rust::Output<String>,
        /// Description of environment profile.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Domain Identifier for environment profile.
        pub domain_identifier: pulumi_gestalt_rust::Output<String>,
        /// ID of the blueprint which the environment will be created with.
        pub environment_blueprint_identifier: pulumi_gestalt_rust::Output<String>,
        /// Name of the environment profile.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Project identifier for environment profile.
        ///
        /// The following arguments are optional:
        pub project_identifier: pulumi_gestalt_rust::Output<String>,
        /// Time of last update to environment profile.
        pub updated_at: pulumi_gestalt_rust::Output<String>,
        /// Array of user parameters of the environment profile with the following attributes:
        pub user_parameters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::datazone::EnvironmentProfileUserParameter>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: EnvironmentProfileArgs,
    ) -> EnvironmentProfileResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let aws_account_id_binding = args.aws_account_id.get_output(context).get_inner();
        let aws_account_region_binding = args
            .aws_account_region
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let domain_identifier_binding = args
            .domain_identifier
            .get_output(context)
            .get_inner();
        let environment_blueprint_identifier_binding = args
            .environment_blueprint_identifier
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_identifier_binding = args
            .project_identifier
            .get_output(context)
            .get_inner();
        let user_parameters_binding = args
            .user_parameters
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:datazone/environmentProfile:EnvironmentProfile".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "awsAccountId".into(),
                    value: &aws_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "awsAccountRegion".into(),
                    value: &aws_account_region_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "domainIdentifier".into(),
                    value: &domain_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "environmentBlueprintIdentifier".into(),
                    value: &environment_blueprint_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "projectIdentifier".into(),
                    value: &project_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "userParameters".into(),
                    value: &user_parameters_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        EnvironmentProfileResult {
            aws_account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("awsAccountId"),
            ),
            aws_account_region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("awsAccountRegion"),
            ),
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
            environment_blueprint_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("environmentBlueprintIdentifier"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("projectIdentifier"),
            ),
            updated_at: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updatedAt"),
            ),
            user_parameters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userParameters"),
            ),
        }
    }
}
