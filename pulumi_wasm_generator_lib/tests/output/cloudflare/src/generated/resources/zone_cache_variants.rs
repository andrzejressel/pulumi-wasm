#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ZoneCacheVariantsArgs {
    /// List of strings with the MIME types of all the variants that should be served for avif.
    #[builder(into, default)]
    pub avifs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// List of strings with the MIME types of all the variants that should be served for bmp.
    #[builder(into, default)]
    pub bmps: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// List of strings with the MIME types of all the variants that should be served for gif.
    #[builder(into, default)]
    pub gifs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// List of strings with the MIME types of all the variants that should be served for jp2.
    #[builder(into, default)]
    pub jp2s: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// List of strings with the MIME types of all the variants that should be served for jpeg.
    #[builder(into, default)]
    pub jpegs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// List of strings with the MIME types of all the variants that should be served for jpg2.
    #[builder(into, default)]
    pub jpg2s: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// List of strings with the MIME types of all the variants that should be served for jpg.
    #[builder(into, default)]
    pub jpgs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// List of strings with the MIME types of all the variants that should be served for png.
    #[builder(into, default)]
    pub pngs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// List of strings with the MIME types of all the variants that should be served for tiff.
    #[builder(into, default)]
    pub tiffs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// List of strings with the MIME types of all the variants that should be served for tif.
    #[builder(into, default)]
    pub tifs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// List of strings with the MIME types of all the variants that should be served for webp.
    #[builder(into, default)]
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
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let avifs_binding = args.avifs.get_inner();
    let bmps_binding = args.bmps.get_inner();
    let gifs_binding = args.gifs.get_inner();
    let jp2s_binding = args.jp2s.get_inner();
    let jpegs_binding = args.jpegs.get_inner();
    let jpg2s_binding = args.jpg2s.get_inner();
    let jpgs_binding = args.jpgs.get_inner();
    let pngs_binding = args.pngs.get_inner();
    let tiffs_binding = args.tiffs.get_inner();
    let tifs_binding = args.tifs.get_inner();
    let webps_binding = args.webps.get_inner();
    let zone_id_binding = args.zone_id.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/zoneCacheVariants:ZoneCacheVariants".into(),
        name: name.to_string(),
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
        results: vec![
            register_interface::ResultField { name : "avifs".into() },
            register_interface::ResultField { name : "bmps".into() },
            register_interface::ResultField { name : "gifs".into() },
            register_interface::ResultField { name : "jp2s".into() },
            register_interface::ResultField { name : "jpegs".into() },
            register_interface::ResultField { name : "jpg2s".into() },
            register_interface::ResultField { name : "jpgs".into() },
            register_interface::ResultField { name : "pngs".into() },
            register_interface::ResultField { name : "tiffs".into() },
            register_interface::ResultField { name : "tifs".into() },
            register_interface::ResultField { name : "webps".into() },
            register_interface::ResultField { name : "zoneId".into() },
        ],
    };
    fn into_domain<F: serde::Serialize>(output: WitOutput) -> Output<F> {
        unsafe { Output::<F>::new_from_handle(output) }
    }
    let o = register_interface::register(&request);
    let mut hashmap: HashMap<String, _> = o
        .fields
        .into_iter()
        .map(|f| (f.name, f.output))
        .collect();
    ZoneCacheVariantsResult {
        avifs: into_domain(hashmap.remove("avifs").unwrap()),
        bmps: into_domain(hashmap.remove("bmps").unwrap()),
        gifs: into_domain(hashmap.remove("gifs").unwrap()),
        jp2s: into_domain(hashmap.remove("jp2s").unwrap()),
        jpegs: into_domain(hashmap.remove("jpegs").unwrap()),
        jpg2s: into_domain(hashmap.remove("jpg2s").unwrap()),
        jpgs: into_domain(hashmap.remove("jpgs").unwrap()),
        pngs: into_domain(hashmap.remove("pngs").unwrap()),
        tiffs: into_domain(hashmap.remove("tiffs").unwrap()),
        tifs: into_domain(hashmap.remove("tifs").unwrap()),
        webps: into_domain(hashmap.remove("webps").unwrap()),
        zone_id: into_domain(hashmap.remove("zoneId").unwrap()),
    }
}
