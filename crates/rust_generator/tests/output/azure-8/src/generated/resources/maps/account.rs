/// Manages an Azure Maps Account.
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
///   exampleAccount:
///     type: azure:maps:Account
///     name: example
///     properties:
///       name: example-maps-account
///       resourceGroupName: ${example.name}
///       skuName: S1
///       localAuthenticationEnabled: true
///       tags:
///         environment: Test
/// ```
///
/// ## Import
///
/// A Maps Account can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:maps/account:Account example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Maps/accounts/my-maps-account
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod account {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountArgs {
        /// A `cors` block as defined below
        #[builder(into, default)]
        pub cors: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::maps::AccountCors>,
        >,
        /// One or more `data_store` blocks as defined below.
        #[builder(into, default)]
        pub data_stores: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::maps::AccountDataStore>>,
        >,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::maps::AccountIdentity>,
        >,
        /// Is local authentication enabled for this Azure Maps Account? When `false`, all authentication to the Azure Maps data-plane REST API is disabled, except Azure AD authentication. Defaults to `true`.
        #[builder(into, default)]
        pub local_authentication_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The Location in which the Azure Maps Account should be provisioned. Changing this forces a new resource to be created. Defaults to `global`.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Azure Maps Account. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group in which the Azure Maps Account should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The SKU of the Azure Maps Account. Possible values are `S0`, `S1` and `G2`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** Gen1 SKUs (`S0` and `S1`) are deprecated and can no longer be used for new deployments, which should instead use a Gen2 SKU (`G2`) - more information can be found [in the Azure documentation](https://learn.microsoft.com/azure/azure-maps/how-to-manage-pricing-tier).
        #[builder(into)]
        pub sku_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the Azure Maps Account.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AccountResult {
        /// A `cors` block as defined below
        pub cors: pulumi_gestalt_rust::Output<
            Option<super::super::types::maps::AccountCors>,
        >,
        /// One or more `data_store` blocks as defined below.
        pub data_stores: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::maps::AccountDataStore>>,
        >,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::maps::AccountIdentity>,
        >,
        /// Is local authentication enabled for this Azure Maps Account? When `false`, all authentication to the Azure Maps data-plane REST API is disabled, except Azure AD authentication. Defaults to `true`.
        pub local_authentication_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The Location in which the Azure Maps Account should be provisioned. Changing this forces a new resource to be created. Defaults to `global`.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the Azure Maps Account. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The primary key used to authenticate and authorize access to the Maps REST APIs.
        pub primary_access_key: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group in which the Azure Maps Account should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The secondary key used to authenticate and authorize access to the Maps REST APIs.
        pub secondary_access_key: pulumi_gestalt_rust::Output<String>,
        /// The SKU of the Azure Maps Account. Possible values are `S0`, `S1` and `G2`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** Gen1 SKUs (`S0` and `S1`) are deprecated and can no longer be used for new deployments, which should instead use a Gen2 SKU (`G2`) - more information can be found [in the Azure documentation](https://learn.microsoft.com/azure/azure-maps/how-to-manage-pricing-tier).
        pub sku_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the Azure Maps Account.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A unique identifier for the Maps Account.
        pub x_ms_client_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AccountArgs,
    ) -> AccountResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cors_binding = args.cors.get_output(context);
        let data_stores_binding = args.data_stores.get_output(context);
        let identity_binding = args.identity.get_output(context);
        let local_authentication_enabled_binding = args
            .local_authentication_enabled
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let sku_name_binding = args.sku_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:maps/account:Account".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cors".into(),
                    value: cors_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataStores".into(),
                    value: data_stores_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: identity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "localAuthenticationEnabled".into(),
                    value: local_authentication_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skuName".into(),
                    value: sku_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AccountResult {
            cors: o.get_field("cors"),
            data_stores: o.get_field("dataStores"),
            identity: o.get_field("identity"),
            local_authentication_enabled: o.get_field("localAuthenticationEnabled"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            primary_access_key: o.get_field("primaryAccessKey"),
            resource_group_name: o.get_field("resourceGroupName"),
            secondary_access_key: o.get_field("secondaryAccessKey"),
            sku_name: o.get_field("skuName"),
            tags: o.get_field("tags"),
            x_ms_client_id: o.get_field("xMsClientId"),
        }
    }
}
