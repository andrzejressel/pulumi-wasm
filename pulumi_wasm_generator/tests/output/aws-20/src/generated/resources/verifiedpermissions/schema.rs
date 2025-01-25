/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:verifiedpermissions:Schema
///     properties:
///       policyStoreId: ${exampleAwsVerifiedpermissionsPolicyStore.policyStoreId}
///       definition:
///         - value:
///             fn::toJSON:
///               Namespace:
///                 entityTypes: {}
///                 actions: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Verified Permissions Policy Store Schema using the `policy_store_id`. For example:
///
/// console
///
///  % pulumi import aws_verifiedpermissions_schema.example DxQg2j8xvXJQ1tQCYNWj9T
///
pub mod schema {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SchemaArgs {
        /// The definition of the schema.
        #[builder(into, default)]
        pub definition: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::verifiedpermissions::SchemaDefinition>,
        >,
        /// The ID of the Policy Store.
        #[builder(into)]
        pub policy_store_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SchemaResult {
        /// The definition of the schema.
        pub definition: pulumi_wasm_rust::Output<
            Option<super::super::types::verifiedpermissions::SchemaDefinition>,
        >,
        /// (Optional) Identifies the namespaces of the entities referenced by this schema.
        pub namespaces: pulumi_wasm_rust::Output<Vec<String>>,
        /// The ID of the Policy Store.
        pub policy_store_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: SchemaArgs,
    ) -> SchemaResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let definition_binding = args.definition.get_output(context).get_inner();
        let policy_store_id_binding = args
            .policy_store_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:verifiedpermissions/schema:Schema".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "definition".into(),
                    value: &definition_binding,
                },
                register_interface::ObjectField {
                    name: "policyStoreId".into(),
                    value: &policy_store_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "definition".into(),
                },
                register_interface::ResultField {
                    name: "namespaces".into(),
                },
                register_interface::ResultField {
                    name: "policyStoreId".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SchemaResult {
            definition: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("definition").unwrap(),
            ),
            namespaces: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namespaces").unwrap(),
            ),
            policy_store_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyStoreId").unwrap(),
            ),
        }
    }
}
