/// Manages the Static Website of an Azure Storage Account.
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
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: storageaccountname
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: GRS
///       tags:
///         environment: staging
///   test:
///     type: azure:storage:AccountStaticWebsite
///     properties:
///       storageAccountId: ${testAzurermStorageAccount.id}
///       error404Document: custom_not_found.html
///       indexDocument: custom_index.html
/// ```
///
/// ## Import
///
/// Storage Account Static Websites can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:storage/accountStaticWebsite:AccountStaticWebsite mysite /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/myresourcegroup/providers/Microsoft.Storage/storageAccounts/myaccount
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod account_static_website {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountStaticWebsiteArgs {
        /// The absolute path to a custom webpage that should be used when a request is made which does not correspond to an existing file.
        #[builder(into, default)]
        pub error404_document: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The webpage that Azure Storage serves for requests to the root of a website or any subfolder. For example, index.html.
        #[builder(into, default)]
        pub index_document: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Storage Account to set Static Website on. Changing this forces a new resource to be created.
        #[builder(into)]
        pub storage_account_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AccountStaticWebsiteResult {
        /// The absolute path to a custom webpage that should be used when a request is made which does not correspond to an existing file.
        pub error404_document: pulumi_gestalt_rust::Output<Option<String>>,
        /// The webpage that Azure Storage serves for requests to the root of a website or any subfolder. For example, index.html.
        pub index_document: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the Storage Account to set Static Website on. Changing this forces a new resource to be created.
        pub storage_account_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: AccountStaticWebsiteArgs,
    ) -> AccountStaticWebsiteResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let error404_document_binding_1 = args.error404_document.get_output(context);
        let error404_document_binding = error404_document_binding_1.get_inner();
        let index_document_binding_1 = args.index_document.get_output(context);
        let index_document_binding = index_document_binding_1.get_inner();
        let storage_account_id_binding_1 = args.storage_account_id.get_output(context);
        let storage_account_id_binding = storage_account_id_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:storage/accountStaticWebsite:AccountStaticWebsite".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "error404Document".into(),
                    value: &error404_document_binding,
                },
                register_interface::ObjectField {
                    name: "indexDocument".into(),
                    value: &index_document_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountId".into(),
                    value: &storage_account_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AccountStaticWebsiteResult {
            error404_document: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("error404Document"),
            ),
            index_document: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("indexDocument"),
            ),
            storage_account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageAccountId"),
            ),
        }
    }
}
