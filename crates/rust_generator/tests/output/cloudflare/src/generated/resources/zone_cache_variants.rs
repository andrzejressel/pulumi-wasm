/// Provides a resource which customizes Cloudflare zone cache variants.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = zone_cache_variants::create(
///         "example",
///         ZoneCacheVariantsArgs::builder()
///             .avifs(vec!["image/avif", "image/webp",])
///             .bmps(vec!["image/bmp", "image/webp",])
///             .gifs(vec!["image/gif", "image/webp",])
///             .jp_2_s(vec!["image/jp2", "image/webp",])
///             .jpegs(vec!["image/jpeg", "image/webp",])
///             .jpg_2_s(vec!["image/jpg2", "image/webp",])
///             .jpgs(vec!["image/jpg", "image/webp",])
///             .pngs(vec!["image/png", "image/webp",])
///             .tiffs(vec!["image/tiff", "image/webp",])
///             .tifs(vec!["image/tif", "image/webp",])
///             .webps(vec!["image/jpeg", "image/webp",])
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod zone_cache_variants {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZoneCacheVariantsArgs {
        /// List of strings with the MIME types of all the variants that should be served for avif.
        #[builder(into, default)]
        pub avifs: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// List of strings with the MIME types of all the variants that should be served for bmp.
        #[builder(into, default)]
        pub bmps: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// List of strings with the MIME types of all the variants that should be served for gif.
        #[builder(into, default)]
        pub gifs: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// List of strings with the MIME types of all the variants that should be served for jp2.
        #[builder(into, default)]
        pub jp2s: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// List of strings with the MIME types of all the variants that should be served for jpeg.
        #[builder(into, default)]
        pub jpegs: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// List of strings with the MIME types of all the variants that should be served for jpg2.
        #[builder(into, default)]
        pub jpg2s: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// List of strings with the MIME types of all the variants that should be served for jpg.
        #[builder(into, default)]
        pub jpgs: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// List of strings with the MIME types of all the variants that should be served for png.
        #[builder(into, default)]
        pub pngs: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// List of strings with the MIME types of all the variants that should be served for tiff.
        #[builder(into, default)]
        pub tiffs: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// List of strings with the MIME types of all the variants that should be served for tif.
        #[builder(into, default)]
        pub tifs: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// List of strings with the MIME types of all the variants that should be served for webp.
        #[builder(into, default)]
        pub webps: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ZoneCacheVariantsResult {
        /// List of strings with the MIME types of all the variants that should be served for avif.
        pub avifs: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// List of strings with the MIME types of all the variants that should be served for bmp.
        pub bmps: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// List of strings with the MIME types of all the variants that should be served for gif.
        pub gifs: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// List of strings with the MIME types of all the variants that should be served for jp2.
        pub jp2s: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// List of strings with the MIME types of all the variants that should be served for jpeg.
        pub jpegs: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// List of strings with the MIME types of all the variants that should be served for jpg2.
        pub jpg2s: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// List of strings with the MIME types of all the variants that should be served for jpg.
        pub jpgs: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// List of strings with the MIME types of all the variants that should be served for png.
        pub pngs: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// List of strings with the MIME types of all the variants that should be served for tiff.
        pub tiffs: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// List of strings with the MIME types of all the variants that should be served for tif.
        pub tifs: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// List of strings with the MIME types of all the variants that should be served for webp.
        pub webps: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ZoneCacheVariantsArgs,
    ) -> ZoneCacheVariantsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let avifs_binding = args.avifs.get_output(context);
        let bmps_binding = args.bmps.get_output(context);
        let gifs_binding = args.gifs.get_output(context);
        let jp2s_binding = args.jp2s.get_output(context);
        let jpegs_binding = args.jpegs.get_output(context);
        let jpg2s_binding = args.jpg2s.get_output(context);
        let jpgs_binding = args.jpgs.get_output(context);
        let pngs_binding = args.pngs.get_output(context);
        let tiffs_binding = args.tiffs.get_output(context);
        let tifs_binding = args.tifs.get_output(context);
        let webps_binding = args.webps.get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/zoneCacheVariants:ZoneCacheVariants".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "avifs".into(),
                    value: &avifs_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bmps".into(),
                    value: &bmps_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gifs".into(),
                    value: &gifs_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "jp2s".into(),
                    value: &jp2s_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "jpegs".into(),
                    value: &jpegs_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "jpg2s".into(),
                    value: &jpg2s_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "jpgs".into(),
                    value: &jpgs_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pngs".into(),
                    value: &pngs_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tiffs".into(),
                    value: &tiffs_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tifs".into(),
                    value: &tifs_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "webps".into(),
                    value: &webps_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ZoneCacheVariantsResult {
            avifs: o.get_field("avifs"),
            bmps: o.get_field("bmps"),
            gifs: o.get_field("gifs"),
            jp2s: o.get_field("jp2s"),
            jpegs: o.get_field("jpegs"),
            jpg2s: o.get_field("jpg2s"),
            jpgs: o.get_field("jpgs"),
            pngs: o.get_field("pngs"),
            tiffs: o.get_field("tiffs"),
            tifs: o.get_field("tifs"),
            webps: o.get_field("webps"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
