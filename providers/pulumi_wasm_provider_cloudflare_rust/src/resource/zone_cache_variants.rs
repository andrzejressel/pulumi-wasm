pub struct ZoneCacheVariantsArgs {
    pub avifs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    pub bmps: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    pub gifs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    pub jp2s: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    pub jpegs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    pub jpg2s: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    pub jpgs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    pub pngs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    pub tiffs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    pub tifs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    pub webps: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct ZoneCacheVariantsResult {
    pub avifs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    pub bmps: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    pub gifs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    pub jp2s: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    pub jpegs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    pub jpg2s: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    pub jpgs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    pub pngs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    pub tiffs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    pub tifs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    pub webps: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: ZoneCacheVariantsArgs) -> ZoneCacheVariantsResult {
    let result = crate::bindings::pulumi::cloudflare::zone_cache_variants::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::zone_cache_variants::Args {
            avifs: args.avifs.get_inner(),
            bmps: args.bmps.get_inner(),
            gifs: args.gifs.get_inner(),
            jp2s: args.jp2s.get_inner(),
            jpegs: args.jpegs.get_inner(),
            jpg2s: args.jpg2s.get_inner(),
            jpgs: args.jpgs.get_inner(),
            pngs: args.pngs.get_inner(),
            tiffs: args.tiffs.get_inner(),
            tifs: args.tifs.get_inner(),
            webps: args.webps.get_inner(),
            zone_id: args.zone_id.get_inner(),
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
