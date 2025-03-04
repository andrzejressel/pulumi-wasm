use serde::{Deserialize, Serialize};
use wasmparser::{Parser, Payload};

#[derive(PartialEq, Debug, Serialize)]
pub(crate) struct ProviderVersion {
    pub name: String,
    pub version: String,
    pub resource: bool,
    pub plugin_download_url: Option<String>,
}

#[derive(Deserialize)]
struct WasmProviderVersion {
    pub version: String,
    #[serde(rename = "pluginDownloadURL")]
    pub plugin_download_url: Option<String>,
}

pub(crate) fn extract_custom_section(data: &[u8]) -> Vec<ProviderVersion> {
    let mut providers = Vec::new();

    let parser = Parser::new(0);

    for payload in parser.parse_all(data) {
        if let Payload::CustomSection(reader) = payload.unwrap() {
            let name = reader.name();
            if let Some(name) = name.strip_prefix("pulumi_gestalt_provider::") {
                let version: WasmProviderVersion = serde_json::from_slice(reader.data()).unwrap();
                providers.push(ProviderVersion {
                    name: name.to_string(),
                    version: version.version,
                    plugin_download_url: version.plugin_download_url,
                    resource: true,
                });
            }
        }
    }

    providers
}

#[cfg(test)]
mod tests {
    use crate::version_finder::{extract_custom_section, ProviderVersion};

    static WAT: &str = include_str!("mocks/wasm.wat");

    #[test]
    fn test_wat() {
        let wasm = wat::parse_str(WAT).unwrap();
        let custom = extract_custom_section(&wasm);
        assert_eq!(
            custom,
            vec![
                ProviderVersion {
                    name: "random".to_string(),
                    version: "4.15.0".to_string(),
                    plugin_download_url: None,
                    resource: true
                },
                ProviderVersion {
                    name: "docker".to_string(),
                    version: "4.5.3".to_string(),
                    plugin_download_url: Some("https://example.com".to_string()),
                    resource: true
                }
            ]
        );
    }
}
