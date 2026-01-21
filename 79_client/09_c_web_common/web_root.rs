use super::*;

#[derive(Clone)]
pub struct Web_Root {
    pub protocol: Arc<String>,
    pub host: Arc<String>,
    pub is_dev: bool,
    pub wasm_version: Arc<String>,
}

impl Web_Root {
    pub fn make_pub_url(&self, part: &str) -> String {
        if self.host.as_str().ends_with("discordsays.com") {
            format!("{}//{}/.proxy/pub/{}", self.protocol.as_str(), self.host.as_str(), part)
        } else {
            format!("{}//{}/pub/{}", self.protocol.as_str(), self.host.as_str(), part)
        }
    }

    pub fn make_priv_url(&self, part: &str) -> String {
        format!("{}//{}/priv/{}", self.protocol.as_str(), self.host.as_str(), part)
    }

    pub fn new(protocol: Arc<String>, host: Arc<String>, is_dev: bool, wasm_version: Arc<String>) -> Web_Root {
        Web_Root {
            protocol: protocol.clone(),
            host: host.clone(),
            is_dev,
            wasm_version,
        }
    }
}
