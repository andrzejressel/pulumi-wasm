/// Resource for managing an AWS Service Catalog AppRegistry Attribute Group Association.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:servicecatalog:AppregistryApplication
///     properties:
///       name: example-app
///   exampleAppregistryAttributeGroup:
///     type: aws:servicecatalog:AppregistryAttributeGroup
///     name: example
///     properties:
///       name: example
///       description: example description
///       attributes:
///         fn::toJSON:
///           app: exampleapp
///           group: examplegroup
///   exampleAppregistryAttributeGroupAssociation:
///     type: aws:servicecatalog:AppregistryAttributeGroupAssociation
///     name: example
///     properties:
///       applicationId: ${example.id}
///       attributeGroupId: ${exampleAppregistryAttributeGroup.id}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Service Catalog AppRegistry Attribute Group Association using `application_id` and `attribute_group_id` arguments separated by a comma (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:servicecatalog/appregistryAttributeGroupAssociation:AppregistryAttributeGroupAssociation example 12456778723424sdffsdfsdq34,12234t3564dsfsdf34asff4ww3
/// ```
pub mod appregistry_attribute_group_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AppregistryAttributeGroupAssociationArgs {
        /// ID of the application.
        #[builder(into)]
        pub application_id: pulumi_wasm_rust::Output<String>,
        /// ID of the attribute group to associate with the application.
        #[builder(into)]
        pub attribute_group_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct AppregistryAttributeGroupAssociationResult {
        /// ID of the application.
        pub application_id: pulumi_wasm_rust::Output<String>,
        /// ID of the attribute group to associate with the application.
        pub attribute_group_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: AppregistryAttributeGroupAssociationArgs,
    ) -> AppregistryAttributeGroupAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let application_id_binding = args.application_id.get_inner();
        let attribute_group_id_binding = args.attribute_group_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:servicecatalog/appregistryAttributeGroupAssociation:AppregistryAttributeGroupAssociation"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationId".into(),
                    value: &application_id_binding,
                },
                register_interface::ObjectField {
                    name: "attributeGroupId".into(),
                    value: &attribute_group_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "applicationId".into(),
                },
                register_interface::ResultField {
                    name: "attributeGroupId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AppregistryAttributeGroupAssociationResult {
            application_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationId").unwrap(),
            ),
            attribute_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("attributeGroupId").unwrap(),
            ),
        }
    }
}
