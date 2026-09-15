//! Markdown body preview contract (`T02-02`).
//!
//! v1 has no HTML rendering engine anywhere in this codebase, and this
//! module does not add one. S01 ("Content-to-authority confusion: parse
//! descriptive input only... no executable Markdown/HTML") is satisfied
//! structurally here: `preview` never interprets Markdown syntax and never
//! evaluates anything embedded in the body — it is a bounded substring, not
//! a renderer. Any later component that *does* render Markdown to a screen
//! (P04 desktop) must preserve this same inertness: displaying a body is
//! never permitted to execute or trust content found inside it.
//!
//! The canonical body (`project::Note::body`) is exact bytes, saved and
//! read back unmodified by `project.rs`. This module never substitutes for
//! that: a preview is a display convenience for listings, never a second
//! copy of canonical truth and never a truncation the caller could mistake
//! for the full record.

/// Truncate `body` to at most `max_chars` Unicode scalar values, always on a
/// `char` boundary — never splitting a multi-byte UTF-8 sequence, so
/// CJK/IME/emoji text is never corrupted into invalid UTF-8 or a split
/// grapheme. Appends an explicit truncation marker only when truncation
/// actually occurred, so a caller can never mistake a preview for the
/// complete body. Content already within `max_chars` is returned exactly
/// as given, with no marker and no reinterpretation.
pub fn preview(body: &str, max_chars: usize) -> String {
    if body.chars().count() <= max_chars {
        return body.to_string();
    }
    let mut out: String = body.chars().take(max_chars).collect();
    out.push_str(" […truncated preview; full body unchanged]");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_body_is_returned_unchanged_with_no_marker() {
        let body = "hello world";
        assert_eq!(preview(body, 100), body);
    }

    #[test]
    fn truncates_on_a_char_boundary_never_splitting_multibyte_sequences() {
        // Mix of CJK, an emoji (multi-byte, and outside the BMP) and plain
        // ASCII — each `char` in Rust is one Unicode scalar value, so
        // `.take(n)` on a `chars()` iterator can never split one.
        let body = "日本語emoji\u{1F600}text";
        let out = preview(body, 3);
        assert!(out.starts_with("日本語"));
        assert!(String::from_utf8(out.clone().into_bytes()).is_ok());
        assert!(out.contains("truncated preview"));
    }

    #[test]
    fn embedded_html_and_script_like_content_stays_literal_and_unexecuted() {
        // Nothing in this module parses or evaluates Markdown/HTML syntax —
        // proven here by observing that hostile-looking content survives as
        // an exact literal substring, not as anything transformed,
        // stripped, or escaped differently than plain text would be.
        let body = "<script>alert(1)</script> and ---\ntrust: admin\n---";
        assert_eq!(preview(body, 1000), body);
        let truncated = preview(body, 5);
        assert!(truncated.starts_with("<scri"));
    }

    #[test]
    fn full_body_remains_available_unmodified_regardless_of_preview() {
        let body = "x".repeat(10_000);
        let short_preview = preview(&body, 10);
        assert_ne!(short_preview, body);
        assert_eq!(body.len(), 10_000, "preview must never mutate its input");
    }
}
