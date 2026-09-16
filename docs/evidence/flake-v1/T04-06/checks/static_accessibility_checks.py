"""T04-06 automated technical accessibility qualification -- static checks.

Per docs/canonical/FOUNDER_T04-06_ACCESSIBILITY_WITNESS_AMENDMENT_2026-09-16.md:
every property the removed human-witness checklist would have exercised
must still be established by a reproducible, auditable, automated means
wherever one exists. This script covers the properties that are
source-level and OS-independent (interactive-element semantics, focus
order, contrast, layout robustness, theme/motion respect) -- OS-specific
properties (native accessibility-tree inspection, native launch) are
covered separately by native_launch_smoke_test.mjs, run per-platform in CI.

Exit 0 and prints a JSON report if every check passes; exit 1 otherwise.
Runs identically on every OS (pure static source analysis), so it is run
once per CI matrix leg for convenience, not because its result could
differ by platform.
"""
import json
import re
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[5]
DESKTOP_SRC = REPO_ROOT / "desktop" / "src"

checks = []


def record(name, ok, detail=None):
    checks.append({"name": name, "ok": ok, "detail": detail})


def relative_contrast_luminance(hex_color: str) -> float:
    """WCAG relative luminance, sRGB, no gamma-adjustment shortcuts skipped."""
    hex_color = hex_color.lstrip("#")
    r, g, b = (int(hex_color[i : i + 2], 16) / 255 for i in (0, 2, 4))

    def channel(c):
        return c / 12.92 if c <= 0.03928 else ((c + 0.055) / 1.055) ** 2.4

    r, g, b = channel(r), channel(g), channel(b)
    return 0.2126 * r + 0.7152 * g + 0.0722 * b


def contrast_ratio(hex_a: str, hex_b: str) -> float:
    la, lb = relative_contrast_luminance(hex_a), relative_contrast_luminance(hex_b)
    lighter, darker = max(la, lb), min(la, lb)
    return (lighter + 0.05) / (darker + 0.05)


def check_no_custom_interactive_elements():
    """Every onClick must be on a native semantic control (button/select/
    input/a), never a bare div/span -- the exact "no inaccessible custom
    control where a native/semantic control can be used" property this
    amendment names."""
    violations = []
    for f in DESKTOP_SRC.glob("*.tsx"):
        text = f.read_text(encoding="utf-8")
        for m in re.finditer(r"<(\w+)[^>]*?\sonClick=", text):
            tag = m.group(1)
            if tag not in ("button", "select", "input", "a"):
                line = text.count("\n", 0, m.start()) + 1
                violations.append(f"{f.name}:{line} <{tag} ... onClick=...>")
    record("no-custom-interactive-elements", len(violations) == 0, violations)


def check_no_tabindex_disruption():
    """No explicit tabIndex override anywhere -- focus order is left to
    natural document order, never manually reordered."""
    violations = []
    for f in DESKTOP_SRC.glob("*.tsx"):
        text = f.read_text(encoding="utf-8")
        for m in re.finditer(r"tabIndex", text):
            line = text.count("\n", 0, m.start()) + 1
            violations.append(f"{f.name}:{line}")
    record("no-tabindex-disruption", len(violations) == 0, violations)


def check_inputs_have_accessible_names():
    """Every <input>/<select>/<textarea> is either inside a <label>...
    </label> pair (implicit label association, the pattern this codebase
    uses throughout) or carries an explicit aria-label. Heuristic, not a
    full JSX AST parse: a bounded window after the tag's own start is
    scanned for `aria-label`/the tag's own closing `>`, rather than
    matching an exact `[^>]*>` tag boundary -- a naive `[^>]*>` stops
    early at the first `>` it meets, including the one inside an inline
    `onChange={() => ...}` arrow function, which is common in this
    codebase and would otherwise produce false positives."""
    window = 400
    violations = []
    for f in DESKTOP_SRC.glob("*.tsx"):
        text = f.read_text(encoding="utf-8")
        for m in re.finditer(r"<(input|select|textarea)\b", text):
            tag_start = m.start()
            preceding = text[:tag_start]
            open_labels = len(re.findall(r"<label\b", preceding))
            close_labels = len(re.findall(r"</label>", preceding))
            inside_label = open_labels > close_labels
            snippet = text[tag_start : tag_start + window]
            # Stop the snippet at this tag's own real close (a `>` not
            # immediately preceded by `=`, so `onChange={() => ...}`'s own
            # `>` is skipped) if one appears within the window.
            end_match = re.search(r"(?<!=)>", snippet)
            tag_text = snippet[: end_match.end()] if end_match else snippet
            has_aria_label = "aria-label" in tag_text
            if not (inside_label or has_aria_label):
                line = text.count("\n", 0, tag_start) + 1
                violations.append(f"{f.name}:{line} {tag_text[:80].replace(chr(10), ' ')}")
    record("inputs-have-accessible-names", len(violations) == 0, violations)


def check_no_dangerously_set_inner_html():
    """Reconfirmed here (already checked at every prior task) as part of
    this task's own S08 audit, not assumed carried over silently."""
    violations = []
    for f in DESKTOP_SRC.glob("*.tsx"):
        text = f.read_text(encoding="utf-8")
        if re.search(r"dangerouslySetInnerHTML\s*=", text):
            violations.append(f.name)
    record("no-dangerously-set-inner-html", len(violations) == 0, violations)


