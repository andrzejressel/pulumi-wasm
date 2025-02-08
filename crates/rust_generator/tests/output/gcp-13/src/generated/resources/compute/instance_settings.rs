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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod instance_settings {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceSettingsArgs {
        /// The metadata key/value pairs assigned to all the instances in the corresponding scope.
        /// Structure is documented below.
        #[builder(into, default)]
        pub metadata: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::InstanceSettingsMetadata>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A reference to the zone where the machine resides.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub zone: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct InstanceSettingsResult {
        /// The fingerprint used for optimistic locking of this resource.  Used
        /// internally during updates.
        pub fingerprint: pulumi_gestalt_rust::Output<String>,
        /// The metadata key/value pairs assigned to all the instances in the corresponding scope.
        /// Structure is documented below.
        pub metadata: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::InstanceSettingsMetadata>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// A reference to the zone where the machine resides.
        ///
        ///
        /// - - -
        pub zone: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: InstanceSettingsArgs,
    ) -> InstanceSettingsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let metadata_binding = args.metadata.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let zone_binding = args.zone.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/instanceSettings:InstanceSettings".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        InstanceSettingsResult {
            fingerprint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fingerprint"),
            ),
            metadata: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("metadata"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            zone: pulumi_gestalt_rust::__private::into_domain(o.extract_field("zone")),
        }
    }
}
