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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EnvironmentArgs,
    ) -> EnvironmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_identifier_binding = args.account_identifier.get_output(context);
        let account_region_binding = args.account_region.get_output(context);
        let blueprint_identifier_binding = args.blueprint_identifier.get_output(context);
        let description_binding = args.description.get_output(context);
        let domain_identifier_binding = args.domain_identifier.get_output(context);
        let glossary_terms_binding = args.glossary_terms.get_output(context);
        let name_binding = args.name.get_output(context);
        let profile_identifier_binding = args.profile_identifier.get_output(context);
        let project_identifier_binding = args.project_identifier.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let user_parameters_binding = args.user_parameters.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:datazone/environment:Environment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountIdentifier".into(),
                    value: account_identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountRegion".into(),
                    value: account_region_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "blueprintIdentifier".into(),
                    value: blueprint_identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainIdentifier".into(),
                    value: domain_identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "glossaryTerms".into(),
                    value: glossary_terms_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "profileIdentifier".into(),
                    value: profile_identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "projectIdentifier".into(),
                    value: project_identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: timeouts_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userParameters".into(),
                    value: user_parameters_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        EnvironmentResult {
            account_identifier: o.get_field("accountIdentifier"),
            account_region: o.get_field("accountRegion"),
            blueprint_identifier: o.get_field("blueprintIdentifier"),
            created_at: o.get_field("createdAt"),
            created_by: o.get_field("createdBy"),
            description: o.get_field("description"),
            domain_identifier: o.get_field("domainIdentifier"),
            glossary_terms: o.get_field("glossaryTerms"),
            last_deployments: o.get_field("lastDeployments"),
            name: o.get_field("name"),
            profile_identifier: o.get_field("profileIdentifier"),
            project_identifier: o.get_field("projectIdentifier"),
            provider_environment: o.get_field("providerEnvironment"),
            provisioned_resources: o.get_field("provisionedResources"),
            timeouts: o.get_field("timeouts"),
            user_parameters: o.get_field("userParameters"),
        }
    }
}
