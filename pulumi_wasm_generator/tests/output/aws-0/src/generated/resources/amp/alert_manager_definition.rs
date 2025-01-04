/// Manages an Amazon Managed Service for Prometheus (AMP) Alert Manager Definition
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let demo = workspace::create("demo", WorkspaceArgs::builder().build_struct());
///     let demoAlertManagerDefinition = alert_manager_definition::create(
///         "demoAlertManagerDefinition",
///         AlertManagerDefinitionArgs::builder()
///             .definition(
///                 "alertmanager_config: |\n  route:\n    receiver: 'default'\n  receivers:\n    - name: 'default'",
///             )
///             .workspace_id("${demo.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import the prometheus alert manager definition using the workspace identifier. For example:
///
/// ```sh
/// $ pulumi import aws:amp/alertManagerDefinition:AlertManagerDefinition demo ws-C6DCB907-F2D7-4D96-957B-66691F865D8B
/// ```
pub mod alert_manager_definition {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AlertManagerDefinitionArgs {
        /// the alert manager definition that you want to be applied. See more [in AWS Docs](https://docs.aws.amazon.com/prometheus/latest/userguide/AMP-alert-manager.html).
        #[builder(into)]
        pub definition: pulumi_wasm_rust::Output<String>,
        /// ID of the prometheus workspace the alert manager definition should be linked to
        #[builder(into)]
        pub workspace_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct AlertManagerDefinitionResult {
        /// the alert manager definition that you want to be applied. See more [in AWS Docs](https://docs.aws.amazon.com/prometheus/latest/userguide/AMP-alert-manager.html).
        pub definition: pulumi_wasm_rust::Output<String>,
        /// ID of the prometheus workspace the alert manager definition should be linked to
        pub workspace_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: AlertManagerDefinitionArgs,
    ) -> AlertManagerDefinitionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let definition_binding = args.definition.get_inner();
        let workspace_id_binding = args.workspace_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:amp/alertManagerDefinition:AlertManagerDefinition".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "definition".into(),
                    value: &definition_binding,
                },
                register_interface::ObjectField {
                    name: "workspaceId".into(),
                    value: &workspace_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "definition".into(),
                },
                register_interface::ResultField {
                    name: "workspaceId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AlertManagerDefinitionResult {
            definition: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("definition").unwrap(),
            ),
            workspace_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workspaceId").unwrap(),
            ),
        }
    }
}
