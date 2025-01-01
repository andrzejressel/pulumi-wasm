/// Manages a Gallery Application.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-rg")
///             .build_struct(),
///     );
///     let exampleGalleryApplication = gallery_application::create(
///         "exampleGalleryApplication",
///         GalleryApplicationArgs::builder()
///             .gallery_id("${exampleSharedImageGallery.id}")
///             .location("${example.location}")
///             .name("example-app")
///             .supported_os_type("Linux")
///             .build_struct(),
///     );
///     let exampleSharedImageGallery = shared_image_gallery::create(
///         "exampleSharedImageGallery",
///         SharedImageGalleryArgs::builder()
///             .location("${example.location}")
///             .name("examplegallery")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Gallery Applications can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:compute/galleryApplication:GalleryApplication example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Compute/galleries/gallery1/applications/galleryApplication1
/// ```
///
pub mod gallery_application {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GalleryApplicationArgs {
        /// A description of the Gallery Application.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The end of life date in RFC3339 format of the Gallery Application.
        #[builder(into, default)]
        pub end_of_life_date: pulumi_wasm_rust::Output<Option<String>>,
        /// The End User Licence Agreement of the Gallery Application.
        #[builder(into, default)]
        pub eula: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Shared Image Gallery. Changing this forces a new resource to be created.
        #[builder(into)]
        pub gallery_id: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the Gallery Application exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Gallery Application. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The URI containing the Privacy Statement associated with the Gallery Application.
        #[builder(into, default)]
        pub privacy_statement_uri: pulumi_wasm_rust::Output<Option<String>>,
        /// The URI containing the Release Notes associated with the Gallery Application.
        #[builder(into, default)]
        pub release_note_uri: pulumi_wasm_rust::Output<Option<String>>,
        /// The type of the Operating System supported for the Gallery Application. Possible values are `Linux` and `Windows`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub supported_os_type: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the Gallery Application.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GalleryApplicationResult {
        /// A description of the Gallery Application.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The end of life date in RFC3339 format of the Gallery Application.
        pub end_of_life_date: pulumi_wasm_rust::Output<Option<String>>,
        /// The End User Licence Agreement of the Gallery Application.
        pub eula: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Shared Image Gallery. Changing this forces a new resource to be created.
        pub gallery_id: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the Gallery Application exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the Gallery Application. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The URI containing the Privacy Statement associated with the Gallery Application.
        pub privacy_statement_uri: pulumi_wasm_rust::Output<Option<String>>,
        /// The URI containing the Release Notes associated with the Gallery Application.
        pub release_note_uri: pulumi_wasm_rust::Output<Option<String>>,
        /// The type of the Operating System supported for the Gallery Application. Possible values are `Linux` and `Windows`. Changing this forces a new resource to be created.
        pub supported_os_type: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the Gallery Application.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: GalleryApplicationArgs) -> GalleryApplicationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let end_of_life_date_binding = args.end_of_life_date.get_inner();
        let eula_binding = args.eula.get_inner();
        let gallery_id_binding = args.gallery_id.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let privacy_statement_uri_binding = args.privacy_statement_uri.get_inner();
        let release_note_uri_binding = args.release_note_uri.get_inner();
        let supported_os_type_binding = args.supported_os_type.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:compute/galleryApplication:GalleryApplication".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "endOfLifeDate".into(),
                    value: &end_of_life_date_binding,
                },
                register_interface::ObjectField {
                    name: "eula".into(),
                    value: &eula_binding,
                },
                register_interface::ObjectField {
                    name: "galleryId".into(),
                    value: &gallery_id_binding,
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
                    name: "privacyStatementUri".into(),
                    value: &privacy_statement_uri_binding,
                },
                register_interface::ObjectField {
                    name: "releaseNoteUri".into(),
                    value: &release_note_uri_binding,
                },
                register_interface::ObjectField {
                    name: "supportedOsType".into(),
                    value: &supported_os_type_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "endOfLifeDate".into(),
                },
                register_interface::ResultField {
                    name: "eula".into(),
                },
                register_interface::ResultField {
                    name: "galleryId".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "privacyStatementUri".into(),
                },
                register_interface::ResultField {
                    name: "releaseNoteUri".into(),
                },
                register_interface::ResultField {
                    name: "supportedOsType".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GalleryApplicationResult {
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            end_of_life_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endOfLifeDate").unwrap(),
            ),
            eula: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eula").unwrap(),
            ),
            gallery_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("galleryId").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            privacy_statement_uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privacyStatementUri").unwrap(),
            ),
            release_note_uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("releaseNoteUri").unwrap(),
            ),
            supported_os_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportedOsType").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
