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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod appregistry_attribute_group_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AppregistryAttributeGroupAssociationArgs {
        /// ID of the application.
        #[builder(into)]
        pub application_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ID of the attribute group to associate with the application.
        #[builder(into)]
        pub attribute_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AppregistryAttributeGroupAssociationResult {
        /// ID of the application.
        pub application_id: pulumi_gestalt_rust::Output<String>,
        /// ID of the attribute group to associate with the application.
        pub attribute_group_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: AppregistryAttributeGroupAssociationArgs,
    ) -> AppregistryAttributeGroupAssociationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let application_id_binding = args.application_id.get_output(context).get_inner();
        let attribute_group_id_binding = args
            .attribute_group_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:servicecatalog/appregistryAttributeGroupAssociation:AppregistryAttributeGroupAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        AppregistryAttributeGroupAssociationResult {
            application_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("applicationId"),
            ),
            attribute_group_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("attributeGroupId"),
            ),
        }
    }
}
