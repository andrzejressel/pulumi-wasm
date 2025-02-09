/// Manages a Blob Event Trigger inside an Azure Data Factory.
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
///   exampleFactory:
///     type: azure:datafactory:Factory
///     name: example
///     properties:
///       name: example
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   examplePipeline:
///     type: azure:datafactory:Pipeline
///     name: example
///     properties:
///       name: example
///       dataFactoryId: ${exampleFactory.id}
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///   exampleTriggerBlobEvent:
///     type: azure:datafactory:TriggerBlobEvent
///     name: example
///     properties:
///       name: example
///       dataFactoryId: ${exampleFactory.id}
///       storageAccountId: ${exampleAccount.id}
///       events:
///         - Microsoft.Storage.BlobCreated
///         - Microsoft.Storage.BlobDeleted
///       blobPathEndsWith: .txt
///       ignoreEmptyBlobs: true
///       activated: true
///       annotations:
///         - test1
///         - test2
///         - test3
///       description: example description
///       pipelines:
///         - name: ${examplePipeline.name}
///           parameters:
///             Env: Prod
///       additionalProperties:
///         foo: foo1
///         bar: bar2
/// ```
///
/// ## Import
///
/// Data Factory Blob Event Trigger can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datafactory/triggerBlobEvent:TriggerBlobEvent example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example/providers/Microsoft.DataFactory/factories/example/triggers/example
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod trigger_blob_event {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TriggerBlobEventArgs {
        /// Specifies if the Data Factory Blob Event Trigger is activated. Defaults to `true`.
        #[builder(into, default)]
        pub activated: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A map of additional properties to associate with the Data Factory Blob Event Trigger.
        #[builder(into, default)]
        pub additional_properties: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Blob Event Trigger.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The pattern that blob path starts with for trigger to fire.
        #[builder(into, default)]
        pub blob_path_begins_with: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The pattern that blob path ends with for trigger to fire.
        ///
        /// > **Note:** At least one of `blob_path_begins_with` and `blob_path_ends_with` must be set.
        #[builder(into, default)]
        pub blob_path_ends_with: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of Data Factory in which to associate the Trigger with. Changing this forces a new resource.
        #[builder(into)]
        pub data_factory_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The description for the Data Factory Blob Event Trigger.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of events that will fire this trigger. Possible values are `Microsoft.Storage.BlobCreated` and `Microsoft.Storage.BlobDeleted`.
        #[builder(into)]
        pub events: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// are blobs with zero bytes ignored?
        #[builder(into, default)]
        pub ignore_empty_blobs: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the name of the Data Factory Blob Event Trigger. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `pipeline` blocks as defined below.
        #[builder(into)]
        pub pipelines: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::datafactory::TriggerBlobEventPipeline>,
        >,
        /// The ID of Storage Account in which blob event will be listened. Changing this forces a new resource.
        #[builder(into)]
        pub storage_account_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TriggerBlobEventResult {
        /// Specifies if the Data Factory Blob Event Trigger is activated. Defaults to `true`.
        pub activated: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A map of additional properties to associate with the Data Factory Blob Event Trigger.
        pub additional_properties: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Blob Event Trigger.
        pub annotations: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The pattern that blob path starts with for trigger to fire.
        pub blob_path_begins_with: pulumi_gestalt_rust::Output<Option<String>>,
        /// The pattern that blob path ends with for trigger to fire.
        ///
        /// > **Note:** At least one of `blob_path_begins_with` and `blob_path_ends_with` must be set.
        pub blob_path_ends_with: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of Data Factory in which to associate the Trigger with. Changing this forces a new resource.
        pub data_factory_id: pulumi_gestalt_rust::Output<String>,
        /// The description for the Data Factory Blob Event Trigger.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// List of events that will fire this trigger. Possible values are `Microsoft.Storage.BlobCreated` and `Microsoft.Storage.BlobDeleted`.
        pub events: pulumi_gestalt_rust::Output<Vec<String>>,
        /// are blobs with zero bytes ignored?
        pub ignore_empty_blobs: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the name of the Data Factory Blob Event Trigger. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// One or more `pipeline` blocks as defined below.
        pub pipelines: pulumi_gestalt_rust::Output<
            Vec<super::super::types::datafactory::TriggerBlobEventPipeline>,
        >,
        /// The ID of Storage Account in which blob event will be listened. Changing this forces a new resource.
        pub storage_account_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TriggerBlobEventArgs,
    ) -> TriggerBlobEventResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let activated_binding = args.activated.get_output(context);
        let additional_properties_binding = args
            .additional_properties
            .get_output(context);
        let annotations_binding = args.annotations.get_output(context);
        let blob_path_begins_with_binding = args
            .blob_path_begins_with
            .get_output(context);
        let blob_path_ends_with_binding = args.blob_path_ends_with.get_output(context);
        let data_factory_id_binding = args.data_factory_id.get_output(context);
        let description_binding = args.description.get_output(context);
        let events_binding = args.events.get_output(context);
        let ignore_empty_blobs_binding = args.ignore_empty_blobs.get_output(context);
        let name_binding = args.name.get_output(context);
        let pipelines_binding = args.pipelines.get_output(context);
        let storage_account_id_binding = args.storage_account_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:datafactory/triggerBlobEvent:TriggerBlobEvent".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "activated".into(),
                    value: activated_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "additionalProperties".into(),
                    value: additional_properties_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "annotations".into(),
                    value: annotations_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "blobPathBeginsWith".into(),
                    value: blob_path_begins_with_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "blobPathEndsWith".into(),
                    value: blob_path_ends_with_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataFactoryId".into(),
                    value: data_factory_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "events".into(),
                    value: events_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ignoreEmptyBlobs".into(),
                    value: ignore_empty_blobs_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pipelines".into(),
                    value: pipelines_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccountId".into(),
                    value: storage_account_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        TriggerBlobEventResult {
            activated: o.get_field("activated"),
            additional_properties: o.get_field("additionalProperties"),
            annotations: o.get_field("annotations"),
            blob_path_begins_with: o.get_field("blobPathBeginsWith"),
            blob_path_ends_with: o.get_field("blobPathEndsWith"),
            data_factory_id: o.get_field("dataFactoryId"),
            description: o.get_field("description"),
            events: o.get_field("events"),
            ignore_empty_blobs: o.get_field("ignoreEmptyBlobs"),
            name: o.get_field("name"),
            pipelines: o.get_field("pipelines"),
            storage_account_id: o.get_field("storageAccountId"),
        }
    }
}
