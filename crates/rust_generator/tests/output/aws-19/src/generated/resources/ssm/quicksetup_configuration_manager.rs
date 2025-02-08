/// Resource for managing an AWS SSM Quick Setup Configuration Manager.
///
/// ## Example Usage
///
/// ### Patch Policy Configuration Type
///
///
/// ## Import
///
/// Using `pulumi import`, import SSM Quick Setup Configuration Manager using the `manager_arn`. For example:
///
/// ```sh
/// $ pulumi import aws:ssm/quicksetupConfigurationManager:QuicksetupConfigurationManager example arn:aws:ssm-quicksetup:us-east-1:012345678901:configuration-manager/abcd-1234
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod quicksetup_configuration_manager {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct QuicksetupConfigurationManagerArgs {
        /// Definition of the Quick Setup configuration that the configuration manager deploys. See `configuration_definition` below.
        #[builder(into, default)]
        pub configuration_definition: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::ssm::QuicksetupConfigurationManagerConfigurationDefinition,
            >,
        >,
        /// Description of the configuration manager.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration manager name.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags assigned to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ssm::QuicksetupConfigurationManagerTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct QuicksetupConfigurationManagerResult {
        /// Definition of the Quick Setup configuration that the configuration manager deploys. See `configuration_definition` below.
        pub configuration_definition: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::ssm::QuicksetupConfigurationManagerConfigurationDefinition,
            >,
        >,
        /// Description of the configuration manager.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// ARN of the Configuration Manager.
        pub manager_arn: pulumi_gestalt_rust::Output<String>,
        /// Configuration manager name.
        ///
        /// The following arguments are optional:
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A summary of the state of the configuration manager. This includes deployment statuses, association statuses, drift statuses, health checks, and more. See `status_summaries` below.
        pub status_summaries: pulumi_gestalt_rust::Output<
            Vec<super::super::types::ssm::QuicksetupConfigurationManagerStatusSummary>,
        >,
        /// Map of tags assigned to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::ssm::QuicksetupConfigurationManagerTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: QuicksetupConfigurationManagerArgs,
    ) -> QuicksetupConfigurationManagerResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let configuration_definition_binding = args
            .configuration_definition
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let timeouts_binding = args.timeouts.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ssm/quicksetupConfigurationManager:QuicksetupConfigurationManager"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "configurationDefinition".into(),
                    value: &configuration_definition_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
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
        QuicksetupConfigurationManagerResult {
            configuration_definition: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("configurationDefinition"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            manager_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("managerArn"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            status_summaries: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("statusSummaries"),
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
