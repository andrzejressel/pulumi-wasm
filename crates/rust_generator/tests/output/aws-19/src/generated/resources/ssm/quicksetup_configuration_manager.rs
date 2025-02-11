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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: QuicksetupConfigurationManagerArgs,
    ) -> QuicksetupConfigurationManagerResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let configuration_definition_binding = args
            .configuration_definition
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ssm/quicksetupConfigurationManager:QuicksetupConfigurationManager"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configurationDefinition".into(),
                    value: &configuration_definition_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        QuicksetupConfigurationManagerResult {
            configuration_definition: o.get_field("configurationDefinition"),
            description: o.get_field("description"),
            manager_arn: o.get_field("managerArn"),
            name: o.get_field("name"),
            status_summaries: o.get_field("statusSummaries"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            timeouts: o.get_field("timeouts"),
        }
    }
}
