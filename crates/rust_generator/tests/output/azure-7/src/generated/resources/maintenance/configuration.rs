/// Manages a maintenance configuration.
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
///   exampleConfiguration:
///     type: azure:maintenance:Configuration
///     name: example
///     properties:
///       name: example-mc
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       scope: SQLDB
///       tags:
///         Env: prod
/// ```
///
/// ## Import
///
/// Maintenance Configuration can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:maintenance/configuration:Configuration example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Maintenance/maintenanceConfigurations/example-mc
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConfigurationArgs {
        /// The in guest user patch mode. Possible values are `Platform` or `User`. Must be specified when `scope` is `InGuestPatch`.
        #[builder(into, default)]
        pub in_guest_user_patch_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An `install_patches` block as defined below.
        ///
        /// > **NOTE:** `install_patches` must be specified when `scope` is `InGuestPatch`.
        #[builder(into, default)]
        pub install_patches: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::maintenance::ConfigurationInstallPatches>,
        >,
        /// Specified the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Maintenance Configuration. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A mapping of properties to assign to the resource.
        #[builder(into, default)]
        pub properties: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the Resource Group where the Maintenance Configuration should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The scope of the Maintenance Configuration. Possible values are `Extension`, `Host`, `InGuestPatch`, `OSImage`, `SQLDB` or `SQLManagedInstance`.
        #[builder(into)]
        pub scope: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource. The key could not contain upper case letter.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The visibility of the Maintenance Configuration. The only allowable value is `Custom`. Defaults to `Custom`.
        #[builder(into, default)]
        pub visibility: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `window` block as defined below.
        #[builder(into, default)]
        pub window: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::maintenance::ConfigurationWindow>,
        >,
    }
    #[allow(dead_code)]
    pub struct ConfigurationResult {
        /// The in guest user patch mode. Possible values are `Platform` or `User`. Must be specified when `scope` is `InGuestPatch`.
        pub in_guest_user_patch_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// An `install_patches` block as defined below.
        ///
        /// > **NOTE:** `install_patches` must be specified when `scope` is `InGuestPatch`.
        pub install_patches: pulumi_gestalt_rust::Output<
            Option<super::super::types::maintenance::ConfigurationInstallPatches>,
        >,
        /// Specified the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Maintenance Configuration. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of properties to assign to the resource.
        pub properties: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the Resource Group where the Maintenance Configuration should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The scope of the Maintenance Configuration. Possible values are `Extension`, `Host`, `InGuestPatch`, `OSImage`, `SQLDB` or `SQLManagedInstance`.
        pub scope: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource. The key could not contain upper case letter.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The visibility of the Maintenance Configuration. The only allowable value is `Custom`. Defaults to `Custom`.
        pub visibility: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `window` block as defined below.
        pub window: pulumi_gestalt_rust::Output<
            Option<super::super::types::maintenance::ConfigurationWindow>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ConfigurationArgs,
    ) -> ConfigurationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let in_guest_user_patch_mode_binding = args
            .in_guest_user_patch_mode
            .get_output(context);
        let install_patches_binding = args.install_patches.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let properties_binding = args.properties.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let scope_binding = args.scope.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let visibility_binding = args.visibility.get_output(context);
        let window_binding = args.window.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:maintenance/configuration:Configuration".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "inGuestUserPatchMode".into(),
                    value: in_guest_user_patch_mode_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "installPatches".into(),
                    value: install_patches_binding.get_id(),
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
                    name: "properties".into(),
                    value: properties_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scope".into(),
                    value: scope_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "visibility".into(),
                    value: visibility_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "window".into(),
                    value: window_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ConfigurationResult {
            in_guest_user_patch_mode: o.get_field("inGuestUserPatchMode"),
            install_patches: o.get_field("installPatches"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            properties: o.get_field("properties"),
            resource_group_name: o.get_field("resourceGroupName"),
            scope: o.get_field("scope"),
            tags: o.get_field("tags"),
            visibility: o.get_field("visibility"),
            window: o.get_field("window"),
        }
    }
}
