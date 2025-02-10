/// A `KeyRing` is a toplevel logical grouping of `CryptoKeys`.
///
///
/// > **Note:** KeyRings cannot be deleted from Google Cloud Platform.
/// Destroying a provider-managed KeyRing will remove it from state but
/// *will not delete the resource from the project.*
///
///
/// To get more information about KeyRing, see:
///
/// * [API documentation](https://cloud.google.com/kms/docs/reference/rest/v1/projects.locations.keyRings)
/// * How-to Guides
///     * [Creating a key ring](https://cloud.google.com/kms/docs/creating-keys#create_a_key_ring)
///
/// ## Example Usage
///
/// ### Kms Key Ring Basic
///
///
/// ```yaml
/// resources:
///   example-keyring:
///     type: gcp:kms:KeyRing
///     properties:
///       name: keyring-example
///       location: global
/// ```
///
/// ## Import
///
/// KeyRing can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/keyRings/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, KeyRing can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:kms/keyRing:KeyRing default projects/{{project}}/locations/{{location}}/keyRings/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:kms/keyRing:KeyRing default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:kms/keyRing:KeyRing default {{location}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod key_ring {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KeyRingArgs {
        /// The location for the KeyRing.
        /// A full list of valid locations can be found by running `gcloud kms locations list`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The resource name for the KeyRing.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct KeyRingResult {
        /// The location for the KeyRing.
        /// A full list of valid locations can be found by running `gcloud kms locations list`.
        ///
        ///
        /// - - -
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The resource name for the KeyRing.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: KeyRingArgs,
    ) -> KeyRingResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:kms/keyRing:KeyRing".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        KeyRingResult {
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
        }
    }
}
