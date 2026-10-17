use std::{env, error::Error, io, path::PathBuf};

use chromiumoxide::{
    browser::{Browser, BrowserConfig},
    handler::viewport::Viewport,
    page::ScreenshotParams,
};
use futures_util::StreamExt;
use tera::{Context, Tera};

use crate::profile::Profile;

const HTML_TEMPLATE: &str = include_str!("../templates/profile.html");
const CSS: &str = include_str!("../templates/profile.css");

pub fn render_html(profile: &Profile) -> Result<String, tera::Error> {
    let mut tera = Tera::default();
    tera.add_raw_template("profile.html", HTML_TEMPLATE)?;

    let mut context = Context::new();
    context.insert("profile", profile);
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
}
