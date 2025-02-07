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
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SchemaArgs {
        /// The definition of the schema.
        #[builder(into, default)]
        pub definition: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::verifiedpermissions::SchemaDefinition>,
        >,
        /// The ID of the Policy Store.
        #[builder(into)]
        pub policy_store_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SchemaResult {
        /// The definition of the schema.
        pub definition: pulumi_gestalt_rust::Output<
            Option<super::super::types::verifiedpermissions::SchemaDefinition>,
        >,
        /// (Optional) Identifies the namespaces of the entities referenced by this schema.
        pub namespaces: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The ID of the Policy Store.
        pub policy_store_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SchemaArgs,
    ) -> SchemaResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        SchemaResult {
            definition: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("definition"),
            ),
            namespaces: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("namespaces"),
            ),
            policy_store_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("policyStoreId"),
            ),
        }
    }
}
