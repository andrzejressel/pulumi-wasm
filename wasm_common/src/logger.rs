use crate::bindings;
use crate::bindings::component::pulumi_wasm::log::Content;
use bindings::component::pulumi_wasm;
use log::kv::{Source, VisitSource};
use log::{Level, Log, Metadata, Record};
use std::collections::HashMap;

pub(crate) struct Logger {}

impl Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let level = record.metadata().level();
            let target = record.metadata().target();
            let args = record.args().to_string();
            let module_path = record.module_path();
            let file = record.file();
            let line = record.line();
            let key_values = source_to_map(record.key_values());

            let content = Content {
                level: match level {
                    Level::Error => pulumi_wasm::log::Level::Error,
                    Level::Warn => pulumi_wasm::log::Level::Warn,
                    Level::Info => pulumi_wasm::log::Level::Info,
                    Level::Debug => pulumi_wasm::log::Level::Debug,
                    Level::Trace => pulumi_wasm::log::Level::Trace,
                },
                target: target.to_string(),
                args,
                module_path: module_path.map(|s| s.to_string()),
                file: file.map(|s| s.to_string()),
                line,
                key_values: key_values.into_iter().collect(),
            };

            pulumi_wasm::log::log(&content);
        }
    }

    fn flush(&self) {}
}

struct HashMapPrinter {
    map: HashMap<String, String>,
}

impl HashMapPrinter {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}

impl<'kvs> VisitSource<'kvs> for HashMapPrinter {
    fn visit_pair(
        &mut self,
        key: log::kv::Key<'kvs>,
        value: log::kv::Value<'kvs>,
    ) -> Result<(), log::kv::Error> {
        self.map.insert(key.to_string(), value.to_string());
        Ok(())
    }
}

fn source_to_map(s: &dyn Source) -> HashMap<String, String> {
    let mut printer = HashMapPrinter::new();
    s.visit(&mut printer).unwrap();
    printer.map
}
