/// Manages an Azure Data Factory (Version 2).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleFactory = factory::create(
///         "exampleFactory",
///         FactoryArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Data Factory can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datafactory/factory:Factory example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example/providers/Microsoft.DataFactory/factories/example
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod factory {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FactoryArgs {
        /// Specifies the Azure Key Vault Key ID to be used as the Customer Managed Key (CMK) for double encryption. Required with user assigned identity.
        #[builder(into, default)]
        pub customer_managed_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the ID of the user assigned identity associated with the Customer Managed Key. Must be supplied if `customer_managed_key_id` is set.
        #[builder(into, default)]
        pub customer_managed_key_identity_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// A `github_configuration` block as defined below.
        #[builder(into, default)]
        pub github_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::datafactory::FactoryGithubConfiguration>,
        >,
        /// A list of `global_parameter` blocks as defined above.
        #[builder(into, default)]
        pub global_parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::datafactory::FactoryGlobalParameter>>,
        >,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::datafactory::FactoryIdentity>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Is Managed Virtual Network enabled?
        #[builder(into, default)]
        pub managed_virtual_network_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Specifies the name of the Data Factory. Changing this forces a new resource to be created. Must be globally unique. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Is the Data Factory visible to the public network? Defaults to `true`.
        #[builder(into, default)]
        pub public_network_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the ID of the purview account resource associated with the Data Factory.
        #[builder(into, default)]
        pub purview_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which to create the Data Factory. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `vsts_configuration` block as defined below.
        #[builder(into, default)]
        pub vsts_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::datafactory::FactoryVstsConfiguration>,
        >,
    }
    #[allow(dead_code)]
    pub struct FactoryResult {
        /// Specifies the Azure Key Vault Key ID to be used as the Customer Managed Key (CMK) for double encryption. Required with user assigned identity.
        pub customer_managed_key_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the ID of the user assigned identity associated with the Customer Managed Key. Must be supplied if `customer_managed_key_id` is set.
        pub customer_managed_key_identity_id: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// A `github_configuration` block as defined below.
        pub github_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::datafactory::FactoryGithubConfiguration>,
        >,
        /// A list of `global_parameter` blocks as defined above.
        pub global_parameters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::datafactory::FactoryGlobalParameter>>,
        >,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::datafactory::FactoryIdentity>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Is Managed Virtual Network enabled?
        pub managed_virtual_network_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the name of the Data Factory. Changing this forces a new resource to be created. Must be globally unique. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Is the Data Factory visible to the public network? Defaults to `true`.
        pub public_network_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the ID of the purview account resource associated with the Data Factory.
        pub purview_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the Data Factory. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `vsts_configuration` block as defined below.
        pub vsts_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::datafactory::FactoryVstsConfiguration>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FactoryArgs,
    ) -> FactoryResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let customer_managed_key_id_binding = args
            .customer_managed_key_id
            .get_output(context);
        let customer_managed_key_identity_id_binding = args
            .customer_managed_key_identity_id
            .get_output(context);
        let github_configuration_binding = args.github_configuration.get_output(context);
        let global_parameters_binding = args.global_parameters.get_output(context);
        let identity_binding = args.identity.get_output(context);
        let location_binding = args.location.get_output(context);
        let managed_virtual_network_enabled_binding = args
            .managed_virtual_network_enabled
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let public_network_enabled_binding = args
            .public_network_enabled
            .get_output(context);
        let purview_id_binding = args.purview_id.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let vsts_configuration_binding = args.vsts_configuration.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:datafactory/factory:Factory".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customerManagedKeyId".into(),
                    value: &customer_managed_key_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customerManagedKeyIdentityId".into(),
                    value: &customer_managed_key_identity_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "githubConfiguration".into(),
                    value: &github_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "globalParameters".into(),
                    value: &global_parameters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: &identity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "managedVirtualNetworkEnabled".into(),
                    value: &managed_virtual_network_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicNetworkEnabled".into(),
                    value: &public_network_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "purviewId".into(),
                    value: &purview_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vstsConfiguration".into(),
                    value: &vsts_configuration_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        FactoryResult {
            customer_managed_key_id: o.get_field("customerManagedKeyId"),
            customer_managed_key_identity_id: o
                .get_field("customerManagedKeyIdentityId"),
            github_configuration: o.get_field("githubConfiguration"),
            global_parameters: o.get_field("globalParameters"),
            identity: o.get_field("identity"),
            location: o.get_field("location"),
            managed_virtual_network_enabled: o.get_field("managedVirtualNetworkEnabled"),
            name: o.get_field("name"),
            public_network_enabled: o.get_field("publicNetworkEnabled"),
            purview_id: o.get_field("purviewId"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
            vsts_configuration: o.get_field("vstsConfiguration"),
        }
    }
}
