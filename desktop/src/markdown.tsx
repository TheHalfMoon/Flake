// A minimal, dependency-free, safe Markdown-subset previewer.
//
// Security posture (T04-02 S01/S08: "HTML/remote media inert... no external
// link execution"): this renders React elements only, never raw HTML via
// `dangerouslySetInnerHTML` or an equivalent -- so a note body containing
// literal HTML (e.g. `<script>...</script>` or `<img src=...>`) is always
// displayed as plain visible text, never parsed or executed. Links and
// images are deliberately rendered as plain text (`label (url)`), never as
// a clickable `<a href>` or a loaded `<img src>` -- this preview can issue
// zero network requests and navigate nowhere, by construction, not by a
// runtime check that could be bypassed.

import type { ReactNode } from "react";

interface Segment {
  text: string;
  bold?: boolean;
  italic?: boolean;
  code?: boolean;
}

function parseInline(line: string): Segment[] {
  // Links/images first: [text](url) or ![alt](url) -> plain "text (url)".
  const withLinksResolved = line.replace(
    /!?\[([^\]]*)\]\(([^)]*)\)/g,
    (_m, label: string, url: string) => `${label} (${url})`
  );

  const segments: Segment[] = [];
  const re = /(\*\*([^*]+)\*\*|\*([^*]+)\*|_([^_]+)_|`([^`]+)`)/g;
  let lastIndex = 0;
  let match: RegExpExecArray | null;
  while ((match = re.exec(withLinksResolved)) !== null) {
    if (match.index > lastIndex) {
      segments.push({ text: withLinksResolved.slice(lastIndex, match.index) });
    }
    if (match[2] !== undefined) segments.push({ text: match[2], bold: true });
    else if (match[3] !== undefined) segments.push({ text: match[3], italic: true });
    else if (match[4] !== undefined) segments.push({ text: match[4], italic: true });
    else if (match[5] !== undefined) segments.push({ text: match[5], code: true });
    lastIndex = re.lastIndex;
  }
  if (lastIndex < withLinksResolved.length) {
    segments.push({ text: withLinksResolved.slice(lastIndex) });
  }
  return segments;
}

function renderInline(line: string, keyPrefix: string) {
  return parseInline(line).map((seg, i) => {
    const key = `${keyPrefix}-${i}`;
    if (seg.bold) return <strong key={key}>{seg.text}</strong>;
    if (seg.italic) return <em key={key}>{seg.text}</em>;
    if (seg.code) return <code key={key}>{seg.text}</code>;
    return <span key={key}>{seg.text}</span>;
  });
}

export function MarkdownPreview({ text }: { text: string }) {
  const lines = text.split("\n");
  const blocks: ReactNode[] = [];
  let listItems: string[] = [];
  let blockKey = 0;

  function flushList() {
    if (listItems.length === 0) return;
    blocks.push(
      <ul key={`list-${blockKey++}`}>
        {listItems.map((item, i) => (
          <li key={i}>{renderInline(item, `li-${blockKey}-${i}`)}</li>
        ))}
      </ul>
    );
    listItems = [];
  }

  for (const rawLine of lines) {
    const heading = /^(#{1,3})\s+(.*)$/.exec(rawLine);
    const listItem = /^[-*]\s+(.*)$/.exec(rawLine);
    if (heading) {
      flushList();
      const level = heading[1].length;
      const key = `h-${blockKey++}`;
      const content = renderInline(heading[2], key);
      if (level === 1) blocks.push(<h1 key={key}>{content}</h1>);
      else if (level === 2) blocks.push(<h2 key={key}>{content}</h2>);
      else blocks.push(<h3 key={key}>{content}</h3>);
    } else if (listItem) {
      listItems.push(listItem[1]);
    } else if (rawLine.trim() === "") {
      flushList();
    } else {
      flushList();
      blocks.push(<p key={`p-${blockKey++}`}>{renderInline(rawLine, `p-${blockKey}`)}</p>);
    }
  }
  flushList();

  return <div className="markdown-preview">{blocks}</div>;
}
