/// Manages a Logic App Integration Account Map.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleIntegrationAccount:
///     type: azure:logicapps:IntegrationAccount
///     name: example
///     properties:
///       name: example-ia
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       skuName: Standard
///   exampleIntegrationAccountMap:
///     type: azure:logicapps:IntegrationAccountMap
///     name: example
///     properties:
///       name: example-iamap
///       resourceGroupName: ${example.name}
///       integrationAccountName: ${exampleIntegrationAccount.name}
///       mapType: Xslt
///       content:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: testdata/integration_account_map_content.xsd
///           return: result
/// ```
///
/// ## Import
///
/// Logic App Integration Account Maps can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:logicapps/integrationAccountMap:IntegrationAccountMap example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Logic/integrationAccounts/account1/maps/map1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod integration_account_map {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IntegrationAccountMapArgs {
        /// The content of the Logic App Integration Account Map.
        #[builder(into)]
        pub content: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Logic App Integration Account. Changing this forces a new Logic App Integration Account Map to be created.
        #[builder(into)]
        pub integration_account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type of the Logic App Integration Account Map. Possible values are `Liquid`, `NotSpecified`, `Xslt`, `Xslt30` and `Xslt20`.
        #[builder(into)]
        pub map_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The metadata of the Logic App Integration Account Map.
        #[builder(into, default)]
        pub metadata: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name which should be used for this Logic App Integration Account Map. Changing this forces a new Logic App Integration Account Map to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Logic App Integration Account Map should exist. Changing this forces a new Logic App Integration Account Map to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct IntegrationAccountMapResult {
        /// The content of the Logic App Integration Account Map.
        pub content: pulumi_gestalt_rust::Output<String>,
        /// The name of the Logic App Integration Account. Changing this forces a new Logic App Integration Account Map to be created.
        pub integration_account_name: pulumi_gestalt_rust::Output<String>,
        /// The type of the Logic App Integration Account Map. Possible values are `Liquid`, `NotSpecified`, `Xslt`, `Xslt30` and `Xslt20`.
        pub map_type: pulumi_gestalt_rust::Output<String>,
        /// The metadata of the Logic App Integration Account Map.
        pub metadata: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name which should be used for this Logic App Integration Account Map. Changing this forces a new Logic App Integration Account Map to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Logic App Integration Account Map should exist. Changing this forces a new Logic App Integration Account Map to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: IntegrationAccountMapArgs,
    ) -> IntegrationAccountMapResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let content_binding_1 = args.content.get_output(context);
        let content_binding = content_binding_1.get_inner();
        let integration_account_name_binding_1 = args
            .integration_account_name
            .get_output(context);
        let integration_account_name_binding = integration_account_name_binding_1
            .get_inner();
        let map_type_binding_1 = args.map_type.get_output(context);
        let map_type_binding = map_type_binding_1.get_inner();
        let metadata_binding_1 = args.metadata.get_output(context);
        let metadata_binding = metadata_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:logicapps/integrationAccountMap:IntegrationAccountMap".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "content".into(),
                    value: &content_binding,
                },
                register_interface::ObjectField {
                    name: "integrationAccountName".into(),
                    value: &integration_account_name_binding,
                },
                register_interface::ObjectField {
                    name: "mapType".into(),
                    value: &map_type_binding,
                },
                register_interface::ObjectField {
                    name: "metadata".into(),
                    value: &metadata_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        IntegrationAccountMapResult {
            content: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("content"),
            ),
            integration_account_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("integrationAccountName"),
            ),
            map_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("mapType"),
            ),
            metadata: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("metadata"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
        }
    }
}
