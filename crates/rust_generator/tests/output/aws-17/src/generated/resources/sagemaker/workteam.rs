/// Provides a SageMaker Workteam resource.
///
/// ## Example Usage
///
/// ### Cognito Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = workteam::create(
///         "example",
///         WorkteamArgs::builder()
///             .description("example")
///             .member_definitions(
///                 vec![
///                     WorkteamMemberDefinition::builder()
///                     .cognitoMemberDefinition(WorkteamMemberDefinitionCognitoMemberDefinition::builder()
///                     .clientId("${exampleAwsCognitoUserPoolClient.id}")
///                     .userGroup("${exampleAwsCognitoUserGroup.name}")
///                     .userPool("${exampleAwsCognitoUserPoolDomain.userPoolId}")
///                     .build_struct()).build_struct(),
///                 ],
///             )
///             .workforce_name("${exampleAwsSagemakerWorkforce.id}")
///             .workteam_name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Oidc Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = workteam::create(
///         "example",
///         WorkteamArgs::builder()
///             .description("example")
///             .member_definitions(
///                 vec![
///                     WorkteamMemberDefinition::builder()
///                     .oidcMemberDefinition(WorkteamMemberDefinitionOidcMemberDefinition::builder()
///                     .groups(vec!["example",]).build_struct()).build_struct(),
///                 ],
///             )
///             .workforce_name("${exampleAwsSagemakerWorkforce.id}")
///             .workteam_name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SageMaker Workteams using the `workteam_name`. For example:
///
/// ```sh
/// $ pulumi import aws:sagemaker/workteam:Workteam example example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod workteam {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkteamArgs {
        /// A description of the work team.
        #[builder(into)]
        pub description: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A list of Member Definitions that contains objects that identify the workers that make up the work team. Workforces can be created using Amazon Cognito or your own OIDC Identity Provider (IdP). For private workforces created using Amazon Cognito use `cognito_member_definition`. For workforces created using your own OIDC identity provider (IdP) use `oidc_member_definition`. Do not provide input for both of these parameters in a single request. see Member Definition details below.
        #[builder(into)]
        pub member_definitions: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::sagemaker::WorkteamMemberDefinition>,
        >,
        /// Configures notification of workers regarding available or expiring work items. see Notification Configuration details below.
        #[builder(into, default)]
        pub notification_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::sagemaker::WorkteamNotificationConfiguration>,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Use this optional parameter to constrain access to an Amazon S3 resource based on the IP address using supported IAM global condition keys. The Amazon S3 resource is accessed in the worker portal using a Amazon S3 presigned URL. see Worker Access Configuration details below.
        #[builder(into, default)]
        pub worker_access_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::sagemaker::WorkteamWorkerAccessConfiguration>,
        >,
        /// The name of the workforce.
        #[builder(into, default)]
        pub workforce_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Workteam (must be unique).
        #[builder(into)]
        pub workteam_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct WorkteamResult {
        /// The Amazon Resource Name (ARN) assigned by AWS to this Workteam.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A description of the work team.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// A list of Member Definitions that contains objects that identify the workers that make up the work team. Workforces can be created using Amazon Cognito or your own OIDC Identity Provider (IdP). For private workforces created using Amazon Cognito use `cognito_member_definition`. For workforces created using your own OIDC identity provider (IdP) use `oidc_member_definition`. Do not provide input for both of these parameters in a single request. see Member Definition details below.
        pub member_definitions: pulumi_gestalt_rust::Output<
            Vec<super::super::types::sagemaker::WorkteamMemberDefinition>,
        >,
        /// Configures notification of workers regarding available or expiring work items. see Notification Configuration details below.
        pub notification_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::sagemaker::WorkteamNotificationConfiguration>,
        >,
        /// The subdomain for your OIDC Identity Provider.
        pub subdomain: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Use this optional parameter to constrain access to an Amazon S3 resource based on the IP address using supported IAM global condition keys. The Amazon S3 resource is accessed in the worker portal using a Amazon S3 presigned URL. see Worker Access Configuration details below.
        pub worker_access_configuration: pulumi_gestalt_rust::Output<
            super::super::types::sagemaker::WorkteamWorkerAccessConfiguration,
        >,
        /// The name of the workforce.
        pub workforce_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Workteam (must be unique).
        pub workteam_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: WorkteamArgs,
    ) -> WorkteamResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let member_definitions_binding_1 = args.member_definitions.get_output(context);
        let member_definitions_binding = member_definitions_binding_1.get_inner();
        let notification_configuration_binding_1 = args
            .notification_configuration
            .get_output(context);
        let notification_configuration_binding = notification_configuration_binding_1
            .get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let worker_access_configuration_binding_1 = args
            .worker_access_configuration
            .get_output(context);
        let worker_access_configuration_binding = worker_access_configuration_binding_1
            .get_inner();
        let workforce_name_binding_1 = args.workforce_name.get_output(context);
        let workforce_name_binding = workforce_name_binding_1.get_inner();
        let workteam_name_binding_1 = args.workteam_name.get_output(context);
        let workteam_name_binding = workteam_name_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sagemaker/workteam:Workteam".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "memberDefinitions".into(),
                    value: &member_definitions_binding,
                },
                register_interface::ObjectField {
                    name: "notificationConfiguration".into(),
                    value: &notification_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "workerAccessConfiguration".into(),
                    value: &worker_access_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "workforceName".into(),
                    value: &workforce_name_binding,
                },
                register_interface::ObjectField {
                    name: "workteamName".into(),
                    value: &workteam_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        WorkteamResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            member_definitions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("memberDefinitions"),
            ),
            notification_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("notificationConfiguration"),
            ),
            subdomain: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subdomain"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            worker_access_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workerAccessConfiguration"),
            ),
            workforce_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workforceName"),
            ),
            workteam_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workteamName"),
            ),
        }
    }
}
