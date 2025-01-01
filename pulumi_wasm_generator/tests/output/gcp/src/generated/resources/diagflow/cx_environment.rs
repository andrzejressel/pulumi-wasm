/// Represents an environment for an agent. You can create multiple versions of your agent and publish them to separate environments.
/// When you edit an agent, you are editing the draft agent. At any point, you can save the draft agent as an agent version, which is an immutable snapshot of your agent.
/// When you save the draft agent, it is published to the default environment. When you create agent versions, you can publish them to custom environments. You can create a variety of custom environments for testing, development, production, etc.
///
///
/// To get more information about Environment, see:
///
/// * [API documentation](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.environments)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/dialogflow/cx/docs)
///
/// ## Example Usage
///
/// ### Dialogflowcx Environment Full
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let agent = cx_agent::create(
///         "agent",
///         CxAgentArgs::builder()
///             .avatar_uri(
///                 "https://cloud.google.com/_static/images/cloud/icons/favicons/onecloud/super_cloud.png",
///             )
///             .default_language_code("en")
///             .description("Example description.")
///             .display_name("dialogflowcx-agent")
///             .enable_spell_correction(true)
///             .enable_stackdriver_logging(true)
///             .location("global")
///             .speech_to_text_settings(
///                 CxAgentSpeechToTextSettings::builder()
///                     .enableSpeechAdaptation(true)
///                     .build_struct(),
///             )
///             .supported_language_codes(vec!["fr", "de", "es",])
///             .time_zone("America/New_York")
///             .build_struct(),
///     );
///     let development = cx_environment::create(
///         "development",
///         CxEnvironmentArgs::builder()
///             .description("Development Environment")
///             .display_name("Development")
///             .parent("${agent.id}")
///             .version_configs(
///                 vec![
///                     CxEnvironmentVersionConfig::builder().version("${version1.id}")
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let version1 = cx_version::create(
///         "version1",
///         CxVersionArgs::builder()
///             .description("version 1.0.0")
///             .display_name("1.0.0")
///             .parent("${agent.startFlow}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Environment can be imported using any of these accepted formats:
///
/// * `{{parent}}/environments/{{name}}`
///
/// * `{{parent}}/{{name}}`
///
/// When using the `pulumi import` command, Environment can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:diagflow/cxEnvironment:CxEnvironment default {{parent}}/environments/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:diagflow/cxEnvironment:CxEnvironment default {{parent}}/{{name}}
/// ```
///
pub mod cx_environment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CxEnvironmentArgs {
        /// The human-readable description of the environment. The maximum length is 500 characters. If exceeded, the request is
        /// rejected.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The human-readable name of the environment (unique in an agent). Limit of 64 characters.
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The Agent to create an Environment for. Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>.
        #[builder(into, default)]
        pub parent: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of configurations for flow versions. You should include version configs for all flows that are reachable from [Start Flow][Agent.start_flow] in the agent. Otherwise, an error will be returned.
        /// Structure is documented below.
        #[builder(into)]
        pub version_configs: pulumi_wasm_rust::Output<
            Vec<super::super::types::diagflow::CxEnvironmentVersionConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct CxEnvironmentResult {
        /// The human-readable description of the environment. The maximum length is 500 characters. If exceeded, the request is
        /// rejected.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The human-readable name of the environment (unique in an agent). Limit of 64 characters.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The name of the environment.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The Agent to create an Environment for. Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>.
        pub parent: pulumi_wasm_rust::Output<Option<String>>,
        /// Update time of this environment. A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub update_time: pulumi_wasm_rust::Output<String>,
        /// A list of configurations for flow versions. You should include version configs for all flows that are reachable from [Start Flow][Agent.start_flow] in the agent. Otherwise, an error will be returned.
        /// Structure is documented below.
        pub version_configs: pulumi_wasm_rust::Output<
            Vec<super::super::types::diagflow::CxEnvironmentVersionConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: CxEnvironmentArgs) -> CxEnvironmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let parent_binding = args.parent.get_inner();
        let version_configs_binding = args.version_configs.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:diagflow/cxEnvironment:CxEnvironment".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "parent".into(),
                    value: &parent_binding,
                },
                register_interface::ObjectField {
                    name: "versionConfigs".into(),
                    value: &version_configs_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "parent".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
                register_interface::ResultField {
                    name: "versionConfigs".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CxEnvironmentResult {
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            parent: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parent").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
            version_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versionConfigs").unwrap(),
            ),
        }
    }
}
