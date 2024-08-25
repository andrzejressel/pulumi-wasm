use anyhow::*;
use async_trait::async_trait;
use directories::BaseDirs;
use std::fs;
use std::path::PathBuf;

#[async_trait]
pub trait ProviderSource {
    async fn get_component(&self, pulumi_wasm_version: &str) -> Result<Vec<u8>>;
}

#[async_trait]
pub trait DefaultProviderSource {
    async fn get_component(
        &self,
        provider_name: &str,
        provider_version: &str,
        pulumi_wasm_version: &str,
        debug: bool,
    ) -> Result<Vec<u8>>;
}

#[async_trait]
pub trait PulumiWasmSource {
    async fn get(&self, version: &str, debug: bool) -> Result<Vec<u8>>;
}

pub struct GithubPulumiWasmSource;

#[async_trait]
impl PulumiWasmSource for GithubPulumiWasmSource {
    async fn get(&self, version: &str, debug: bool) -> Result<Vec<u8>> {
        let profile = if debug { "debug" } else { "release" };
        let debug_suffix = if debug { "-debug" } else { "" };

        let wasm_location = BaseDirs::new()
            .context("Unable to get user directories")?
            .cache_dir()
            .join("pulumi-wasm")
            .join(format!("pulumi-wasm-{version}-{profile}.wasm"));

        let url = format!(
            "https://github.com/andrzejressel/pulumi-wasm/releases/download/v{version}/pulumi_wasm{debug_suffix}.wasm"
        );

        download_file_and_cache(wasm_location, &url)
            .await
            .context(format!(
            "Cannot download pulumi-wasm in version {version} with profile {profile}. Url: [{url}]"
        ))
    }
}

#[async_trait]
impl DefaultProviderSource for GithubPulumiWasmSource {
    async fn get_component(
        &self,
        provider_name: &str,
        provider_version: &str,
        pulumi_wasm_version: &str,
        debug: bool,
    ) -> Result<Vec<u8>> {
        let profile = if debug { "debug" } else { "release" };
        let debug_suffix = if debug { "-debug" } else { "" };
        let wasm_location = BaseDirs::new()
            .context("Unable to get user directories")?
            .cache_dir()
            .join("pulumi-wasm")
            .join("providers")
            .join(format!(
                "{provider_name}-{provider_version}-{pulumi_wasm_version}-{profile}.wasm",
            ));

        let url = format!("https://github.com/andrzejressel/pulumi-wasm/releases/download/v{pulumi_wasm_version}/pulumi_wasm_{provider_name}_provider{debug_suffix}.wasm");

        download_file_and_cache(wasm_location, &url)
            .await.context(format!("Cannot download provider {provider_name} in version {provider_version} for pulumi wasm {pulumi_wasm_version} with profile {profile}. Url: [{url}]"))
    }
}

async fn download_file_and_cache(cache: PathBuf, url: &String) -> Result<Vec<u8>> {
    if !cache.exists() {
        fs::create_dir_all(cache.parent().unwrap())?;

        let response = reqwest::get(url).await?;

        let status = response.status();

        let bytes = response
            .bytes()
            .await
            .context("Failed to download pulumi_wasm")?;

        if !status.is_success() {
            bail!(
                "Cannot download - message from server: {}",
                String::from_utf8(bytes.to_vec()).unwrap()
            );
        }

        tokio::fs::write(&cache, &bytes)
            .await
            .context("Failed to write pulumi_wasm file")?;
        Ok(bytes.to_vec())
    } else {
        fs::read(&cache).context(format!("Cannot read file: {}", cache.to_str().unwrap()))
    }
}

pub struct FileSource(PathBuf);

impl FileSource {
    pub fn new(path_buf: PathBuf) -> Self {
        Self(path_buf)
    }
}

#[async_trait]
impl DefaultProviderSource for FileSource {
    async fn get_component(
        &self,
        _provider_name: &str,
        _provider_version: &str,
        _pulumi_wasm_version: &str,
        _debug: bool,
    ) -> Result<Vec<u8>> {
        Ok(fs::read(&self.0)?)
    }
}

#[async_trait]
impl ProviderSource for FileSource {
    async fn get_component(&self, _pulumi_wasm_version: &str) -> Result<Vec<u8>> {
        Ok(fs::read(&self.0)?)
    }
}

#[async_trait]
impl PulumiWasmSource for FileSource {
    async fn get(&self, _version: &str, _debug: bool) -> Result<Vec<u8>> {
        Ok(fs::read(&self.0)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod pulumi_wasm_source {
        use super::*;

        #[tokio::test]
        async fn should_download_existing_pulumi_wasm() -> Result<()> {
            let source = GithubPulumiWasmSource {};
            let res = source.get("0.0.0-NIGHTLY-3fee0e3", false).await?;
            assert!(!res.is_empty());
            Ok(())
        }

        #[tokio::test]
        async fn should_download_existing_debug_pulumi_wasm() -> Result<()> {
            let source = GithubPulumiWasmSource {};
            let res = source.get("0.0.0-NIGHTLY-3fee0e3", true).await?;
            assert!(!res.is_empty());
            Ok(())
        }

        #[tokio::test]
        async fn should_fail_on_noexisting_version() -> Result<()> {
            let source = GithubPulumiWasmSource {};
            let err = source
                .get("0.0.0-NIGHTLY-nonexistent", false)
                .await
                .expect_err("Expected error");
            assert_eq!(
                err.to_string(),
                "Cannot download pulumi-wasm in version 0.0.0-NIGHTLY-nonexistent with profile release. Url: [https://github.com/andrzejressel/pulumi-wasm/releases/download/v0.0.0-NIGHTLY-nonexistent/pulumi_wasm.wasm]"
            );
            Ok(())
        }

        #[tokio::test]
        async fn should_download_existing_provider() -> Result<()> {
            let source = GithubPulumiWasmSource {};
            let res = source
                .get_component(
                    "cloudflare",
                    "0.0.0-NIGHTLY-3fee0e3",
                    "0.0.0-NIGHTLY-3fee0e3",
                    false,
                )
                .await?;
            assert!(!res.is_empty());
            Ok(())
        }

        #[tokio::test]
        async fn should_download_existing_debug_provider() -> Result<()> {
            let source = GithubPulumiWasmSource {};
            let res = source
                .get_component(
                    "cloudflare",
                    "0.0.0-NIGHTLY-3fee0e3",
                    "0.0.0-NIGHTLY-3fee0e3",
                    true,
                )
                .await?;
            assert!(!res.is_empty());
            Ok(())
        }

        #[tokio::test]
        async fn should_fail_on_nonexisting_provider() -> Result<()> {
            let source = GithubPulumiWasmSource {};
            let err = source
                .get_component("cloudflare", "0.0.0", "1.1.1", false)
                .await
                .expect_err("Expected error");
            assert_eq!(
                err.to_string(),
                "Cannot download provider cloudflare in version 0.0.0 for pulumi wasm 1.1.1 with profile release. Url: [https://github.com/andrzejressel/pulumi-wasm/releases/download/v1.1.1/pulumi_wasm_cloudflare_provider.wasm]"
            );
            Ok(())
        }
    }
}
