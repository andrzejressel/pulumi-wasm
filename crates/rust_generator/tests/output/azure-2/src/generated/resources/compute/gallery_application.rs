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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: GalleryApplicationArgs,
    ) -> GalleryApplicationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let end_of_life_date_binding = args.end_of_life_date.get_output(context);
        let eula_binding = args.eula.get_output(context);
        let gallery_id_binding = args.gallery_id.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let privacy_statement_uri_binding = args
            .privacy_statement_uri
            .get_output(context);
        let release_note_uri_binding = args.release_note_uri.get_output(context);
        let supported_os_type_binding = args.supported_os_type.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:compute/galleryApplication:GalleryApplication".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endOfLifeDate".into(),
                    value: end_of_life_date_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eula".into(),
                    value: eula_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "galleryId".into(),
                    value: gallery_id_binding.get_id(),
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
                    name: "privacyStatementUri".into(),
                    value: privacy_statement_uri_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "releaseNoteUri".into(),
                    value: release_note_uri_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "supportedOsType".into(),
                    value: supported_os_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        GalleryApplicationResult {
            description: o.get_field("description"),
            end_of_life_date: o.get_field("endOfLifeDate"),
            eula: o.get_field("eula"),
            gallery_id: o.get_field("galleryId"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            privacy_statement_uri: o.get_field("privacyStatementUri"),
            release_note_uri: o.get_field("releaseNoteUri"),
            supported_os_type: o.get_field("supportedOsType"),
            tags: o.get_field("tags"),
        }
    }
}