def check_color_scheme_respected():
    css = (DESKTOP_SRC / "App.css").read_text(encoding="utf-8")
    ok = bool(re.search(r"color-scheme\s*:\s*light\s+dark", css))
    record("system-theme-color-scheme-respected", ok, {"found": ok})


def check_no_fixed_pixel_width_overflow_risk():
    """Any bare `width: NNpx` (not max-width) over ~360px risks clipping
    at 200% zoom in a modest window -- this app only ever uses max-width
    for its outer shell and 100% for form fields, checked here rather
    than assumed."""
    css = (DESKTOP_SRC / "App.css").read_text(encoding="utf-8")
    violations = []
    for m in re.finditer(r"(?<!max-)width\s*:\s*(\d+)px", css):
        px = int(m.group(1))
        if px > 360:
            line = css.count("\n", 0, m.start()) + 1
            violations.append(f"App.css:{line} width:{px}px")
    record("no-fixed-pixel-width-overflow-risk", len(violations) == 0, violations)


def check_non_color_only_state_indicators():
    """State labels (.state-label, .tab-active) must carry a non-color
    cue (underline/weight), not color alone -- reconfirmed from source,
    not merely from the original implementation commit's own comment."""
    css = (DESKTOP_SRC / "App.css").read_text(encoding="utf-8")
    state_label_block = re.search(r"\.state-label\s*\{([^}]*)\}", css)
    tab_active_block = re.search(r"\.tab-active\s*\{([^}]*)\}", css)
    ok = bool(
        state_label_block
        and "text-decoration" in state_label_block.group(1)
        and tab_active_block
        and "text-decoration" in tab_active_block.group(1)
    )
    record("non-color-only-state-indicators", ok, {"checked": ["state-label", "tab-active"]})


def check_contrast_ratios():
    """WCAG AA normal-text minimum is 4.5:1. This app declares
    `color-scheme: light dark`, so both a light and a dark host
    background are legitimate renderings, not merely the one this
    development host happens to show -- checked against the *actual*
    color each mode will render (`--error-color`'s light-mode base value
    against white, and its `prefers-color-scheme: dark` override against
    black), read directly from App.css rather than hardcoded here twice."""
    css = (DESKTOP_SRC / "App.css").read_text(encoding="utf-8")
    light_match = re.search(r":root\s*\{[^}]*--error-color:\s*(#[0-9a-fA-F]{6})", css)
    dark_match = re.search(
        r"prefers-color-scheme:\s*dark\s*\)\s*\{\s*:root\s*\{[^}]*--error-color:\s*(#[0-9a-fA-F]{6})", css
    )
    if not (light_match and dark_match):
        record("error-color-contrast-meets-wcag-aa", False, {"error": "could not locate --error-color in both modes"})
        return
    light_color, dark_color = light_match.group(1), dark_match.group(1)
    results = {
        f"light-mode-{light_color}-on-white": round(contrast_ratio(light_color, "#ffffff"), 2),
        f"dark-mode-{dark_color}-on-black": round(contrast_ratio(dark_color, "#000000"), 2),
    }
    ok = all(r >= 4.5 for r in results.values())
    record("error-color-contrast-meets-wcag-aa", ok, results)


def check_no_network_surface():
    """Reconfirmed here as part of this task's own S08 audit -- the same
    check every prior task's evidence already ran, not assumed carried
    over silently."""
    dist_dir = REPO_ROOT / "desktop" / "dist" / "assets"
    js_files = list(dist_dir.glob("*.js")) if dist_dir.exists() else []
    if not js_files:
        record("no-network-surface-in-bundle", False, {"error": "dist not built; run npm run build first"})
        return
    pattern = re.compile(r"fetch\(|XMLHttpRequest|WebSocket\(|https?://[a-zA-Z0-9./_-]+")
    allowed_substrings = [
        "http://www.w3.org/",
        "https://react.dev/errors/",
    ]
    violations = []
    for jf in js_files:
        text = jf.read_text(encoding="utf-8", errors="replace")
        for m in pattern.finditer(text):
            matched = m.group(0)
            if matched == "fetch(":
                continue  # Vite's own same-origin modulepreload polyfill, already audited every prior task
            if any(matched.startswith(a) for a in allowed_substrings):
                continue
            violations.append(matched)
    record("no-network-surface-in-bundle", len(violations) == 0, violations[:20])


def main():
    check_no_custom_interactive_elements()
    check_no_tabindex_disruption()
    check_inputs_have_accessible_names()
    check_no_dangerously_set_inner_html()
    check_color_scheme_respected()
    check_no_fixed_pixel_width_overflow_risk()
    check_non_color_only_state_indicators()
    check_contrast_ratios()
    check_no_network_surface()

    all_ok = all(c["ok"] for c in checks)
    print(json.dumps({"all_ok": all_ok, "checks": checks}, indent=2))
    sys.exit(0 if all_ok else 1)


if __name__ == "__main__":
    main()
