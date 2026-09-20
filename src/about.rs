//! Product identity, license, source, privacy and support-route facts.
//!
//! Single canonical source for the "About/help/distribution include license,
//! source, privacy and support/reporting route" requirement (plan section 25/28,
//! task `T05-04`). Both the CLI's `license` command and the desktop app's
//! `about_info` bridge command read these same constants, so the two surfaces
//! can never state this diverging text. Every value is a compile-time constant;
//! nothing here performs a network request.

use serde::Serialize;

/// The product's own compiled version (root crate `Cargo.toml`).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// SPDX identifier of the project's own license.
pub const LICENSE_SPDX: &str = "Apache-2.0";

/// Where the full project license text lives in a checked-out or distributed copy.
pub const LICENSE_FILE: &str = "LICENSE";

/// Where the project's own copyright/attribution notice lives.
pub const NOTICE_FILE: &str = "NOTICE";

/// Where every shipped third-party component's exact license is recorded.
pub const THIRD_PARTY_LICENSES_FILE: &str = "docs/legal/THIRD-PARTY-LICENSES.md";

/// Public source repository (also the release/download/reporting home).
pub const SOURCE_URL: &str = "https://github.com/TheHalfMoon/Pluma";

/// Where to report a bug or ask for support.
pub const SUPPORT_URL: &str = "https://github.com/TheHalfMoon/Pluma/issues";

/// The privacy statement this product actually implements, not aspirational
/// copy: local-first, offline, account-free, no telemetry, no cloud sync.
/// Data leaves this device only when the owner explicitly exports or
/// discloses it (`export-run`/`package-export`).
pub const PRIVACY_STATEMENT: &str = "Pluma is local-first, offline and account-free. \
It does not sign in, sync to a cloud service, or send telemetry. No data leaves this \
device unless the owner explicitly exports or discloses it.";

/// Everything the About/help surface and distributed package need to show,
/// bundled as one struct so the CLI and desktop bridge render identical facts.
#[derive(Debug, Clone, Serialize)]
pub struct AboutInfo {
    pub version: &'static str,
    pub license_spdx: &'static str,
    pub license_file: &'static str,
    pub notice_file: &'static str,
    pub third_party_licenses_file: &'static str,
    pub source_url: &'static str,
    pub support_url: &'static str,
    pub privacy_statement: &'static str,
}

/// Build the current, static About information.
pub fn about_info() -> AboutInfo {
    AboutInfo {
        version: VERSION,
        license_spdx: LICENSE_SPDX,
        license_file: LICENSE_FILE,
        notice_file: NOTICE_FILE,
        third_party_licenses_file: THIRD_PARTY_LICENSES_FILE,
        source_url: SOURCE_URL,
        support_url: SUPPORT_URL,
        privacy_statement: PRIVACY_STATEMENT,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn about_info_carries_every_required_fact() {
        let info = about_info();
        assert!(!info.version.is_empty());
        assert_eq!(info.license_spdx, "Apache-2.0");
        assert_eq!(info.license_file, "LICENSE");
        assert_eq!(info.notice_file, "NOTICE");
        assert_eq!(
            info.third_party_licenses_file,
            "docs/legal/THIRD-PARTY-LICENSES.md"
        );
        assert!(info.source_url.starts_with("https://github.com/"));
        assert!(info.support_url.starts_with("https://github.com/"));
        assert!(!info.privacy_statement.is_empty());
    }
}
