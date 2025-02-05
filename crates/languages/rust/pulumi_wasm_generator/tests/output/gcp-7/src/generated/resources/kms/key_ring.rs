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
pub mod key_ring {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KeyRingArgs {
        /// The location for the KeyRing.
        /// A full list of valid locations can be found by running `gcloud kms locations list`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// The resource name for the KeyRing.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct KeyRingResult {
        /// The location for the KeyRing.
        /// A full list of valid locations can be found by running `gcloud kms locations list`.
        ///
        ///
        /// - - -
        pub location: pulumi_wasm_rust::Output<String>,
        /// The resource name for the KeyRing.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: KeyRingArgs,
    ) -> KeyRingResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:kms/keyRing:KeyRing".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        KeyRingResult {
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_wasm_rust::__private::into_domain(o.extract_field("project")),
        }
    }
}
