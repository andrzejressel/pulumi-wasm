//! Provides a resource which customizes Cloudflare zone cache variants.
//!
//! ## Example Usage
//!
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//!
//! const example = new cloudflare.ZoneCacheVariants("example", {
//!     avifs: [
//!         "image/avif",
//!         "image/webp",
//!     ],
//!     bmps: [
//!         "image/bmp",
//!         "image/webp",
//!     ],
//!     gifs: [
//!         "image/gif",
//!         "image/webp",
//!     ],
//!     jp2s: [
//!         "image/jp2",
//!         "image/webp",
//!     ],
//!     jpegs: [
//!         "image/jpeg",
//!         "image/webp",
//!     ],
//!     jpgs: [
//!         "image/jpg",
//!         "image/webp",
//!     ],
//!     jpg2s: [
//!         "image/jpg2",
//!         "image/webp",
//!     ],
//!     pngs: [
//!         "image/png",
//!         "image/webp",
//!     ],
//!     tifs: [
//!         "image/tif",
//!         "image/webp",
//!     ],
//!     tiffs: [
//!         "image/tiff",
//!         "image/webp",
//!     ],
//!     webps: [
//!         "image/jpeg",
//!         "image/webp",
//!     ],
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//!
//! example = cloudflare.ZoneCacheVariants("example",
//!     avifs=[
//!         "image/avif",
//!         "image/webp",
//!     ],
//!     bmps=[
//!         "image/bmp",
//!         "image/webp",
//!     ],
//!     gifs=[
//!         "image/gif",
//!         "image/webp",
//!     ],
//!     jp2s=[
//!         "image/jp2",
//!         "image/webp",
//!     ],
//!     jpegs=[
//!         "image/jpeg",
//!         "image/webp",
//!     ],
//!     jpgs=[
//!         "image/jpg",
//!         "image/webp",
//!     ],
//!     jpg2s=[
//!         "image/jpg2",
//!         "image/webp",
//!     ],
//!     pngs=[
//!         "image/png",
//!         "image/webp",
//!     ],
//!     tifs=[
//!         "image/tif",
//!         "image/webp",
//!     ],
//!     tiffs=[
//!         "image/tiff",
//!         "image/webp",
//!     ],
//!     webps=[
//!         "image/jpeg",
//!         "image/webp",
//!     ],
//!     zone_id="0da42c8d2132a9ddaf714f9e7c920711")
//! ```
//! ### C#
//! ```csharp
//! using System.Collections.Generic;
//! using System.Linq;
//! using Pulumi;
//! using Cloudflare = Pulumi.Cloudflare;
//!
//! return await Deployment.RunAsync(() =>
//! {
//!     var example = new Cloudflare.ZoneCacheVariants("example", new()
//!     {
//!         Avifs = new[]
//!         {
//!             "image/avif",
//!             "image/webp",
//!         },
//!         Bmps = new[]
//!         {
//!             "image/bmp",
//!             "image/webp",
//!         },
//!         Gifs = new[]
//!         {
//!             "image/gif",
//!             "image/webp",
//!         },
//!         Jp2s = new[]
//!         {
//!             "image/jp2",
//!             "image/webp",
//!         },
//!         Jpegs = new[]
//!         {
//!             "image/jpeg",
//!             "image/webp",
//!         },
//!         Jpgs = new[]
//!         {
//!             "image/jpg",
//!             "image/webp",
//!         },
//!         Jpg2s = new[]
//!         {
//!             "image/jpg2",
//!             "image/webp",
//!         },
//!         Pngs = new[]
//!         {
//!             "image/png",
//!             "image/webp",
//!         },
//!         Tifs = new[]
//!         {
//!             "image/tif",
//!             "image/webp",
//!         },
//!         Tiffs = new[]
//!         {
//!             "image/tiff",
//!             "image/webp",
//!         },
//!         Webps = new[]
//!         {
//!             "image/jpeg",
//!             "image/webp",
//!         },
//!         ZoneId = "0da42c8d2132a9ddaf714f9e7c920711",
//!     });
//!
//! });
//! ```
//! ### Go
//! ```go
//! package main
//!
//! import (
//! 	"github.com/pulumi/pulumi-cloudflare/sdk/v5/go/cloudflare"
//! 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
//! )
//!
//! func main() {
//! 	pulumi.Run(func(ctx *pulumi.Context) error {
//! 		_, err := cloudflare.NewZoneCacheVariants(ctx, "example", &cloudflare.ZoneCacheVariantsArgs{
//! 			Avifs: pulumi.StringArray{
//! 				pulumi.String("image/avif"),
//! 				pulumi.String("image/webp"),
//! 			},
//! 			Bmps: pulumi.StringArray{
//! 				pulumi.String("image/bmp"),
//! 				pulumi.String("image/webp"),
//! 			},
//! 			Gifs: pulumi.StringArray{
//! 				pulumi.String("image/gif"),
//! 				pulumi.String("image/webp"),
//! 			},
//! 			Jp2s: pulumi.StringArray{
//! 				pulumi.String("image/jp2"),
//! 				pulumi.String("image/webp"),
//! 			},
//! 			Jpegs: pulumi.StringArray{
//! 				pulumi.String("image/jpeg"),
//! 				pulumi.String("image/webp"),
//! 			},
//! 			Jpgs: pulumi.StringArray{
//! 				pulumi.String("image/jpg"),
//! 				pulumi.String("image/webp"),
//! 			},
//! 			Jpg2s: pulumi.StringArray{
//! 				pulumi.String("image/jpg2"),
//! 				pulumi.String("image/webp"),
//! 			},
//! 			Pngs: pulumi.StringArray{
//! 				pulumi.String("image/png"),
//! 				pulumi.String("image/webp"),
//! 			},
//! 			Tifs: pulumi.StringArray{
//! 				pulumi.String("image/tif"),
//! 				pulumi.String("image/webp"),
//! 			},
//! 			Tiffs: pulumi.StringArray{
//! 				pulumi.String("image/tiff"),
//! 				pulumi.String("image/webp"),
//! 			},
//! 			Webps: pulumi.StringArray{
//! 				pulumi.String("image/jpeg"),
//! 				pulumi.String("image/webp"),
//! 			},
//! 			ZoneId: pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		return nil
//! 	})
//! }
//! ```
//! ### Java
//! ```java
//! package generated_program;
//!
//! import com.pulumi.Context;
//! import com.pulumi.Pulumi;
//! import com.pulumi.core.Output;
//! import com.pulumi.cloudflare.ZoneCacheVariants;
//! import com.pulumi.cloudflare.ZoneCacheVariantsArgs;
//! import java.util.List;
//! import java.util.ArrayList;
//! import java.util.Map;
//! import java.io.File;
//! import java.nio.file.Files;
//! import java.nio.file.Paths;
//!
//! public class App {
//!     public static void main(String[] args) {
//!         Pulumi.run(App::stack);
//!     }
//!
//!     public static void stack(Context ctx) {
//!         var example = new ZoneCacheVariants("example", ZoneCacheVariantsArgs.builder()        
//!             .avifs(            
//!                 "image/avif",
//!                 "image/webp")
//!             .bmps(            
//!                 "image/bmp",
//!                 "image/webp")
//!             .gifs(            
//!                 "image/gif",
//!                 "image/webp")
//!             .jp2s(            
//!                 "image/jp2",
//!                 "image/webp")
//!             .jpegs(            
//!                 "image/jpeg",
//!                 "image/webp")
//!             .jpgs(            
//!                 "image/jpg",
//!                 "image/webp")
//!             .jpg2s(            
//!                 "image/jpg2",
//!                 "image/webp")
//!             .pngs(            
//!                 "image/png",
//!                 "image/webp")
//!             .tifs(            
//!                 "image/tif",
//!                 "image/webp")
//!             .tiffs(            
//!                 "image/tiff",
//!                 "image/webp")
//!             .webps(            
//!                 "image/jpeg",
//!                 "image/webp")
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build());
//!
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:ZoneCacheVariants
//!     properties:
//!       avifs:
//!         - image/avif
//!         - image/webp
//!       bmps:
//!         - image/bmp
//!         - image/webp
//!       gifs:
//!         - image/gif
//!         - image/webp
//!       jp2s:
//!         - image/jp2
//!         - image/webp
//!       jpegs:
//!         - image/jpeg
//!         - image/webp
//!       jpgs:
//!         - image/jpg
//!         - image/webp
//!       jpg2s:
//!         - image/jpg2
//!         - image/webp
//!       pngs:
//!         - image/png
//!         - image/webp
//!       tifs:
//!         - image/tif
//!         - image/webp
//!       tiffs:
//!         - image/tiff
//!         - image/webp
//!       webps:
//!         - image/jpeg
//!         - image/webp
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//! ```
//! <!--End PulumiCodeChooser -->

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
    let result = crate::bindings::pulumi::cloudflare::zone_cache_variants::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::zone_cache_variants::Args {
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
        },
    );

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
