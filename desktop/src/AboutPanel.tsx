import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { AboutInfo } from "./types";

// T05-04: "About/help/distribution include license, source, privacy and
// support/reporting route." `about_info` returns the exact same compile-time
// facts the CLI's own `license` command prints (`fehrest::about`) -- this
// panel never fetches anything over the network, and links render as plain
// text (this shell renders no clickable external link anywhere -- consistent
// with the note editor's own Markdown-preview boundary), never a loaded/clicked
// external navigation.
export function AboutPanel({ onClose }: { onClose: () => void }) {
  const [info, setInfo] = useState<AboutInfo | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    invoke<AboutInfo>("about_info")
      .then(setInfo)
      .catch((e) => setError(String(e)));
  }, []);

  return (
    <main className="shell">
      <h1>About Pluma</h1>
      <button onClick={onClose}>&larr; Back</button>
      {error && <p className="error">{error}</p>}
      {info && (
        <section>
          <p>Version: {info.version}</p>
          <p>
            License: {info.license_spdx} (see <code>{info.license_file}</code>)
          </p>
          <p>
            Third-party notices: <code>{info.notice_file}</code>, full list in{" "}
            <code>{info.third_party_licenses_file}</code>
          </p>
          <p>Source: {info.source_url}</p>
          <p>Report a problem / get support: {info.support_url}</p>
          <p>Privacy: {info.privacy_statement}</p>
        </section>
      )}
    </main>
  );
}
