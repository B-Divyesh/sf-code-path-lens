use anyhow::{Context, Result, bail};
use serde::Deserialize;

const PRODUCT_SLUG: &str = "code-path-lens";
/// The released product always verifies against Sociobot's production billing
/// service. Staging and local integration tests can use the explicit
/// `CODE_PATH_LENS_BILLING_BASE` override instead.
const DEFAULT_BILLING_BASE: &str = "https://api.sociobot.in/api/v1";

#[derive(Deserialize)]
struct VerifyResponse {
    valid: bool,
    reason: String,
}

pub fn require_pro(explicit_token: Option<&str>) -> Result<()> {
    let token = explicit_token
        .map(ToOwned::to_owned)
        .or_else(|| std::env::var("CODE_PATH_LENS_LICENSE").ok())
        .filter(|value| !value.trim().is_empty())
        .context(
            "this bound needs Code Path Lens Pro; pass --license or set CODE_PATH_LENS_LICENSE",
        )?;
    let base = std::env::var("CODE_PATH_LENS_BILLING_BASE")
        .unwrap_or_else(|_| DEFAULT_BILLING_BASE.to_string());
    let url = format!(
        "{}/products/{}/verify",
        base.trim_end_matches('/'),
        PRODUCT_SLUG
    );
    let mut response = ureq::get(&url)
        .query("license", &token)
        .call()
        .context("could not verify the Pro license; check the network and try again")?;
    let body = response
        .body_mut()
        .read_to_string()
        .context("could not read the license verification response")?;
    let verdict: VerifyResponse =
        serde_json::from_str(&body).context("license service returned an unexpected response")?;
    if !verdict.valid {
        bail!("license is not active ({})", verdict.reason);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::DEFAULT_BILLING_BASE;

    #[test]
    fn released_cli_defaults_to_the_production_billing_service() {
        assert_eq!(DEFAULT_BILLING_BASE, "https://api.sociobot.in/api/v1");
        assert!(!DEFAULT_BILLING_BASE.contains("pilot-api"));
    }
}
