/// Resource for managing an AWS DataZone Environment.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = environment::create(
///         "example",
///         EnvironmentArgs::builder()
///             .account_identifier("${test.accountId}")
///             .account_region("${testAwsRegion.name}")
///             .blueprint_identifier(
///                 "${testAwsDatazoneEnvironmentBlueprintConfiguration.environmentBlueprintId}",
///             )
///             .domain_identifier("${testAwsDatazoneDomain.id}")
///             .name("example")
///             .profile_identifier("${testAwsDatazoneEnvironmentProfile.id}")
///             .project_identifier("${testAwsDatazoneProject.id}")
///             .user_parameters(
///                 vec![
///                     EnvironmentUserParameter::builder().name("consumerGlueDbName")
///                     .value("consumer").build_struct(),
///                     EnvironmentUserParameter::builder().name("producerGlueDbName")
///                     .value("producer").build_struct(),
///                     EnvironmentUserParameter::builder().name("workgroupName")
///                     .value("workgroup").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import DataZone Environment using the `domain_idntifier,id`. For example:
///
/// ```sh
/// $ pulumi import aws:datazone/environment:Environment example dzd_d2i7tzk3tnjjf4,5vpywijpwryec0
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod environment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnvironmentArgs {
        /// The ID of the Amazon Web Services account where the environment exists
        #[builder(into, default)]
        pub account_identifier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Amazon Web Services region where the environment exists.
        #[builder(into, default)]
        pub account_region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The blueprint with which the environment is created.
        #[builder(into, default)]
        pub blueprint_identifier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the domain where the environment exists.
        #[builder(into)]
        pub domain_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The business glossary terms that can be used in this environment.
        #[builder(into, default)]
        pub glossary_terms: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The name of the environment.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the profile with which the environment is created.
        #[builder(into)]
        pub profile_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project where the environment exists.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub project_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::datazone::EnvironmentTimeouts>,
        >,
        /// The user parameters that are used in the environment. See User Parameters for more information.
        #[builder(into, default)]
        pub user_parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::datazone::EnvironmentUserParameter>>,
        >,
    }
    #[allow(dead_code)]
    pub struct EnvironmentResult {
        /// The ID of the Amazon Web Services account where the environment exists
        pub account_identifier: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Web Services region where the environment exists.
        pub account_region: pulumi_gestalt_rust::Output<String>,
        /// The blueprint with which the environment is created.
        pub blueprint_identifier: pulumi_gestalt_rust::Output<String>,
        /// The time the environment was created.
        pub created_at: pulumi_gestalt_rust::Output<String>,
        /// The user who created the environment.
        pub created_by: pulumi_gestalt_rust::Output<String>,
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the domain where the environment exists.
        pub domain_identifier: pulumi_gestalt_rust::Output<String>,
        /// The business glossary terms that can be used in this environment.
        pub glossary_terms: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The details of the last deployment of the environment.
        pub last_deployments: pulumi_gestalt_rust::Output<
            Vec<super::super::types::datazone::EnvironmentLastDeployment>,
        >,
        /// The name of the environment.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the profile with which the environment is created.
        pub profile_identifier: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project where the environment exists.
        ///
        /// The following arguments are optional:
        pub project_identifier: pulumi_gestalt_rust::Output<String>,
        /// The provider of the environment.
        pub provider_environment: pulumi_gestalt_rust::Output<String>,
        pub provisioned_resources: pulumi_gestalt_rust::Output<
            Vec<super::super::types::datazone::EnvironmentProvisionedResource>,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::datazone::EnvironmentTimeouts>,
        >,
        /// The user parameters that are used in the environment. See User Parameters for more information.
        pub user_parameters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::datazone::EnvironmentUserParameter>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: EnvironmentArgs,
    ) -> EnvironmentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_identifier_binding_1 = args.account_identifier.get_output(context);
        let account_identifier_binding = account_identifier_binding_1.get_inner();
        let account_region_binding_1 = args.account_region.get_output(context);
        let account_region_binding = account_region_binding_1.get_inner();
        let blueprint_identifier_binding_1 = args
            .blueprint_identifier
            .get_output(context);
        let blueprint_identifier_binding = blueprint_identifier_binding_1.get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let domain_identifier_binding_1 = args.domain_identifier.get_output(context);
        let domain_identifier_binding = domain_identifier_binding_1.get_inner();
        let glossary_terms_binding_1 = args.glossary_terms.get_output(context);
        let glossary_terms_binding = glossary_terms_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let profile_identifier_binding_1 = args.profile_identifier.get_output(context);
        let profile_identifier_binding = profile_identifier_binding_1.get_inner();
        let project_identifier_binding_1 = args.project_identifier.get_output(context);
        let project_identifier_binding = project_identifier_binding_1.get_inner();
        let timeouts_binding_1 = args.timeouts.get_output(context);
        let timeouts_binding = timeouts_binding_1.get_inner();
        let user_parameters_binding_1 = args.user_parameters.get_output(context);
        let user_parameters_binding = user_parameters_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:datazone/environment:Environment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountIdentifier".into(),
                    value: &account_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "accountRegion".into(),
                    value: &account_region_binding,
                },
                register_interface::ObjectField {
                    name: "blueprintIdentifier".into(),
                    value: &blueprint_identifier_binding,
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
                    name: "glossaryTerms".into(),
                    value: &glossary_terms_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "profileIdentifier".into(),
                    value: &profile_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "projectIdentifier".into(),
                    value: &project_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
                register_interface::ObjectField {
                    name: "userParameters".into(),
                    value: &user_parameters_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        EnvironmentResult {
            account_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountIdentifier"),
            ),
            account_region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountRegion"),
            ),
            blueprint_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("blueprintIdentifier"),
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
            glossary_terms: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("glossaryTerms"),
            ),
            last_deployments: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lastDeployments"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            profile_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("profileIdentifier"),
            ),
            project_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("projectIdentifier"),
            ),
            provider_environment: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("providerEnvironment"),
            ),
            provisioned_resources: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("provisionedResources"),
            ),
            timeouts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timeouts"),
            ),
            user_parameters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userParameters"),
            ),
        }
    }
}
