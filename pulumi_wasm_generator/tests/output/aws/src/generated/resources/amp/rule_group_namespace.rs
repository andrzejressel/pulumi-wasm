/// Manages an Amazon Managed Service for Prometheus (AMP) Rule Group Namespace
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let demo = workspace::create("demo", WorkspaceArgs::builder().build_struct());
///     let demoRuleGroupNamespace = rule_group_namespace::create(
///         "demoRuleGroupNamespace",
///         RuleGroupNamespaceArgs::builder()
///             .data(
///                 "groups:\n  - name: test\n    rules:\n    - record: metric:recording_rule\n      expr: avg(rate(container_cpu_usage_seconds_total[5m]))",
///             )
///             .name("rules")
///             .workspace_id("${demo.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import the prometheus rule group namespace using the arn. For example:
///
/// ```sh
/// $ pulumi import aws:amp/ruleGroupNamespace:RuleGroupNamespace demo arn:aws:aps:us-west-2:123456789012:rulegroupsnamespace/IDstring/namespace_name
/// ```
pub mod rule_group_namespace {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RuleGroupNamespaceArgs {
        /// the rule group namespace data that you want to be applied. See more [in AWS Docs](https://docs.aws.amazon.com/prometheus/latest/userguide/AMP-Ruler.html).
        #[builder(into)]
        pub data: pulumi_wasm_rust::Output<String>,
        /// The name of the rule group namespace
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// ID of the prometheus workspace the rule group namespace should be linked to
        #[builder(into)]
        pub workspace_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct RuleGroupNamespaceResult {
        /// the rule group namespace data that you want to be applied. See more [in AWS Docs](https://docs.aws.amazon.com/prometheus/latest/userguide/AMP-Ruler.html).
        pub data: pulumi_wasm_rust::Output<String>,
        /// The name of the rule group namespace
        pub name: pulumi_wasm_rust::Output<String>,
        /// ID of the prometheus workspace the rule group namespace should be linked to
        pub workspace_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: RuleGroupNamespaceArgs) -> RuleGroupNamespaceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let data_binding = args.data.get_inner();
        let name_binding = args.name.get_inner();
        let workspace_id_binding = args.workspace_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:amp/ruleGroupNamespace:RuleGroupNamespace".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "data".into(),
                    value: &data_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "workspaceId".into(),
                    value: &workspace_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "data".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
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
        RuleGroupNamespaceResult {
            data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("data").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            workspace_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workspaceId").unwrap(),
            ),
        }
    }
}