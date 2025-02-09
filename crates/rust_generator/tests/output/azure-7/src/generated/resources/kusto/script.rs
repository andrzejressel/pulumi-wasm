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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod script {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ScriptArgs {
        /// Flag that indicates whether to continue if one of the command fails.
        #[builder(into, default)]
        pub continue_on_errors_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The ID of the Kusto Database. Changing this forces a new Kusto Script to be created.
        #[builder(into)]
        pub database_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A unique string. If changed the script will be applied again.
        #[builder(into, default)]
        pub force_an_update_when_value_changed: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The name which should be used for this Kusto Script. Changing this forces a new Kusto Script to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The SAS token used to access the script. Must be provided when using scriptUrl property. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub sas_token: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The script content. This property should be used when the script is provide inline and not through file in a SA. Must not be used together with `url` and `sas_token` properties. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub script_content: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The url to the KQL script blob file. Must not be used together with scriptContent property. Please reference [this documentation](https://docs.microsoft.com/azure/data-explorer/database-script) that describes the commands that are allowed in the script.
        #[builder(into, default)]
        pub url: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ScriptResult {
        /// Flag that indicates whether to continue if one of the command fails.
        pub continue_on_errors_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The ID of the Kusto Database. Changing this forces a new Kusto Script to be created.
        pub database_id: pulumi_gestalt_rust::Output<String>,
        /// A unique string. If changed the script will be applied again.
        pub force_an_update_when_value_changed: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Kusto Script. Changing this forces a new Kusto Script to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The SAS token used to access the script. Must be provided when using scriptUrl property. Changing this forces a new resource to be created.
        pub sas_token: pulumi_gestalt_rust::Output<Option<String>>,
        /// The script content. This property should be used when the script is provide inline and not through file in a SA. Must not be used together with `url` and `sas_token` properties. Changing this forces a new resource to be created.
        pub script_content: pulumi_gestalt_rust::Output<Option<String>>,
        /// The url to the KQL script blob file. Must not be used together with scriptContent property. Please reference [this documentation](https://docs.microsoft.com/azure/data-explorer/database-script) that describes the commands that are allowed in the script.
        pub url: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ScriptArgs,
    ) -> ScriptResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let continue_on_errors_enabled_binding = args
            .continue_on_errors_enabled
            .get_output(context);
        let database_id_binding = args.database_id.get_output(context);
        let force_an_update_when_value_changed_binding = args
            .force_an_update_when_value_changed
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let sas_token_binding = args.sas_token.get_output(context);
        let script_content_binding = args.script_content.get_output(context);
        let url_binding = args.url.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:kusto/script:Script".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "continueOnErrorsEnabled".into(),
                    value: continue_on_errors_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "databaseId".into(),
                    value: database_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "forceAnUpdateWhenValueChanged".into(),
                    value: force_an_update_when_value_changed_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sasToken".into(),
                    value: sas_token_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scriptContent".into(),
                    value: script_content_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "url".into(),
                    value: url_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ScriptResult {
            continue_on_errors_enabled: o.get_field("continueOnErrorsEnabled"),
            database_id: o.get_field("databaseId"),
            force_an_update_when_value_changed: o
                .get_field("forceAnUpdateWhenValueChanged"),
            name: o.get_field("name"),
            sas_token: o.get_field("sasToken"),
            script_content: o.get_field("scriptContent"),
            url: o.get_field("url"),
        }
    }
}
