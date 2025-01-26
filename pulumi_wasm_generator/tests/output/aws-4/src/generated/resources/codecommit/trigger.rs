/// Provides a CodeCommit Trigger Resource.
///
///
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod trigger {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TriggerArgs {
        /// The name for the repository. This needs to be less than 100 characters.
        #[builder(into)]
        pub repository_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the trigger.
        #[builder(into)]
        pub triggers: pulumi_wasm_rust::InputOrOutput<
            Vec<super::super::types::codecommit::TriggerTrigger>,
        >,
    }
    #[allow(dead_code)]
    pub struct TriggerResult {
        /// System-generated unique identifier.
        pub configuration_id: pulumi_wasm_rust::Output<String>,
        /// The name for the repository. This needs to be less than 100 characters.
        pub repository_name: pulumi_wasm_rust::Output<String>,
        /// The name of the trigger.
        pub triggers: pulumi_wasm_rust::Output<
            Vec<super::super::types::codecommit::TriggerTrigger>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: TriggerArgs,
    ) -> TriggerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let repository_name_binding = args
            .repository_name
            .get_output(context)
            .get_inner();
        let triggers_binding = args.triggers.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:codecommit/trigger:Trigger".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "repositoryName".into(),
                    value: &repository_name_binding,
                },
                register_interface::ObjectField {
                    name: "triggers".into(),
                    value: &triggers_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "configurationId".into(),
                },
                register_interface::ResultField {
                    name: "repositoryName".into(),
                },
                register_interface::ResultField {
                    name: "triggers".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TriggerResult {
            configuration_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configurationId").unwrap(),
            ),
            repository_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("repositoryName").unwrap(),
            ),
            triggers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("triggers").unwrap(),
            ),
        }
    }
}
