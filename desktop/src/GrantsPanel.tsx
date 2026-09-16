import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { GrantEntry, PackageCompileResult, PackagePreviewResult } from "./types";
import { useConfirm } from "./Confirm";

const DISCLOSABLE_KINDS = ["note", "action", "decision", "relation", "source"];

// §18's own package scope/budget/privacy preview: this panel never writes
// a package to disk (that is `T04-05`'s "expose... export safely" scope,
// which also needs a native destination dialog this task does not add) --
// `package_preview` calls Core's own `preview_disclosure_package`, which
// commits no receipt and requires no write access, exactly for this
// preview purpose.
export function GrantsPanel({ vaultPath, projectId }: { vaultPath: string; projectId: string }) {
  const [grants, setGrants] = useState<GrantEntry[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [selectedKinds, setSelectedKinds] = useState<string[]>(["note"]);
  const [byteBudget, setByteBudget] = useState(65536);
  const [ttlHours, setTtlHours] = useState(24);
  const [previewGrantId, setPreviewGrantId] = useState<string | null>(null);
  const [requestId, setRequestId] = useState("");
  const [principal, setPrincipal] = useState("agent");
  const [preview, setPreview] = useState<PackagePreviewResult | null>(null);
  const [compiled, setCompiled] = useState<PackageCompileResult | null>(null);
  const { requestConfirm, confirmDialog } = useConfirm();

  function toggleKind(kind: string) {
    setSelectedKinds((prev) => (prev.includes(kind) ? prev.filter((k) => k !== kind) : [...prev, kind]));
  }

  async function handleIssue() {
    if (selectedKinds.length === 0) return;
    try {
      const grant = await invoke<GrantEntry>("grant_issue", {
        vaultPath,
        projectId,
        allowedKinds: selectedKinds,
        byteBudget,
        ttlSecs: ttlHours * 3600,
        options: {},
      });
      setGrants((prev) => [...prev, grant]);
    } catch (e) {
      setError(String(e));
    }
  }

  async function handleRevoke(g: GrantEntry) {
    try {
      const updated = await invoke<GrantEntry>("grant_revoke", {
        vaultPath,
        grantId: g.id,
        expectedRevisionId: g.revision_id,
      });
      setGrants((prev) => prev.map((x) => (x.id === updated.id ? updated : x)));
    } catch (e) {
      setError(String(e));
    }
  }

  async function handlePreview() {
    if (!previewGrantId || !requestId.trim()) return;
    setError(null);
    try {
      const result = await invoke<PackagePreviewResult>("package_preview", {
        vaultPath,
        grantId: previewGrantId,
        requestId: requestId.trim(),
        principal: principal.trim() || null,
      });
      setPreview(result);
    } catch (e) {
      setError(String(e));
      setPreview(null);
    }
  }

  async function handleCompile() {
    if (!previewGrantId || !requestId.trim()) return;
    setError(null);
    try {
      const result = await invoke<PackageCompileResult>("package_compile", {
        vaultPath,
        grantId: previewGrantId,
        requestId: requestId.trim(),
        principal: principal.trim() || null,
      });
      setCompiled(result);
    } catch (e) {
      setError(String(e));
      setCompiled(null);
    }
  }

  return (
    <div className="grants-panel">
      {confirmDialog}
      <h3>Export grants &amp; package preview</h3>
      {grants.length === 0 ? (
        <p>No grants issued yet.</p>
      ) : (
        <ul>
          {grants.map((g) => (
            <li key={g.id}>
              <span className="state-label">{g.state === "active" ? "Active" : "Revoked"}</span>{" "}
              kinds=[{g.allowed_kinds.join(", ")}] budget={g.byte_budget}B expires={g.expires_at}{" "}
              {g.state === "active" && (
                <>
                  <button
                    onClick={() =>
                      requestConfirm(`Revoke this grant (kinds: ${g.allowed_kinds.join(", ")})?`, () =>
                        void handleRevoke(g)
                      )
                    }
                  >
                    Revoke
                  </button>
                  <button onClick={() => setPreviewGrantId(g.id)}>Preview a package with this grant</button>
                </>
              )}
            </li>
          ))}
        </ul>
      )}

      <h4>Issue a new grant</h4>
      <fieldset>
        <legend>Allowed kinds</legend>
        {DISCLOSABLE_KINDS.map((kind) => (
          <label key={kind}>
            <input type="checkbox" checked={selectedKinds.includes(kind)} onChange={() => toggleKind(kind)} />
            {kind}
          </label>
        ))}
      </fieldset>
      <label>
        Byte budget
        <input type="number" value={byteBudget} onChange={(e) => setByteBudget(Number(e.target.value))} />
      </label>
      <label>
        TTL (hours, max 7 days)
        <input type="number" value={ttlHours} onChange={(e) => setTtlHours(Number(e.target.value))} />
      </label>
      <button onClick={() => void handleIssue()} disabled={selectedKinds.length === 0}>
        Issue grant
      </button>

      {previewGrantId && (
        <div className="package-preview-form">
          <h4>Preview package for grant {previewGrantId}</h4>
          <label>
            Request ID
            <input value={requestId} onChange={(e) => setRequestId(e.target.value)} />
          </label>
          <label>
            Principal label
            <input value={principal} onChange={(e) => setPrincipal(e.target.value)} />
          </label>
          <button onClick={() => void handlePreview()} disabled={!requestId.trim()}>
            Preview (no receipt persisted)
          </button>
          <button
            onClick={() =>
              requestConfirm(
                "Compile and persist a disclosure receipt for this request ID? Unlike Preview, this is a real, durable canonical write.",
                () => void handleCompile()
              )
            }
            disabled={!requestId.trim()}
          >
            Compile (persists a receipt)
          </button>
          <button onClick={() => { setPreviewGrantId(null); setPreview(null); setCompiled(null); }}>Close</button>
        </div>
      )}

      {preview && (
        <div className="package-preview-result">
          <p>
            Selected: {preview.receipt.selected.length}, rejected: {preview.receipt.rejected.length}, emitted{" "}
            {preview.receipt.emitted_byte_count} bytes (sha256 {preview.receipt.emitted_sha256.slice(0, 12)}…)
          </p>
          {preview.receipt.rejected.length > 0 && (
            <ul>
              {preview.receipt.rejected.map((r) => (
                <li key={r.object_id}>
                  {r.kind} {r.object_id}: {r.reason}
                </li>
              ))}
            </ul>
          )}
          <textarea
            className="note-body"
            aria-label="Previewed package wire content"
            readOnly
            value={preview.wire}
            rows={10}
          />
        </div>
      )}

      {compiled && (
        <div className="package-preview-result">
          <p>
            Receipt persisted: {compiled.receipt_id} -- selected: {compiled.receipt.selected.length}, emitted{" "}
            {compiled.receipt.emitted_byte_count} bytes (sha256 {compiled.receipt.emitted_sha256.slice(0, 12)}…)
          </p>
          <textarea
            className="note-body"
            aria-label="Compiled package wire content"
            readOnly
            value={compiled.wire}
            rows={10}
          />
        </div>
      )}

      {error && <p className="error">{error}</p>}
    </div>
  );
}
