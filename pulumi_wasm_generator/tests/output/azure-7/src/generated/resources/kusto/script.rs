/// Manages a Kusto Script.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: example
///       location: West Europe
///   exampleCluster:
///     type: azure:kusto:Cluster
///     name: example
///     properties:
///       name: example
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       sku:
///         name: Dev(No SLA)_Standard_D11_v2
///         capacity: 1
///   exampleDatabase:
///     type: azure:kusto:Database
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${exampleResourceGroup.name}
///       location: ${exampleResourceGroup.location}
///       clusterName: ${exampleCluster.name}
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${exampleResourceGroup.name}
///       location: ${exampleResourceGroup.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///   exampleContainer:
///     type: azure:storage:Container
///     name: example
///     properties:
///       name: setup-files
///       storageAccountName: ${exampleAccount.name}
///       containerAccessType: private
///   exampleBlob:
///     type: azure:storage:Blob
///     name: example
///     properties:
///       name: script.txt
///       storageAccountName: ${exampleAccount.name}
///       storageContainerName: ${exampleContainer.name}
///       type: Block
///       sourceContent: .create table MyTable (Level:string, Timestamp:datetime, UserId:string, TraceId:string, Message:string, ProcessId:int32)
///   exampleScript:
///     type: azure:kusto:Script
///     name: example
///     properties:
///       name: example
///       databaseId: ${exampleDatabase.id}
///       url: ${exampleBlob.id}
///       sasToken: ${example.sas}
///       continueOnErrorsEnabled: true
///       forceAnUpdateWhenValueChanged: first
/// variables:
///   example:
///     fn::invoke:
///       function: azure:storage:getAccountBlobContainerSAS
///       arguments:
///         connectionString: ${exampleAccount.primaryConnectionString}
///         containerName: ${exampleContainer.name}
///         httpsOnly: true
///         start: 2017-03-21
///         expiry: 2022-03-21
///         permissions:
///           read: true
///           add: false
///           create: false
///           write: true
///           delete: false
///           list: true
/// ```
///
/// ## Import
///
/// Kusto Scripts can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:kusto/script:Script example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resGroup1/providers/Microsoft.Kusto/clusters/cluster1/databases/database1/scripts/script1
/// ```
///
pub mod script {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ScriptArgs {
        /// Flag that indicates whether to continue if one of the command fails.
        #[builder(into, default)]
        pub continue_on_errors_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the Kusto Database. Changing this forces a new Kusto Script to be created.
        #[builder(into)]
        pub database_id: pulumi_wasm_rust::Output<String>,
        /// A unique string. If changed the script will be applied again.
        #[builder(into, default)]
        pub force_an_update_when_value_changed: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Kusto Script. Changing this forces a new Kusto Script to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The SAS token used to access the script. Must be provided when using scriptUrl property. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub sas_token: pulumi_wasm_rust::Output<Option<String>>,
        /// The script content. This property should be used when the script is provide inline and not through file in a SA. Must not be used together with `url` and `sas_token` properties. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub script_content: pulumi_wasm_rust::Output<Option<String>>,
        /// The url to the KQL script blob file. Must not be used together with scriptContent property. Please reference [this documentation](https://docs.microsoft.com/azure/data-explorer/database-script) that describes the commands that are allowed in the script.
        #[builder(into, default)]
        pub url: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ScriptResult {
        /// Flag that indicates whether to continue if one of the command fails.
        pub continue_on_errors_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the Kusto Database. Changing this forces a new Kusto Script to be created.
        pub database_id: pulumi_wasm_rust::Output<String>,
        /// A unique string. If changed the script will be applied again.
        pub force_an_update_when_value_changed: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Kusto Script. Changing this forces a new Kusto Script to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The SAS token used to access the script. Must be provided when using scriptUrl property. Changing this forces a new resource to be created.
        pub sas_token: pulumi_wasm_rust::Output<Option<String>>,
        /// The script content. This property should be used when the script is provide inline and not through file in a SA. Must not be used together with `url` and `sas_token` properties. Changing this forces a new resource to be created.
        pub script_content: pulumi_wasm_rust::Output<Option<String>>,
        /// The url to the KQL script blob file. Must not be used together with scriptContent property. Please reference [this documentation](https://docs.microsoft.com/azure/data-explorer/database-script) that describes the commands that are allowed in the script.
        pub url: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ScriptArgs) -> ScriptResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let continue_on_errors_enabled_binding = args
            .continue_on_errors_enabled
            .get_inner();
        let database_id_binding = args.database_id.get_inner();
        let force_an_update_when_value_changed_binding = args
            .force_an_update_when_value_changed
            .get_inner();
        let name_binding = args.name.get_inner();
        let sas_token_binding = args.sas_token.get_inner();
        let script_content_binding = args.script_content.get_inner();
        let url_binding = args.url.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:kusto/script:Script".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "continueOnErrorsEnabled".into(),
                    value: &continue_on_errors_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "databaseId".into(),
                    value: &database_id_binding,
                },
                register_interface::ObjectField {
                    name: "forceAnUpdateWhenValueChanged".into(),
                    value: &force_an_update_when_value_changed_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "sasToken".into(),
                    value: &sas_token_binding,
                },
                register_interface::ObjectField {
                    name: "scriptContent".into(),
                    value: &script_content_binding,
                },
                register_interface::ObjectField {
                    name: "url".into(),
                    value: &url_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "continueOnErrorsEnabled".into(),
                },
                register_interface::ResultField {
                    name: "databaseId".into(),
                },
                register_interface::ResultField {
                    name: "forceAnUpdateWhenValueChanged".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "sasToken".into(),
                },
                register_interface::ResultField {
                    name: "scriptContent".into(),
                },
                register_interface::ResultField {
                    name: "url".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ScriptResult {
            continue_on_errors_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("continueOnErrorsEnabled").unwrap(),
            ),
            database_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("databaseId").unwrap(),
            ),
            force_an_update_when_value_changed: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("forceAnUpdateWhenValueChanged").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            sas_token: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sasToken").unwrap(),
            ),
            script_content: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scriptContent").unwrap(),
            ),
            url: pulumi_wasm_rust::__private::into_domain(hashmap.remove("url").unwrap()),
        }
    }
}
