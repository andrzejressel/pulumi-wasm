/// Manages a Logic App Integration Account.
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
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       skuName: Standard
///       tags:
///         foo: bar
/// ```
///
/// ## Import
///
/// Logic App Integration Accounts can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:logicapps/integrationAccount:IntegrationAccount example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Logic/integrationAccounts/account1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod integration_account {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IntegrationAccountArgs {
        /// The resource ID of the Integration Service Environment. Changing this forces a new Logic App Integration Account to be created.
        #[builder(into, default)]
        pub integration_service_environment_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The Azure Region where the Logic App Integration Account should exist. Changing this forces a new Logic App Integration Account to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Logic App Integration Account. Changing this forces a new Logic App Integration Account to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Logic App Integration Account should exist. Changing this forces a new Logic App Integration Account to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The SKU name of the Logic App Integration Account. Possible Values are `Basic`, `Free` and `Standard`.
        #[builder(into)]
        pub sku_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Logic App Integration Account.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct IntegrationAccountResult {
        /// The resource ID of the Integration Service Environment. Changing this forces a new Logic App Integration Account to be created.
        pub integration_service_environment_id: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// The Azure Region where the Logic App Integration Account should exist. Changing this forces a new Logic App Integration Account to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Logic App Integration Account. Changing this forces a new Logic App Integration Account to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Logic App Integration Account should exist. Changing this forces a new Logic App Integration Account to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The SKU name of the Logic App Integration Account. Possible Values are `Basic`, `Free` and `Standard`.
        pub sku_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Logic App Integration Account.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: IntegrationAccountArgs,
    ) -> IntegrationAccountResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let integration_service_environment_id_binding = args
            .integration_service_environment_id
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let sku_name_binding = args.sku_name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:logicapps/integrationAccount:IntegrationAccount".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "integrationServiceEnvironmentId".into(),
                    value: &integration_service_environment_id_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "skuName".into(),
                    value: &sku_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        IntegrationAccountResult {
            integration_service_environment_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("integrationServiceEnvironmentId"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            sku_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("skuName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
