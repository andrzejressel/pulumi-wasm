/// Manages a Gallery Application.
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
#[allow(clippy::doc_lazy_continuation)]
pub mod gallery_application {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GalleryApplicationArgs {
        /// A description of the Gallery Application.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The end of life date in RFC3339 format of the Gallery Application.
        #[builder(into, default)]
        pub end_of_life_date: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The End User Licence Agreement of the Gallery Application.
        #[builder(into, default)]
        pub eula: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Shared Image Gallery. Changing this forces a new resource to be created.
        #[builder(into)]
        pub gallery_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Azure Region where the Gallery Application exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Gallery Application. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The URI containing the Privacy Statement associated with the Gallery Application.
        #[builder(into, default)]
        pub privacy_statement_uri: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The URI containing the Release Notes associated with the Gallery Application.
        #[builder(into, default)]
        pub release_note_uri: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The type of the Operating System supported for the Gallery Application. Possible values are `Linux` and `Windows`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub supported_os_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the Gallery Application.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GalleryApplicationResult {
        /// A description of the Gallery Application.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The end of life date in RFC3339 format of the Gallery Application.
        pub end_of_life_date: pulumi_gestalt_rust::Output<Option<String>>,
        /// The End User Licence Agreement of the Gallery Application.
        pub eula: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the Shared Image Gallery. Changing this forces a new resource to be created.
        pub gallery_id: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the Gallery Application exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the Gallery Application. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The URI containing the Privacy Statement associated with the Gallery Application.
        pub privacy_statement_uri: pulumi_gestalt_rust::Output<Option<String>>,
        /// The URI containing the Release Notes associated with the Gallery Application.
        pub release_note_uri: pulumi_gestalt_rust::Output<Option<String>>,
        /// The type of the Operating System supported for the Gallery Application. Possible values are `Linux` and `Windows`. Changing this forces a new resource to be created.
        pub supported_os_type: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the Gallery Application.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: GalleryApplicationArgs,
    ) -> GalleryApplicationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let end_of_life_date_binding = args
            .end_of_life_date
            .get_output(context)
            .get_inner();
        let eula_binding = args.eula.get_output(context).get_inner();
        let gallery_id_binding = args.gallery_id.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let privacy_statement_uri_binding = args
            .privacy_statement_uri
            .get_output(context)
            .get_inner();
        let release_note_uri_binding = args
            .release_note_uri
            .get_output(context)
            .get_inner();
        let supported_os_type_binding = args
            .supported_os_type
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:compute/galleryApplication:GalleryApplication".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        GalleryApplicationResult {
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            end_of_life_date: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endOfLifeDate"),
            ),
            eula: pulumi_gestalt_rust::__private::into_domain(o.extract_field("eula")),
            gallery_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("galleryId"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            privacy_statement_uri: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privacyStatementUri"),
            ),
            release_note_uri: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("releaseNoteUri"),
            ),
            supported_os_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("supportedOsType"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
