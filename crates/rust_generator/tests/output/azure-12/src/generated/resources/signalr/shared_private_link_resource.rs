/// Manages the Shared Private Link Resource for a Signalr service.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: terraform-signalr
///       location: east us
///   exampleKeyVault:
///     type: azure:keyvault:KeyVault
///     name: example
///     properties:
///       name: examplekeyvault
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       tenantId: ${current.tenantId}
///       skuName: standard
///       softDeleteRetentionDays: 7
///       accessPolicies:
///         - tenantId: ${current.tenantId}
///           objectId: ${current.objectId}
///           certificatePermissions:
///             - ManageContacts
///           keyPermissions:
///             - Create
///           secretPermissions:
///             - Set
///   test:
///     type: azure:signalr:Service
///     properties:
///       name: tfex-signalr
///       location: ${testAzurermResourceGroup.location}
///       resourceGroupName: ${testAzurermResourceGroup.name}
///       sku:
///         name: Standard_S1
///         capacity: 1
///   exampleSharedPrivateLinkResource:
///     type: azure:signalr:SharedPrivateLinkResource
///     name: example
///     properties:
///       name: tfex-signalr-splr
///       signalrServiceId: ${exampleAzurermSignalrService.id}
///       subResourceName: vault
///       targetResourceId: ${exampleKeyVault.id}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Signalr Shared Private Link Resource can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:signalr/sharedPrivateLinkResource:SharedPrivateLinkResource example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.SignalRService/signalR/signalr1/sharedPrivateLinkResources/resource1
/// ```
///
pub mod shared_private_link_resource {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SharedPrivateLinkResourceArgs {
        /// The name of the Signalr Shared Private Link Resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The request message for requesting approval of the Shared Private Link Enabled Remote Resource.
        #[builder(into, default)]
        pub request_message: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The id of the Signalr Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub signalr_service_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The sub resource name which the Signalr Private Endpoint can connect to. Possible values are `sites`, `vault`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub sub_resource_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the Shared Private Link Enabled Remote Resource which this Signalr Private Endpoint should be connected to. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The `sub_resource_name` should match with the type of the `target_resource_id` that's being specified.
        #[builder(into)]
        pub target_resource_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SharedPrivateLinkResourceResult {
        /// The name of the Signalr Shared Private Link Resource. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The request message for requesting approval of the Shared Private Link Enabled Remote Resource.
        pub request_message: pulumi_wasm_rust::Output<Option<String>>,
        /// The id of the Signalr Service. Changing this forces a new resource to be created.
        pub signalr_service_id: pulumi_wasm_rust::Output<String>,
        /// The status of a private endpoint connection. Possible values are `Pending`, `Approved`, `Rejected` or `Disconnected`.
        pub status: pulumi_wasm_rust::Output<String>,
        /// The sub resource name which the Signalr Private Endpoint can connect to. Possible values are `sites`, `vault`. Changing this forces a new resource to be created.
        pub sub_resource_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Shared Private Link Enabled Remote Resource which this Signalr Private Endpoint should be connected to. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The `sub_resource_name` should match with the type of the `target_resource_id` that's being specified.
        pub target_resource_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: SharedPrivateLinkResourceArgs,
    ) -> SharedPrivateLinkResourceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let request_message_binding = args
            .request_message
            .get_output(context)
            .get_inner();
        let signalr_service_id_binding = args
            .signalr_service_id
            .get_output(context)
            .get_inner();
        let sub_resource_name_binding = args
            .sub_resource_name
            .get_output(context)
            .get_inner();
        let target_resource_id_binding = args
            .target_resource_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:signalr/sharedPrivateLinkResource:SharedPrivateLinkResource"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "requestMessage".into(),
                    value: &request_message_binding,
                },
                register_interface::ObjectField {
                    name: "signalrServiceId".into(),
                    value: &signalr_service_id_binding,
                },
                register_interface::ObjectField {
                    name: "subResourceName".into(),
                    value: &sub_resource_name_binding,
                },
                register_interface::ObjectField {
                    name: "targetResourceId".into(),
                    value: &target_resource_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SharedPrivateLinkResourceResult {
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            request_message: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("requestMessage"),
            ),
            signalr_service_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("signalrServiceId"),
            ),
            status: pulumi_wasm_rust::__private::into_domain(o.extract_field("status")),
            sub_resource_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("subResourceName"),
            ),
            target_resource_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("targetResourceId"),
            ),
        }
    }
}
