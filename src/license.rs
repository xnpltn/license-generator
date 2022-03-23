use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct License {
    pub key: String,
    pub name: String,
    pub spdx_id: String,
    pub url: String,
    pub node_id: String,
}

#[derive(Debug, Deserialize)]
pub struct Licenses {
    pub license: Vec<License>
}

impl License {
    pub fn fetch() -> Licenses {
        let response: Vec<License> = match ureq::get("https://api.github.com/licenses").call() {
            Ok(res) => res.into_json().unwrap(),
            Err(error) => panic!("Unable to get licenses: {}", error)
        };

        Licenses {license: response}
    }
}