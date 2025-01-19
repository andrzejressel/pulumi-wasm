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
pub mod configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConfigurationArgs {
        /// The in guest user patch mode. Possible values are `Platform` or `User`. Must be specified when `scope` is `InGuestPatch`.
        #[builder(into, default)]
        pub in_guest_user_patch_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// An `install_patches` block as defined below.
        ///
        /// > **NOTE:** `install_patches` must be specified when `scope` is `InGuestPatch`.
        #[builder(into, default)]
        pub install_patches: pulumi_wasm_rust::Output<
            Option<super::super::types::maintenance::ConfigurationInstallPatches>,
        >,
        /// Specified the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Maintenance Configuration. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of properties to assign to the resource.
        #[builder(into, default)]
        pub properties: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the Resource Group where the Maintenance Configuration should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The scope of the Maintenance Configuration. Possible values are `Extension`, `Host`, `InGuestPatch`, `OSImage`, `SQLDB` or `SQLManagedInstance`.
        #[builder(into)]
        pub scope: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource. The key could not contain upper case letter.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The visibility of the Maintenance Configuration. The only allowable value is `Custom`. Defaults to `Custom`.
        #[builder(into, default)]
        pub visibility: pulumi_wasm_rust::Output<Option<String>>,
        /// A `window` block as defined below.
        #[builder(into, default)]
        pub window: pulumi_wasm_rust::Output<
            Option<super::super::types::maintenance::ConfigurationWindow>,
        >,
    }
    #[allow(dead_code)]
    pub struct ConfigurationResult {
        /// The in guest user patch mode. Possible values are `Platform` or `User`. Must be specified when `scope` is `InGuestPatch`.
        pub in_guest_user_patch_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// An `install_patches` block as defined below.
        ///
        /// > **NOTE:** `install_patches` must be specified when `scope` is `InGuestPatch`.
        pub install_patches: pulumi_wasm_rust::Output<
            Option<super::super::types::maintenance::ConfigurationInstallPatches>,
        >,
        /// Specified the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Maintenance Configuration. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A mapping of properties to assign to the resource.
        pub properties: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the Resource Group where the Maintenance Configuration should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The scope of the Maintenance Configuration. Possible values are `Extension`, `Host`, `InGuestPatch`, `OSImage`, `SQLDB` or `SQLManagedInstance`.
        pub scope: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource. The key could not contain upper case letter.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The visibility of the Maintenance Configuration. The only allowable value is `Custom`. Defaults to `Custom`.
        pub visibility: pulumi_wasm_rust::Output<Option<String>>,
        /// A `window` block as defined below.
        pub window: pulumi_wasm_rust::Output<
            Option<super::super::types::maintenance::ConfigurationWindow>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ConfigurationArgs) -> ConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let in_guest_user_patch_mode_binding = args.in_guest_user_patch_mode.get_inner();
        let install_patches_binding = args.install_patches.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let properties_binding = args.properties.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let scope_binding = args.scope.get_inner();
        let tags_binding = args.tags.get_inner();
        let visibility_binding = args.visibility.get_inner();
        let window_binding = args.window.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:maintenance/configuration:Configuration".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "inGuestUserPatchMode".into(),
                    value: &in_guest_user_patch_mode_binding,
                },
                register_interface::ObjectField {
                    name: "installPatches".into(),
                    value: &install_patches_binding,
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
                    name: "properties".into(),
                    value: &properties_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "scope".into(),
                    value: &scope_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "visibility".into(),
                    value: &visibility_binding,
                },
                register_interface::ObjectField {
                    name: "window".into(),
                    value: &window_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "inGuestUserPatchMode".into(),
                },
                register_interface::ResultField {
                    name: "installPatches".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "properties".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "scope".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "visibility".into(),
                },
                register_interface::ResultField {
                    name: "window".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ConfigurationResult {
            in_guest_user_patch_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("inGuestUserPatchMode").unwrap(),
            ),
            install_patches: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("installPatches").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("properties").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            scope: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scope").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            visibility: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("visibility").unwrap(),
            ),
            window: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("window").unwrap(),
            ),
        }
    }
}
