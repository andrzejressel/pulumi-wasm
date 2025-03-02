pub(crate) trait Encoder {
    fn encode_collapsible_block_start(&self, title: &str) -> String;
    fn encode_collapsible_block_element(&self, line: &str) -> String;
    fn encode_collapsible_block_end(&self) -> String;
}

pub(crate) struct GithubFlavorEncoder {}

impl Encoder for GithubFlavorEncoder {
    fn encode_collapsible_block_start(&self, title: &str) -> String {
        let mut s = String::new();
        s.push_str("<details>\n");
        s.push_str(&format!("<summary><h3>{}</h3></summary>\n", title));
        s.push('\n');
        s
    }

    fn encode_collapsible_block_element(&self, line: &str) -> String {
        format!("{}\n", line)
    }

    fn encode_collapsible_block_end(&self) -> String {
        "</details>\n\n".to_string()
    }
}

pub(crate) struct MkdocsEncoder {}

impl Encoder for MkdocsEncoder {
    fn encode_collapsible_block_start(&self, title: &str) -> String {
        format!("??? \"{}\"\n", title)
    }

    fn encode_collapsible_block_element(&self, line: &str) -> String {
        format!("    {}\n", line)
    }

    fn encode_collapsible_block_end(&self) -> String {
        "\n\n".to_string()
    }
}
