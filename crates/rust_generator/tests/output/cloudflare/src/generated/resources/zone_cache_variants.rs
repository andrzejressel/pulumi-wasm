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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ZoneCacheVariantsArgs,
    ) -> ZoneCacheVariantsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let avifs_binding = args.avifs.get_output(context).get_inner();
        let bmps_binding = args.bmps.get_output(context).get_inner();
        let gifs_binding = args.gifs.get_output(context).get_inner();
        let jp2s_binding = args.jp2s.get_output(context).get_inner();
        let jpegs_binding = args.jpegs.get_output(context).get_inner();
        let jpg2s_binding = args.jpg2s.get_output(context).get_inner();
        let jpgs_binding = args.jpgs.get_output(context).get_inner();
        let pngs_binding = args.pngs.get_output(context).get_inner();
        let tiffs_binding = args.tiffs.get_output(context).get_inner();
        let tifs_binding = args.tifs.get_output(context).get_inner();
        let webps_binding = args.webps.get_output(context).get_inner();
        let zone_id_binding = args.zone_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/zoneCacheVariants:ZoneCacheVariants".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "avifs".into(),
                    value: &avifs_binding,
                },
                register_interface::ObjectField {
                    name: "bmps".into(),
                    value: &bmps_binding,
                },
                register_interface::ObjectField {
                    name: "gifs".into(),
                    value: &gifs_binding,
                },
                register_interface::ObjectField {
                    name: "jp2s".into(),
                    value: &jp2s_binding,
                },
                register_interface::ObjectField {
                    name: "jpegs".into(),
                    value: &jpegs_binding,
                },
                register_interface::ObjectField {
                    name: "jpg2s".into(),
                    value: &jpg2s_binding,
                },
                register_interface::ObjectField {
                    name: "jpgs".into(),
                    value: &jpgs_binding,
                },
                register_interface::ObjectField {
                    name: "pngs".into(),
                    value: &pngs_binding,
                },
                register_interface::ObjectField {
                    name: "tiffs".into(),
                    value: &tiffs_binding,
                },
                register_interface::ObjectField {
                    name: "tifs".into(),
                    value: &tifs_binding,
                },
                register_interface::ObjectField {
                    name: "webps".into(),
                    value: &webps_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ZoneCacheVariantsResult {
            avifs: pulumi_gestalt_rust::__private::into_domain(o.extract_field("avifs")),
            bmps: pulumi_gestalt_rust::__private::into_domain(o.extract_field("bmps")),
            gifs: pulumi_gestalt_rust::__private::into_domain(o.extract_field("gifs")),
            jp2s: pulumi_gestalt_rust::__private::into_domain(o.extract_field("jp2s")),
            jpegs: pulumi_gestalt_rust::__private::into_domain(o.extract_field("jpegs")),
            jpg2s: pulumi_gestalt_rust::__private::into_domain(o.extract_field("jpg2s")),
            jpgs: pulumi_gestalt_rust::__private::into_domain(o.extract_field("jpgs")),
            pngs: pulumi_gestalt_rust::__private::into_domain(o.extract_field("pngs")),
            tiffs: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tiffs")),
            tifs: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tifs")),
            webps: pulumi_gestalt_rust::__private::into_domain(o.extract_field("webps")),
            zone_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("zoneId"),
            ),
        }
    }
}
