/// Provides a CodeCommit Trigger Resource.
///
///
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = repository::create(
///         "test",
///         RepositoryArgs::builder().repository_name("test").build_struct(),
///     );
///     let testTrigger = trigger::create(
///         "testTrigger",
///         TriggerArgs::builder()
///             .repository_name("${test.repositoryName}")
///             .triggers(
///                 vec![
///                     TriggerTrigger::builder().destinationArn("${testAwsSnsTopic.arn}")
///                     .events(vec!["all",]).name("all").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod trigger {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TriggerArgs {
        /// The name for the repository. This needs to be less than 100 characters.
        #[builder(into)]
        pub repository_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the trigger.
        #[builder(into)]
        pub triggers: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::codecommit::TriggerTrigger>,
        >,
    }
    #[allow(dead_code)]
    pub struct TriggerResult {
        /// System-generated unique identifier.
        pub configuration_id: pulumi_gestalt_rust::Output<String>,
        /// The name for the repository. This needs to be less than 100 characters.
        pub repository_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the trigger.
        pub triggers: pulumi_gestalt_rust::Output<
            Vec<super::super::types::codecommit::TriggerTrigger>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TriggerArgs,
    ) -> TriggerResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let repository_name_binding = args.repository_name.get_output(context);
        let triggers_binding = args.triggers.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:codecommit/trigger:Trigger".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "repositoryName".into(),
                    value: repository_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "triggers".into(),
                    value: triggers_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        TriggerResult {
            configuration_id: o.get_field("configurationId"),
            repository_name: o.get_field("repositoryName"),
            triggers: o.get_field("triggers"),
        }
    }
}
