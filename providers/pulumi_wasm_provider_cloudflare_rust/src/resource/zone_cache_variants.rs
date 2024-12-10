//! Provides a resource which customizes Cloudflare zone cache variants.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = zone_cache_variants::create(
//!         "example",
//!         ZoneCacheVariantsArgs::builder()
//!             .avifs(vec!["image/avif", "image/webp",])
//!             .bmps(vec!["image/bmp", "image/webp",])
//!             .gifs(vec!["image/gif", "image/webp",])
//!             .jp_2_s(vec!["image/jp2", "image/webp",])
//!             .jpegs(vec!["image/jpeg", "image/webp",])
//!             .jpg_2_s(vec!["image/jpg2", "image/webp",])
//!             .jpgs(vec!["image/jpg", "image/webp",])
//!             .pngs(vec!["image/png", "image/webp",])
//!             .tiffs(vec!["image/tiff", "image/webp",])
//!             .tifs(vec!["image/tif", "image/webp",])
//!             .webps(vec!["image/jpeg", "image/webp",])
//!             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build_struct(),
//!     );
//! }
//! ```

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct ZoneCacheVariantsArgs {
    /// List of strings with the MIME types of all the variants that should be served for avif.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub avifs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// List of strings with the MIME types of all the variants that should be served for bmp.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub bmps: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// List of strings with the MIME types of all the variants that should be served for gif.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub gifs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// List of strings with the MIME types of all the variants that should be served for jp2.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub jp2s: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// List of strings with the MIME types of all the variants that should be served for jpeg.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub jpegs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// List of strings with the MIME types of all the variants that should be served for jpg2.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub jpg2s: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// List of strings with the MIME types of all the variants that should be served for jpg.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub jpgs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// List of strings with the MIME types of all the variants that should be served for png.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub pngs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// List of strings with the MIME types of all the variants that should be served for tiff.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub tiffs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// List of strings with the MIME types of all the variants that should be served for tif.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub tifs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// List of strings with the MIME types of all the variants that should be served for webp.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub webps: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct ZoneCacheVariantsResult {
    /// List of strings with the MIME types of all the variants that should be served for avif.
    pub avifs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// List of strings with the MIME types of all the variants that should be served for bmp.
    pub bmps: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// List of strings with the MIME types of all the variants that should be served for gif.
    pub gifs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// List of strings with the MIME types of all the variants that should be served for jp2.
    pub jp2s: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// List of strings with the MIME types of all the variants that should be served for jpeg.
    pub jpegs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// List of strings with the MIME types of all the variants that should be served for jpg2.
    pub jpg2s: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// List of strings with the MIME types of all the variants that should be served for jpg.
    pub jpgs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// List of strings with the MIME types of all the variants that should be served for png.
    pub pngs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// List of strings with the MIME types of all the variants that should be served for tiff.
    pub tiffs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// List of strings with the MIME types of all the variants that should be served for tif.
    pub tifs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// List of strings with the MIME types of all the variants that should be served for webp.
    pub webps: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ZoneCacheVariantsArgs) -> ZoneCacheVariantsResult {

    let result = crate::bindings::pulumi::cloudflare::zone_cache_variants::invoke(name, &crate::bindings::pulumi::cloudflare::zone_cache_variants::Args {
        avifs: &args.avifs.get_inner(),
        bmps: &args.bmps.get_inner(),
        gifs: &args.gifs.get_inner(),
        jp2s: &args.jp2s.get_inner(),
        jpegs: &args.jpegs.get_inner(),
        jpg2s: &args.jpg2s.get_inner(),
        jpgs: &args.jpgs.get_inner(),
        pngs: &args.pngs.get_inner(),
        tiffs: &args.tiffs.get_inner(),
        tifs: &args.tifs.get_inner(),
        webps: &args.webps.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    ZoneCacheVariantsResult {
        avifs: crate::into_domain(result.avifs),
        bmps: crate::into_domain(result.bmps),
        gifs: crate::into_domain(result.gifs),
        jp2s: crate::into_domain(result.jp2s),
        jpegs: crate::into_domain(result.jpegs),
        jpg2s: crate::into_domain(result.jpg2s),
        jpgs: crate::into_domain(result.jpgs),
        pngs: crate::into_domain(result.pngs),
        tiffs: crate::into_domain(result.tiffs),
        tifs: crate::into_domain(result.tifs),
        webps: crate::into_domain(result.webps),
        zone_id: crate::into_domain(result.zone_id),
    }
}

