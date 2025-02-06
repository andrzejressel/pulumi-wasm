/// Manages a Container within an Azure Storage Account.
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
///       name: examplestoraccount
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///       tags:
///         environment: staging
///   exampleContainer:
///     type: azure:storage:Container
///     name: example
///     properties:
///       name: vhds
///       storageAccountId: ${exampleAccount.id}
///       containerAccessType: private
/// ```
///
/// ## Import
///
/// Storage Containers can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:storage/container:Container container1 https://example.blob.core.windows.net/container
/// ```
///
pub mod container {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ContainerArgs {
        /// The Access Level configured for this Container. Possible values are `blob`, `container` or `private`. Defaults to `private`.
        ///
        /// > **Note** When updating `container_access_type` for an existing storage container resource, Shared Key authentication will always be used, as AzureAD authentication is not supported.
        #[builder(into, default)]
        pub container_access_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The default encryption scope to use for blobs uploaded to this container. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub default_encryption_scope: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether to allow blobs to override the default encryption scope for this container. Can only be set when specifying `default_encryption_scope`. Defaults to `true`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub encryption_scope_override_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// A mapping of MetaData for this Container. All metadata keys should be lowercase.
        #[builder(into, default)]
        pub metadata: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the Container which should be created within the Storage Account. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Storage Account where the Container should be created. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** One of `storage_account_name` or `storage_account_id` must be specified. When specifying `storage_account_id` the resource will use the Resource Manager API, rather than the Data Plane API.
        #[builder(into, default)]
        pub storage_account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Storage Account where the Container should be created. Changing this forces a new resource to be created. This property is deprecated in favour of `storage_account_id`.
        #[builder(into, default)]
        pub storage_account_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ContainerResult {
        /// The Access Level configured for this Container. Possible values are `blob`, `container` or `private`. Defaults to `private`.
        ///
        /// > **Note** When updating `container_access_type` for an existing storage container resource, Shared Key authentication will always be used, as AzureAD authentication is not supported.
        pub container_access_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The default encryption scope to use for blobs uploaded to this container. Changing this forces a new resource to be created.
        pub default_encryption_scope: pulumi_gestalt_rust::Output<String>,
        /// Whether to allow blobs to override the default encryption scope for this container. Can only be set when specifying `default_encryption_scope`. Defaults to `true`. Changing this forces a new resource to be created.
        pub encryption_scope_override_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Is there an Immutability Policy configured on this Storage Container?
        pub has_immutability_policy: pulumi_gestalt_rust::Output<bool>,
        /// Is there a Legal Hold configured on this Storage Container?
        pub has_legal_hold: pulumi_gestalt_rust::Output<bool>,
        /// A mapping of MetaData for this Container. All metadata keys should be lowercase.
        pub metadata: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The name of the Container which should be created within the Storage Account. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Resource Manager ID of this Storage Container.
        pub resource_manager_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the Storage Account where the Container should be created. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** One of `storage_account_name` or `storage_account_id` must be specified. When specifying `storage_account_id` the resource will use the Resource Manager API, rather than the Data Plane API.
        pub storage_account_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Storage Account where the Container should be created. Changing this forces a new resource to be created. This property is deprecated in favour of `storage_account_id`.
        pub storage_account_name: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ContainerArgs,
    ) -> ContainerResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let container_access_type_binding = args
            .container_access_type
            .get_output(context)
            .get_inner();
        let default_encryption_scope_binding = args
            .default_encryption_scope
            .get_output(context)
            .get_inner();
        let encryption_scope_override_enabled_binding = args
            .encryption_scope_override_enabled
            .get_output(context)
            .get_inner();
        let metadata_binding = args.metadata.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let storage_account_id_binding = args
            .storage_account_id
            .get_output(context)
            .get_inner();
        let storage_account_name_binding = args
            .storage_account_name
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:storage/container:Container".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "containerAccessType".into(),
                    value: &container_access_type_binding,
                },
                register_interface::ObjectField {
                    name: "defaultEncryptionScope".into(),
                    value: &default_encryption_scope_binding,
                },
                register_interface::ObjectField {
                    name: "encryptionScopeOverrideEnabled".into(),
                    value: &encryption_scope_override_enabled_binding,
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
                    name: "storageAccountId".into(),
                    value: &storage_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountName".into(),
                    value: &storage_account_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ContainerResult {
            container_access_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("containerAccessType"),
            ),
            default_encryption_scope: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultEncryptionScope"),
            ),
            encryption_scope_override_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("encryptionScopeOverrideEnabled"),
            ),
            has_immutability_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hasImmutabilityPolicy"),
            ),
            has_legal_hold: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hasLegalHold"),
            ),
            metadata: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("metadata"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_manager_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceManagerId"),
            ),
            storage_account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageAccountId"),
            ),
            storage_account_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageAccountName"),
            ),
        }
    }
}
