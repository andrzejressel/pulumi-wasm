use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::zone_cache_variants;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl zone_cache_variants::Guest for Component {
    fn invoke(
        name: String,
        args: zone_cache_variants::Args
    ) -> zone_cache_variants::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/zoneCacheVariants:ZoneCacheVariants".into(),
            name,
            object: vec![
                ObjectField { name: "avifs".into(), value: args.avifs },
                ObjectField { name: "bmps".into(), value: args.bmps },
                ObjectField { name: "gifs".into(), value: args.gifs },
                ObjectField { name: "jp2s".into(), value: args.jp2s },
                ObjectField { name: "jpegs".into(), value: args.jpegs },
                ObjectField { name: "jpg2s".into(), value: args.jpg2s },
                ObjectField { name: "jpgs".into(), value: args.jpgs },
                ObjectField { name: "pngs".into(), value: args.pngs },
                ObjectField { name: "tiffs".into(), value: args.tiffs },
                ObjectField { name: "tifs".into(), value: args.tifs },
                ObjectField { name: "webps".into(), value: args.webps },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "avifs".into() },
                ResultField { name: "bmps".into() },
                ResultField { name: "gifs".into() },
                ResultField { name: "jp2s".into() },
                ResultField { name: "jpegs".into() },
                ResultField { name: "jpg2s".into() },
                ResultField { name: "jpgs".into() },
                ResultField { name: "pngs".into() },
                ResultField { name: "tiffs".into() },
                ResultField { name: "tifs".into() },
                ResultField { name: "webps".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        zone_cache_variants::Res {
            avifs: hashmap.remove("avifs").unwrap(),
            bmps: hashmap.remove("bmps").unwrap(),
            gifs: hashmap.remove("gifs").unwrap(),
            jp2s: hashmap.remove("jp2s").unwrap(),
            jpegs: hashmap.remove("jpegs").unwrap(),
            jpg2s: hashmap.remove("jpg2s").unwrap(),
            jpgs: hashmap.remove("jpgs").unwrap(),
            pngs: hashmap.remove("pngs").unwrap(),
            tiffs: hashmap.remove("tiffs").unwrap(),
            tifs: hashmap.remove("tifs").unwrap(),
            webps: hashmap.remove("webps").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
