/// Represents an Instance Settings resource. Instance settings are centralized configuration parameters that allow users to configure the default values for specific VM parameters that are normally set using GCE instance API methods.
///
///
/// To get more information about InstanceSettings, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/beta/instanceSettings)
/// * How-to Guides
///     * [Update Instance Settings](https://cloud.google.com/compute/docs/metadata/setting-custom-metadata#set-custom-project-zonal-metadata)
///
/// ## Example Usage
///
/// ### Instance Settings Basic
///
///
/// ```yaml
/// resources:
///   gceInstanceSettings:
///     type: gcp:compute:InstanceSettings
///     name: gce_instance_settings
///     properties:
///       zone: us-east7-b
///       metadata:
///         items:
///           foo: baz
/// ```
///
/// ## Import
///
/// InstanceSettings can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/zones/{{zone}}/instanceSettings`
///
/// * `{{project}}/{{zone}}`
///
/// * `{{zone}}`
///
/// When using the `pulumi import` command, InstanceSettings can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/instanceSettings:InstanceSettings default projects/{{project}}/zones/{{zone}}/instanceSettings
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/instanceSettings:InstanceSettings default {{project}}/{{zone}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/instanceSettings:InstanceSettings default {{zone}}
/// ```
///
pub mod instance_settings {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceSettingsArgs {
        /// The metadata key/value pairs assigned to all the instances in the corresponding scope.
        /// Structure is documented below.
        #[builder(into, default)]
        pub metadata: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::InstanceSettingsMetadata>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// A reference to the zone where the machine resides.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub zone: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct InstanceSettingsResult {
        /// The fingerprint used for optimistic locking of this resource.  Used
        /// internally during updates.
        pub fingerprint: pulumi_wasm_rust::Output<String>,
        /// The metadata key/value pairs assigned to all the instances in the corresponding scope.
        /// Structure is documented below.
        pub metadata: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::InstanceSettingsMetadata>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// A reference to the zone where the machine resides.
        ///
        ///
        /// - - -
        pub zone: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: InstanceSettingsArgs) -> InstanceSettingsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let metadata_binding = args.metadata.get_inner();
        let project_binding = args.project.get_inner();
        let zone_binding = args.zone.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/instanceSettings:InstanceSettings".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "metadata".into(),
                    value: &metadata_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "zone".into(),
                    value: &zone_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "fingerprint".into(),
                },
                register_interface::ResultField {
                    name: "metadata".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "zone".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        InstanceSettingsResult {
            fingerprint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fingerprint").unwrap(),
            ),
            metadata: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metadata").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zone").unwrap(),
            ),
        }
    }
}
