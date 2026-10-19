use std::{env, error::Error, io, path::PathBuf};

use chromiumoxide::{
    browser::{Browser, BrowserConfig},
    handler::viewport::Viewport,
    page::ScreenshotParams,
};
use futures_util::StreamExt;
use qrcode::{QrCode, render::svg};
use serde::Serialize;
use tera::{Context, Tera};

use crate::profile::Profile;

const HTML_TEMPLATE: &str = include_str!("../templates/profile.html");
const CSS: &str = include_str!("../templates/profile.css");

#[derive(Serialize)]
struct RenderedProfile<'a> {
    name: &'a str,
    title: Option<&'a str>,
    bio: Option<&'a str>,
    links: Vec<RenderedLink<'a>>,
}

#[derive(Serialize)]
struct RenderedLink<'a> {
    label: &'a str,
    url: &'a str,
    qr_svg: String,
}

pub fn render_html(profile: &Profile) -> Result<String, tera::Error> {
    let mut tera = Tera::default();
    tera.add_raw_template("profile.html", HTML_TEMPLATE)?;

    let links = profile
        .links
        .iter()
        .map(|link| {
            let qr_svg = QrCode::new(link.url.as_bytes())
                .map_err(|error| {
                    tera::Error::message(format!(
                        "could not generate QR code for link {:?}: {error}",
                        link.label
                    ))
                })?
                .render::<svg::Color>()
                .min_dimensions(128, 128)
                .build();
            Ok(RenderedLink {
                label: &link.label,
                url: &link.url,
                qr_svg,
            })
        })
        .collect::<Result<Vec<_>, tera::Error>>()?;
    let rendered_profile = RenderedProfile {
        name: &profile.name,
        title: profile.title.as_deref(),
        bio: profile.bio.as_deref(),
        links,
    };

    let mut context = Context::new();
    context.insert("profile", &rendered_profile);
    context.insert("css", CSS);
    tera.render("profile.html", &context)
}

pub async fn render_png(html: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut config = BrowserConfig::builder();
    if let Some(path) = env::var_os("CHROMIUM_PATH") {
        config = config.chrome_executable(PathBuf::from(path));
    }
    let (mut browser, mut handler) = Browser::launch(
        config
            .window_size(1200, 630)
            .viewport(Viewport {
                width: 1200,
                height: 630,
                device_scale_factor: Some(1.0),
                emulating_mobile: false,
                is_landscape: true,
                has_touch: false,
            })
            .build()?,
    )
    .await?;

    let handler_task = tokio::spawn(async move {
        while let Some(event) = handler.next().await {
            event?;
        }
        Ok::<(), chromiumoxide::error::CdpError>(())
    });

    let render_result = async {
        let page = browser.new_page("about:blank").await?;
        page.set_content(html).await?;
        let png = page.screenshot(ScreenshotParams::default()).await?;
        page.close().await?;
        Ok::<_, chromiumoxide::error::CdpError>(png)
    }
    .await;
    let close_result = browser.close().await;
    let handler_result = handler_task.await;

    let mut cleanup_errors = Vec::new();
    if let Err(error) = close_result {
        cleanup_errors.push(format!("could not close Chromium: {error}"));
    }
    match handler_result {
        Ok(Ok(())) => {}
        Ok(Err(error)) => cleanup_errors.push(format!("Chromium event handler failed: {error}")),
        Err(error) => cleanup_errors.push(format!("Chromium event handler task failed: {error}")),
    }

    match render_result {
        Ok(png) if cleanup_errors.is_empty() => Ok(png),
        Ok(_) => Err(io::Error::other(cleanup_errors.join("; ")).into()),
        Err(error) if cleanup_errors.is_empty() => Err(error.into()),
        Err(error) => {
            Err(io::Error::other(format!("{error}; {}", cleanup_errors.join("; "))).into())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::profile::Profile;

    use super::render_html;

    #[test]
    fn renders_profile_fields_and_escapes_html() {
        let profile = Profile {
            name: "<script>alert(1)</script>".to_owned(),
            title: Some("Engineer".to_owned()),
            bio: Some("Builds useful software".to_owned()),
            links: vec![],
        };

        let html = render_html(&profile).unwrap();

        assert!(html.contains("&lt;script&gt;alert(1)&lt;/script&gt;"));
        assert!(html.contains("Engineer"));
        assert!(html.contains("Builds useful software"));
        assert!(!html.contains("<script>alert(1)</script>"));
    }

    #[test]
    fn renders_link_url_and_qr_code() {
        let profile = Profile {
            name: "Example".to_owned(),
            title: None,
            bio: None,
            links: vec![
                crate::profile::ProfileLink {
                    label: "Project".to_owned(),
                    url: "https://example.com/project".to_owned(),
                },
                crate::profile::ProfileLink {
                    label: "Source".to_owned(),
                    url: "https://example.com/source".to_owned(),
                },
            ],
        };

        let html = render_html(&profile).unwrap();

        assert!(html.contains("https://example.com/project"));
        assert!(html.contains("aria-label=\"QR code for Project\""));
        assert!(html.contains("https://example.com/source"));
        assert!(html.contains("aria-label=\"QR code for Source\""));
        assert_eq!(html.matches("<svg").count(), 2);
        assert!(html.contains("<path"));
    }
}
