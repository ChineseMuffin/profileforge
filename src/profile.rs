use std::{error::Error, fs, io, path::Path};

use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Profile {
    pub name: String,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub bio: Option<String>,
    #[serde(default)]
    pub links: Vec<ProfileLink>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProfileLink {
    pub label: String,
    pub url: String,
}

pub fn load(path: &Path) -> Result<Profile, Box<dyn Error>> {
    let yaml = fs::read_to_string(path).map_err(|error| {
        io::Error::new(
            error.kind(),
            format!("could not read profile {}: {error}", path.display()),
        )
    })?;
    let profile: Profile = serde_yaml_ng::from_str(&yaml).map_err(|error| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("invalid profile {}: {error}", path.display()),
        )
    })?;
    profile.validate()?;
    Ok(profile)
}

impl Profile {
    fn validate(&self) -> Result<(), io::Error> {
        if self.name.trim().is_empty() {
            return Err(invalid_profile("name must not be empty"));
        }

        for (index, link) in self.links.iter().enumerate() {
            if link.label.trim().is_empty() {
                return Err(invalid_profile(format!(
                    "links[{index}].label must not be empty"
                )));
            }

            let url = Url::parse(&link.url).map_err(|error| {
                invalid_profile(format!("links[{index}].url is invalid: {error}"))
            })?;
            if !matches!(url.scheme(), "http" | "https") || url.host().is_none() {
                return Err(invalid_profile(format!(
                    "links[{index}].url must be an absolute HTTP or HTTPS URL"
                )));
            }
        }

        Ok(())
    }
}

fn invalid_profile(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn profile_with_link(url: &str) -> Profile {
        Profile {
            name: "Example".to_owned(),
            title: None,
            bio: None,
            links: vec![ProfileLink {
                label: "Website".to_owned(),
                url: url.to_owned(),
            }],
        }
    }

    #[test]
    fn accepts_http_and_https_links() {
        profile_with_link("https://example.com").validate().unwrap();
        profile_with_link("http://example.com").validate().unwrap();
    }

    #[test]
    fn rejects_blank_names() {
        let mut profile = profile_with_link("https://example.com");
        profile.name = "  ".to_owned();

        assert!(profile.validate().is_err());
    }

    #[test]
    fn rejects_non_http_links() {
        assert!(profile_with_link("javascript:alert(1)").validate().is_err());
    }

    #[test]
    fn rejects_blank_link_labels() {
        let mut profile = profile_with_link("https://example.com");
        profile.links[0].label = "  ".to_owned();

        assert!(profile.validate().is_err());
    }

    #[test]
    fn sample_profile_is_valid() {
        let profile: Profile =
            serde_yaml_ng::from_str(include_str!("../examples/sample.yaml")).unwrap();

        profile.validate().unwrap();
    }
}
