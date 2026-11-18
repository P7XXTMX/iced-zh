#!/usr/bin/env node
// iced-zh 双语注释工具。
//
//   node zh-tools/zh.mjs extract [路径前缀...]         列出注释单元（英文原文）
//   node zh-tools/zh.mjs todo    [路径前缀...]         只列出还没有译文的单元
//   node zh-tools/zh.mjs apply   [译文文件...]         按译文重写源文件（默认 zh-tools/translations/*.txt）
//   node zh-tools/zh.mjs check   [译文文件...]         校验源文件、英文原文和上游文件，并统计覆盖率
//   node zh-tools/zh.mjs migrate [--dry-run] <提交>    把基线换成新的上游提交，并迁移译文
//
// 源文件总是由「基线提交里的英文原文 + 译文」重新生成，因此 apply 可以反复执行。
// 基线提交记录在 zh-tools/BASE 里，只应由 migrate 修改。
// 译文文件格式：
//   @@ path/to/file.rs:行号
//   中文译文（Markdown，不带注释前缀；`=` 表示有意跳过）
import { execFileSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const TR_DIR = path.join(ROOT, 'zh-tools', 'translations');
const BASE_FILE = path.join(ROOT, 'zh-tools', 'BASE');
const MD_SRC = path.join(ROOT, 'zh-tools', 'markdown.zh.txt');
const SUMMARY = '显示英文原文';
// 本分叉自己添加、上游没有的文件。除此之外，任何与基线不同的文件都必须是工具生成的。
const OWN = [/^zh-tools\//, /^TRANSLATION\.md$/, /^CLAUDE\.md$/, /^rust-glossary\.md$/];

const git = (...args) => execFileSync('git', args, { cwd: ROOT, encoding: 'utf8', maxBuffer: 1 << 30 });
const BASE = fs.readFileSync(BASE_FILE, 'utf8').trim();
const isRs = (f) => f.endsWith('.rs');
const short = (rev) => rev.slice(0, 9);
const listFiles = (rev, prefixes = []) =>
  git('ls-tree', '-r', '--name-only', '-z', rev, '--', ...prefixes).split('\0').filter(Boolean);

// 一次读出某个提交里的多个文件（换行统一为 LF）。
function blobs(rev, files) {
  const out = new Map();
  if (files.length === 0) return out;
  const buf = execFileSync('git', ['cat-file', '--batch'], {
    cwd: ROOT,
    input: files.map((f) => `${rev}:${f}\n`).join(''),
    maxBuffer: 1 << 30,
  });
  let p = 0;
  for (const f of files) {
    const nl = buf.indexOf(10, p);
    const m = buf.toString('utf8', p, nl).match(/^[0-9a-f]+ blob (\d+)$/);
    if (!m) throw new Error(`提交 ${short(rev)} 里没有这个文件：${f}`);
    const end = nl + 1 + Number(m[1]);
    out.set(f, buf.toString('utf8', nl + 1, end).replace(/\r\n/g, '\n'));
    p = end + 1;
  }
  return out;
}

const DOCLINE = /^\s*(?:\/\/\/|\/\/!)(?!\/)(.*)$/;
const REFDEF = /^\s*\[[^\]]+\]:\s/;
const HEADING = /^(\s*#+)\s+(.*)$/;
const FENCE = /^\s*(```|~~~)/;
// rustdoc 里表示「这仍然是 Rust 代码」的围栏标记；出现其他单词（如 text）就不是 Rust。
const RUST_FENCE = /^(rust|no_run|ignore(-\S+)?|should_panic|compile_fail|standalone_crate|test_harness|edition\d{4}|E\d{4})$/;
const hasWords = (s) => /[A-Za-z]{2}/.test(s);

// Rust 词法扫描，用来可靠地区分注释、字符串和代码。返回：
//   start[i]  第 i 行行首的状态：0 代码，1 字符串字面量内，2 块注释内
//   cmt[i]    第 i 行里行注释 `//` 的起始列，没有则为 -1
//   blocks    块注释 /* */ 的位置 { sl, sc, el, ec }（行、列；ec 是 `*/` 之后的列）
function lex(text) {
  const start = [0], cmt = [-1], blocks = [];
  const CHAR = /'(?:\\(?:[nrt\\0'"]|x[0-9a-fA-F]{2}|u\{[0-9a-fA-F_]{1,8}\})|[^\\'\n])'/uy;
  const RAW = /(?:br|cr|r)(#*)"/y;
  const n = text.length;
  let i = 0, line = 0, col0 = 0, state = 0, hashes = 0, depth = 0, open = null;
  while (i < n) {
    const c = text[i];
    if (c === '\n') {
      i++; line++; col0 = i;
      start.push(state === 0 ? 0 : state === 3 ? 2 : 1);
      cmt.push(-1);
    } else if (state === 0) {
      if (c === '/' && text[i + 1] === '/') {
        cmt[line] = i - col0;
        const e = text.indexOf('\n', i);
        i = e < 0 ? n : e;
      } else if (c === '/' && text[i + 1] === '*') {
        state = 3; depth = 1; open = { sl: line, sc: i - col0 }; i += 2;
      } else if (c === '"') {
        state = 1; i++;
      } else if ((c === 'r' || c === 'b' || c === 'c') && !/\w/.test(text[i - 1] ?? ' ')) {
        RAW.lastIndex = i;
        const m = RAW.exec(text);
        if (m) { state = 2; hashes = m[1].length; i += m[0].length; } else i++;
      } else if (c === "'") {
        // 字符字面量；匹配不上就是生命周期或标签
        CHAR.lastIndex = i;
        i = CHAR.test(text) ? CHAR.lastIndex : i + 1;
      } else i++;
    } else if (state === 1) {
      if (c === '\\') i += text[i + 1] === '\n' ? 1 : 2;
      else { if (c === '"') state = 0; i++; }
    } else if (state === 2) {
      if (c === '"' && text.startsWith('#'.repeat(hashes), i + 1)) { state = 0; i += 1 + hashes; } else i++;
    } else if (c === '/' && text[i + 1] === '*') {
      depth++; i += 2;
    } else if (c === '*' && text[i + 1] === '/') {
      i += 2;
      if (--depth === 0) { state = 0; blocks.push({ ...open, el: line, ec: i - col0 }); }
    } else i++;
  }
  return { start, cmt, blocks };
}

// 把一个文件解析为「原样输出的行」和「可翻译单元」的序列。
// 单元类型：doc（文档散文段）、heading（文档标题）、plain（普通 // 注释块）、block（独占整行的 /* */）、
// codecmt（文档代码示例里的整行注释）、trail（行尾注释，包括文档代码示例里的）。
function parse(file, text) {
  const lines = text.split('\n');
  const { start, cmt, blocks } = lex(text);
  const items = []; // { raw: [...] } | { unit }
  const unhandled = []; // 含英文、但工具不处理的注释
  const raw = (l) => {
    const last = items[items.length - 1];
    if (last?.raw) last.raw.push(l);
    else items.push({ raw: [l] });
  };
  const unit = (u) => items.push({ unit: { ...u, id: `${file}:${u.line}` } });

  // 独占若干整行的普通块注释可以成为单元；夹在代码中间的块注释和 /** */ 文档块注释只报告，不处理。
  const standalone = new Map();
  for (const b of blocks) {
    const head = lines[b.sl].slice(b.sc);
    const isDoc = head.startsWith('/*!') || (head.startsWith('/**') && !head.startsWith('/***') && !head.startsWith('/**/'));
    const alone = lines[b.sl].slice(0, b.sc).trim() === '' && lines[b.el].slice(b.ec).trim() === '';
    const body = lines.slice(b.sl, b.el + 1);
    body[body.length - 1] = body[body.length - 1].slice(0, b.ec);
    body[0] = body[0].slice(b.sc);
    if (!isDoc && alone) standalone.set(b.sl, b);
    else if (hasWords(body.join('\n'))) unhandled.push({ line: b.sl + 1, text: lines[b.sl].trim() });
  }

  // 整行的行注释：{ doc, indent, marker, text }；其他情况返回 null。
  const cls = (i) => {
    const line = lines[i];
    const first = line.search(/\S/);
    if (start[i] !== 0 || first < 0 || cmt[i] !== first) return null;
    const s = line.slice(first), indent = line.slice(0, first);
    if (s.startsWith('//!')) return { doc: true, indent, marker: '//!', text: s.slice(3) };
    if (s.startsWith('///') && s[3] !== '/') return { doc: true, indent, marker: '///', text: s.slice(3) };
    return { doc: false, indent, marker: '//', text: s.slice(2) };
  };

  // 文档代码示例：整段做词法扫描，只把真正的注释当作单元。
  const codeLines = (ls, fenceText, prefix, context) => {
    const info = fenceText.trim().replace(/^[`~]+/, '');
    if (!info.split(/[\s,]+/).every((w) => w === '' || RUST_FENCE.test(w))) return ls.forEach((l) => raw(l.src));
    const hidden = (s) => /^\s*#(\s|$)/.test(s); // rustdoc 隐藏行，读者看不到
    const lx = lex(ls.map((l) => (hidden(l.text) ? l.text.replace('#', ' ') : l.text)).join('\n'));
    ls.forEach((l, n) => {
      const at = lx.cmt[n];
      if (at < 0 || hidden(l.text)) return raw(l.src);
      const body = l.text.slice(at);
      if (lx.start[n] === 0 && l.text.slice(0, at).trim() === '') {
        const marker = body.startsWith('//!') ? '//!' : body.startsWith('///') && body[3] !== '/' ? '///' : '//';
        if (hasWords(body.slice(marker.length)))
          unit({ kind: 'codecmt', line: l.line, prefix: prefix + l.text.slice(0, at), marker, en: [l], context });
        else raw(l.src);
      } else if (hasWords(body.slice(2))) {
        unit({ kind: 'trail', line: l.line, prefix: '', en: [{ line: l.line, text: body.slice(2), src: l.src }], context: l.text.slice(0, at).trim() });
      } else raw(l.src);
    });
  };

  let i = 0;
  while (i < lines.length) {
    const c = cls(i);
    if (c?.doc) {
      const prefix = c.indent + c.marker;
      const block = [];
      for (let d; i < lines.length && (d = cls(i)) && d.doc && d.indent === c.indent && d.marker === c.marker; i++)
        block.push({ line: i + 1, text: d.text, src: lines[i] });
      const context = (lines[i] ?? '').trim();
      // 切分：代码围栏 / 标题 / 散文段
      let seg = [];
      const flush = () => {
        if (seg.length === 0) return;
        const s = seg;
        seg = [];
        if (!s.some((l) => hasWords(l.text) && !REFDEF.test(l.text))) return s.forEach((l) => raw(l.src));
        let a = 0, b = s.length;
        while (a < b && s[a].text.trim() === '') a++;
        while (b > a && s[b - 1].text.trim() === '') b--;
        // 尾部的引用式链接定义留在折叠区之外
        let t = b;
        while (t > a && (REFDEF.test(s[t - 1].text) || s[t - 1].text.trim() === '')) t--;
        const tail = s.slice(t, b).filter((l) => l.text.trim() !== '');
        s.slice(0, a).forEach((l) => raw(l.src));
        unit({ kind: 'doc', line: s[a].line, prefix, en: s.slice(a, t), tail, all: s.slice(a), context, trailingBlank: s.length - b });
      };
      for (let k = 0; k < block.length; ) {
        const l = block[k];
        if (FENCE.test(l.text)) {
          flush();
          raw(l.src);
          let e = k + 1;
          while (e < block.length && !FENCE.test(block[e].text)) e++;
          codeLines(block.slice(k + 1, e), l.text, prefix, context);
          if (e < block.length) raw(block[e].src);
          k = e + 1;
        } else if (HEADING.test(l.text)) {
          flush();
          unit({ kind: 'heading', line: l.line, prefix, en: [l], context });
          k++;
        } else {
          seg.push(l);
          k++;
        }
      }
      flush();
      const lastItem = items[items.length - 1];
      if (lastItem?.unit?.kind === 'doc') lastItem.unit.last = true;
      continue;
    }
    if (c) {
      const block = [];
      for (let p; i < lines.length && (p = cls(i)) && !p.doc && p.indent === c.indent; i++)
        block.push({ line: i + 1, text: p.text, src: lines[i] });
      if (block.some((l) => hasWords(l.text)))
        unit({ kind: 'plain', line: block[0].line, prefix: c.indent + '//', en: block, context: (lines[i] ?? '').trim() });
      else block.forEach((l) => raw(l.src));
      continue;
    }
    const b = standalone.get(i);
    if (b) {
      const en = [];
      for (let k = b.sl; k <= b.el; k++) {
        let t = k === b.el ? lines[k].slice(0, b.ec - 2) : lines[k];
        if (k === b.sl) t = t.slice(b.sc + 2);
        en.push({ line: k + 1, text: t.replace(/^\s*\*+(?=\s|$)/, ''), src: lines[k] });
      }
      i = b.el + 1;
      if (en.some((l) => hasWords(l.text)))
        unit({ kind: 'block', line: b.sl + 1, prefix: lines[b.sl].slice(0, b.sc), en, context: (lines[i] ?? '').trim() });
      else en.forEach((l) => raw(l.src));
      continue;
    }
    if (cmt[i] >= 0 && hasWords(lines[i].slice(cmt[i] + 2)))
      unit({ kind: 'trail', line: i + 1, prefix: '', en: [{ line: i + 1, text: lines[i].slice(cmt[i] + 2), src: lines[i] }], context: lines[i].slice(0, cmt[i]).trim() });
    else raw(lines[i]);
    i++;
  }
  return { items, unhandled };
}

function render(u, zh) {
  const P = u.prefix;
  const pl = (s) => (s === '' ? P : `${P} ${s}`);
  const src = u.en.map((l) => l.src);
  switch (u.kind) {
    case 'doc': {
      const out = [...zh.map(pl), P, pl('<details>'), pl(`<summary>${SUMMARY}</summary>`), P, ...src, P, pl('</details>')];
      if (u.tail.length) out.push(P, ...u.tail.map((l) => l.src));
      for (let k = 0; k < u.trailingBlank; k++) out.push(P);
      // `</details>` 之后必须有空行，否则紧随其后的代码块或标题会被当成 HTML 块的一部分
      if (!u.last && out[out.length - 1] !== P) out.push(P);
      return out;
    }
    case 'heading': {
      const [, hashes, text] = u.en[0].text.match(HEADING);
      // clippy 靠这几个标题原文识别文档小节（missing_safety_doc 等），必须保持英文
      if (/^(Safety|Panics|Errors)$/.test(text.trim())) return [u.en[0].src];
      // 译文首行写 `{#原锚点}` 可以保留英文标题的锚点，供其他文档里的 `#fragment` 链接使用
      const anchor = zh[0].match(/^\{#(.+)\}$/);
      const title = `${P}${hashes} ${zh.slice(anchor ? 1 : 0).join(' ')}（${text}）`;
      return anchor ? [pl(`<a id="${anchor[1]}"></a>`), P, title] : [title];
    }
    case 'plain':
      return [...src, ...zh.map(pl)];
    case 'codecmt':
      return [...src, ...zh.map((s) => (s === '' ? P : `${P}${u.marker} ${s}`))];
    case 'block':
      return [...src, ...zh.map((s) => (s === '' ? `${P}//` : `${P}// ${s}`))];
    case 'trail':
      return [`${src[0]}（${zh.join(' ')}）`];
  }
}

const HEADER = /^@@ (\S+):(\d+)\s*$/;
// 译文条目里以 `!!` 开头的行表示「原文有改动，译文已过期」（由 migrate 写入）。
// 过期的条目按未翻译处理，直到译文更新、`!!` 行被删除。
const STALE = '!!';
const translationFiles = () =>
  fs.existsSync(TR_DIR) ? fs.readdirSync(TR_DIR).filter((f) => f.endsWith('.txt')).sort().map((f) => path.join(TR_DIR, f)) : [];

function loadTranslations(files) {
  if (files.length === 0) files = translationFiles();
  const map = new Map(); // 文件 -> Map(单元编号 -> 译文行)
  for (const f of files) {
    let cur = null;
    for (const line of fs.readFileSync(f, 'utf8').replace(/\r\n/g, '\n').split('\n')) {
      const m = line.match(HEADER);
      if (m) {
        const id = `${m[1]}:${m[2]}`;
        if (!map.has(m[1])) map.set(m[1], new Map());
        if (map.get(m[1]).has(id)) throw new Error(`重复的译文 ${id}（${f}）`);
        cur = [];
        map.get(m[1]).set(id, cur);
      } else if (cur && line.startsWith(STALE)) (cur.notes ??= []).push(line);
      else if (cur) cur.push(line);
      else if (line.trim() !== '') throw new Error(`${f}: 译文出现在第一个 @@ 之前`);
    }
  }
  for (const m of map.values())
    for (const [id, zh] of m) {
      while (zh.length && zh[zh.length - 1].trim() === '') zh.pop();
      while (zh.length && zh[0].trim() === '') zh.shift();
      if (zh.length === 0 && !zh.notes) throw new Error(`译文为空：${id}`);
    }
  return map;
}

// 生成一个文件。没有译文（或标记为 `=`）的单元原样输出，因此不带译文时结果与基线完全相同。
function build(file, text, tr) {
  const { items, unhandled } = parse(file, text);
  const out = [], used = new Set(), todo = [];
  let total = 0, done = 0, skipped = 0;
  for (const it of items) {
    if (it.raw) { out.push(...it.raw); continue; }
    const u = it.unit;
    total++;
    const zh = tr?.get(u.id);
    if (zh) used.add(u.id);
    const pending = !zh || Boolean(zh.notes); // 没有译文，或译文已过期
    if (pending || (zh.length === 1 && zh[0].trim() === '=')) {
      if (pending) todo.push({ u, zh });
      else skipped++;
      out.push(...(u.all ?? u.en).map((l) => l.src));
      continue;
    }
    done++;
    out.push(...render(u, zh));
  }
  if (tr)
    for (const id of tr.keys())
      if (!used.has(id)) throw new Error(`译文 ${id} 找不到对应的注释单元（行号是基线 ${short(BASE)} 里的行号，可以用 extract 查看）`);
  return { out, total, done, skipped, todo, unhandled };
}

const printUnit = (u, withContext) => {
  console.log(`@@ ${u.id}  [${u.kind}]${withContext ? `  >> ${u.context.slice(0, 90)}` : ''}`);
  for (const l of u.en) if (u.kind !== 'block' || l.text.trim()) console.log(l.text.replace(/^ /, ''));
};

// ───────────────────────────── migrate ─────────────────────────────

const unitsOf = (file, text) => parse(file, text).items.filter((it) => it.unit).map((it) => it.unit);
// 单元的「指纹」：英文内容，忽略换行和缩进的变化。
const sig = (u) => u.en.map((l) => l.text).join(' ').replace(/\s+/g, ' ').trim();
const words = (s) => s.toLowerCase().match(/[a-z0-9_]+/g) ?? [];
// 两段英文的相似度（0~1）。除了整体重合度，也照顾「原文只是被加长或截短」的情况。
function similarity(a, b) {
  const A = words(a), B = words(b), cnt = new Map();
  for (const w of A) cnt.set(w, (cnt.get(w) ?? 0) + 1);
  let common = 0;
  for (const w of B) {
    const c = cnt.get(w);
    if (c) { common++; cnt.set(w, c - 1); }
  }
  if (A.length + B.length === 0) return 0;
  const dice = (2 * common) / (A.length + B.length);
  const small = Math.min(A.length, B.length);
  return small >= 3 ? Math.max(dice, (0.9 * common) / small) : dice;
}

// 最长公共子序列，返回配对的下标。
function lcs(a, b) {
  const n = a.length, m = b.length, pairs = [];
  let s = 0;
  while (s < n && s < m && a[s] === b[s]) { pairs.push([s, s]); s++; }
  let e = 0;
  while (e < n - s && e < m - s && a[n - 1 - e] === b[m - 1 - e]) e++;
  const A = a.slice(s, n - e), B = b.slice(s, m - e), w = B.length + 1;
  const dp = new Uint32Array((A.length + 1) * w);
  for (let i = A.length - 1; i >= 0; i--)
    for (let j = B.length - 1; j >= 0; j--)
      dp[i * w + j] = A[i] === B[j] ? dp[(i + 1) * w + j + 1] + 1 : Math.max(dp[(i + 1) * w + j], dp[i * w + j + 1]);
  for (let i = 0, j = 0; i < A.length && j < B.length; ) {
    if (A[i] === B[j]) { pairs.push([s + i, s + j]); i++; j++; }
    else if (dp[(i + 1) * w + j] >= dp[i * w + j + 1]) i++;
    else j++;
  }
  for (let k = 0; k < e; k++) pairs.push([n - e + k, m - e + k]);
  return pairs;
}

function migrate(args) {
  const dry = args.includes('--dry-run');
  const target = args.find((a) => !a.startsWith('--'));
  if (!target) throw new Error('用法：node zh-tools/zh.mjs migrate [--dry-run] <新的上游提交，例如 upstream/master>');
  const OLD = BASE;
  const NEW = git('rev-parse', '--verify', `${target}^{commit}`).trim();
  if (NEW === OLD) return console.log(`基线已经是 ${short(OLD)}，不需要迁移。`);
  let ahead = '?';
  try {
    execFileSync('git', ['merge-base', '--is-ancestor', OLD, NEW], { cwd: ROOT });
    ahead = git('rev-list', '--count', `${OLD}..${NEW}`).trim();
  } catch {
    console.log(`注意：当前基线 ${short(OLD)} 不是 ${short(NEW)} 的祖先，将直接按两个提交的差异迁移。`);
  }

  // 1. 上游改了哪些文件
  const changes = new Map(); // 旧路径 -> 新路径（null 表示已删除）
  const added = [];
  const z = git('diff', '--name-status', '-M', '-z', OLD, NEW).split('\0');
  for (let k = 0; k + 1 < z.length; ) {
    const st = z[k++];
    if (st[0] === 'R') { const from = z[k++]; changes.set(from, z[k++]); }
    else { const p = z[k++]; if (st[0] === 'A') added.push(p); else changes.set(p, st[0] === 'D' ? null : p); }
  }
  const pairs = [...changes].filter(([from]) => isRs(from)).map(([from, to]) => ({ from, to: to && isRs(to) ? to : null }));
  const addedRs = [...added, ...[...changes].filter(([from, to]) => !isRs(from) && to).map(([, to]) => to)].filter(isRs);

  // 2. 读入译文条目
  const entries = [];
  const byPath = new Map(); // 路径 -> Map(行号 -> 条目)
  for (const tf of translationFiles()) {
    let cur = null;
    for (const line of fs.readFileSync(tf, 'utf8').replace(/\r\n/g, '\n').split('\n')) {
      const m = line.match(HEADER);
      if (m) {
        cur = { tf, path: m[1], line: Number(m[2]), zh: [] };
        entries.push(cur);
        if (!byPath.has(cur.path)) byPath.set(cur.path, new Map());
        byPath.get(cur.path).set(cur.line, cur);
      } else if (cur) cur.zh.push(line);
    }
  }
  const oldRs = new Set(listFiles(OLD).filter(isRs));
  for (const e of entries) {
    if (!oldRs.has(e.path)) throw new Error(`译文 ${e.path}:${e.line} 引用的文件在基线 ${short(OLD)} 里不存在`);
    if (!changes.has(e.path)) e.to = { path: e.path, line: e.line };
  }

  // 3. 对上游改过的文件，按英文内容把旧单元和新单元对齐
  const oldTexts = blobs(OLD, pairs.map((p) => p.from));
  const newTexts = blobs(NEW, [...pairs.filter((p) => p.to).map((p) => p.to), ...addedRs]);
  const leftOld = []; // 有译文、但没对上的旧单元
  const leftNew = []; // 没对上的新单元
  for (const p of pairs) {
    const uo = unitsOf(p.from, oldTexts.get(p.from));
    const un = p.to ? unitsOf(p.to, newTexts.get(p.to)) : [];
    const mine = byPath.get(p.from) ?? new Map();
    const known = new Set(uo.map((u) => u.line));
    for (const line of mine.keys())
      if (!known.has(line)) throw new Error(`译文 ${p.from}:${line} 在基线 ${short(OLD)} 里找不到对应的注释单元`);
    const mo = new Set(), mn = new Set();
    for (const [a, b] of lcs(uo.map(sig), un.map(sig))) {
      mo.add(a); mn.add(b);
      const e = mine.get(uo[a].line);
      if (e) e.to = { path: p.to, line: un[b].line };
    }
    uo.forEach((u, a) => { if (!mo.has(a) && mine.has(u.line)) leftOld.push({ e: mine.get(u.line), u, p }); });
    un.forEach((u, b) => { if (!mn.has(b)) leftNew.push({ u, p }); });
  }
  for (const f of addedRs) for (const u of unitsOf(f, newTexts.get(f))) leftNew.push({ u, p: { from: null, to: f } });

  // 4. 内容完全相同、只是换了位置（甚至换了文件）的注释，译文跟着走
  const pool = new Map();
  for (const o of leftOld) {
    const s = sig(o.u);
    if (!pool.has(s)) pool.set(s, []);
    pool.get(s).push(o);
  }
  let moved = 0;
  for (const nw of leftNew) {
    const c = pool.get(sig(nw.u));
    if (!c?.length) continue;
    const k = Math.max(0, c.findIndex((o) => o.p === nw.p));
    const [o] = c.splice(k, 1);
    o.e.to = { path: nw.p.to, line: nw.u.line };
    nw.taken = true;
    moved++;
  }

  // 5. 剩下的：原文改了（配上最相似的新单元，方便对照着重译）、原文删了、全新的
  const stale = leftOld.filter((o) => !o.e.to);
  const fresh = leftNew.filter((nw) => !nw.taken);
  for (const o of stale) {
    let best = null, score = 0.5;
    for (const nw of fresh) {
      if (nw.p !== o.p || nw.old) continue;
      const s = similarity(sig(o.u), sig(nw.u));
      if (s > score) { score = s; best = nw; }
    }
    if (!best) continue;
    best.old = o;
    o.now = best;
    // 旧译文留在原处并标记为过期，方便对照着修改
    o.e.to = { path: best.p.to, line: best.u.line };
    o.e.staleFrom = o.u.en.map((l) => l.text.replace(/^ /, ''));
  }
  const changed = fresh.filter((nw) => nw.old);
  const brandNew = fresh.filter((nw) => !nw.old);
  const removed = stale.filter((o) => !o.now);
  const rekeyed = entries.filter((e) => e.to && (e.to.path !== e.path || e.to.line !== e.line)).length;
  const seen = new Set();
  for (const e of entries) {
    if (!e.to) continue;
    const id = `${e.to.path}:${e.to.line}`;
    if (seen.has(id)) throw new Error(`迁移后出现重复的译文 ${id}（工具缺陷）`);
    seen.add(id);
  }

  // 6. 报告
  const mdDocs = fs.existsSync(MD_SRC) ? new Set([...fs.readFileSync(MD_SRC, 'utf8').matchAll(/^@@@ (\S+)/gm)].map((m) => m[1])) : new Set();
  const mdChanged = [...changes].filter(([from]) => mdDocs.has(from));
  const mdAdded = added.filter((p) => p.endsWith('.md'));
  const renamed = pairs.filter((p) => p.to && p.to !== p.from);
  console.log(`基线：${short(OLD)} → ${short(NEW)}（${ahead} 个上游提交）`);
  console.log(`上游改动的 Rust 文件：${pairs.length} 个（其中删除 ${pairs.filter((p) => !p.to).length}，重命名 ${renamed.length}），新增 ${addedRs.length} 个`);
  console.log(`译文条目：${entries.length} 条，其中 ${rekeyed} 条需要更新行号或路径${moved ? `（${moved} 条随注释移动）` : ''}`);
  console.log(`原文有改动、译文标记为过期：${changed.length}`);
  console.log(`原文已删除、译文作废：${removed.length}`);
  console.log(`新增的注释、需要翻译：${brandNew.length}`);
  console.log(`Markdown：已翻译的文件中有 ${mdChanged.length} 个被上游改动，上游新增 ${mdAdded.length} 个`);
  for (const p of renamed) console.log(`  重命名：${p.from} → ${p.to}`);
  for (const p of pairs.filter((q) => !q.to)) console.log(`  已删除：${p.from}`);
  for (const [from, to] of mdChanged) console.log(`  Markdown ${to ? '有改动' : '已删除'}：${from}`);
  for (const p of mdAdded) console.log(`  Markdown 新增：${p}`);
  const zhOf = (e) => e.zh.join('\n').trim();
  if (changed.length) {
    console.log(`\n━━━ 原文有改动：旧译文已标记为过期（${STALE} 行）。请对照新的英文更新译文，然后删除 ${STALE} 行 ━━━`);
    for (const nw of changed) {
      console.log('');
      printUnit(nw.u, true);
      console.log(`--- 旧英文（${nw.old.e.path}:${nw.old.e.line}）`);
      for (const l of nw.old.u.en) console.log(l.text.replace(/^ /, ''));
      console.log('--- 旧译文');
      console.log(zhOf(nw.old.e));
    }
  }
  if (removed.length) {
    console.log('\n━━━ 原文已删除：下列译文已从译文文件中移出 ━━━');
    for (const o of removed) {
      console.log(`\n@@ ${o.e.path}:${o.e.line}  [${o.u.kind}]（旧基线）`);
      for (const l of o.u.en) console.log(l.text.replace(/^ /, ''));
      console.log('--- 旧译文');
      console.log(zhOf(o.e));
    }
  }
  if (brandNew.length) {
    console.log('\n━━━ 新增的注释：需要翻译 ━━━');
    for (const nw of brandNew) printUnit(nw.u, true);
  }
  if (dry) return console.log('\n（--dry-run：没有修改任何文件）');

  // 7. 改写译文文件和基线
  let touched = 0;
  for (const tf of translationFiles()) {
    const rawText = fs.readFileSync(tf, 'utf8');
    const eol = rawText.includes('\r\n') ? '\r\n' : '\n';
    const out = [];
    let keep = true, k = 0;
    const mine = entries.filter((e) => e.tf === tf);
    for (const line of rawText.replace(/\r\n/g, '\n').split('\n')) {
      if (HEADER.test(line)) {
        const e = mine[k++];
        keep = Boolean(e.to);
        if (keep) out.push(`@@ ${e.to.path}:${e.to.line}`);
        // 已经标记过期的条目保留原来的 `!!` 行：它们才是现有译文所对应的原文
        if (e.staleFrom && !e.zh.some((l) => l.startsWith(STALE)))
          out.push(`${STALE} 原文有改动，下面是旧原文。更新译文后请删除所有以 ${STALE} 开头的行`, ...e.staleFrom.map((t) => (t ? `${STALE} ${t}` : STALE)));
      } else if (keep) out.push(line);
    }
    const next = out.join(eol);
    if (next !== rawText) { fs.writeFileSync(tf, next); touched++; }
  }
  fs.writeFileSync(BASE_FILE, `${NEW}\n`);
  // 基线必须是当前分支历史的一部分，否则别人克隆之后找不到这个提交
  const quiet = (...a) => { try { return execFileSync('git', a, { cwd: ROOT, encoding: 'utf8', stdio: ['ignore', 'pipe', 'ignore'] }).trim(); } catch { return null; } };
  if (quiet('merge-base', '--is-ancestor', NEW, 'HEAD') === null && quiet('rev-parse', '--verify', '-q', 'MERGE_HEAD') !== NEW)
    console.log(`\n提醒：${short(NEW)} 还没有合并进当前分支。请先运行 git merge --no-commit ${target}，再运行 apply。`);
  console.log(`\n已更新 zh-tools/BASE 和 ${touched} 个译文文件。接下来：`);
  console.log('  node zh-tools/zh.mjs apply && node zh-tools/md.mjs apply    # 重新生成双语文件');
  console.log('  node zh-tools/zh.mjs check && node zh-tools/md.mjs check    # 校验，并列出待翻译的内容');
}

// ───────────────────────────── 命令 ─────────────────────────────

const [cmd, ...args] = process.argv.slice(2);
try {
  if (cmd === 'extract' || cmd === 'todo') {
    const files = listFiles(BASE, args).filter(isRs);
    const texts = blobs(BASE, files);
    const tr = cmd === 'todo' ? loadTranslations([]) : null;
    let n = 0;
    for (const file of files) {
      let lastCtx = null;
      for (const it of parse(file, texts.get(file)).items) {
        const u = it.unit;
        if (!u) continue;
        const zh = tr?.get(file)?.get(u.id);
        if (zh && !zh.notes) continue;
        printUnit(u, u.context !== lastCtx || u.kind !== 'doc');
        if (zh) {
          console.log('--- 译文已过期，译文文件里现有的内容：');
          for (const l of [...zh.notes, ...zh]) console.log(l);
        }
        lastCtx = u.context;
        n++;
      }
    }
    if (cmd === 'todo') console.error(n ? `待翻译：${n} 个单元` : '没有待翻译的单元');
  } else if (cmd === 'apply') {
    // 不带参数：让所有 Rust 文件都等于「基线 + 译文」的生成结果（没有译文的文件还原为上游内容）。
    // 带译文文件参数：只生成这些译文涉及的文件。内容没有变化的文件不会被重写。
    const tr = loadTranslations(args);
    const all = listFiles(BASE).filter(isRs);
    const known = new Set(all);
    for (const f of tr.keys()) if (!known.has(f)) throw new Error(`译文引用的文件在基线 ${short(BASE)} 里不存在：${f}`);
    const files = args.length ? [...tr.keys()] : all;
    const texts = blobs(BASE, files);
    let written = 0;
    const restored = [];
    for (const file of files) {
      const { out } = build(file, texts.get(file), tr.get(file));
      const abs = path.join(ROOT, file);
      const cur = fs.existsSync(abs) ? fs.readFileSync(abs, 'utf8') : null;
      const next = out.join(cur?.includes('\r\n') ? '\r\n' : '\n');
      if (cur === next) continue;
      fs.mkdirSync(path.dirname(abs), { recursive: true });
      fs.writeFileSync(abs, next);
      written++;
      if (!tr.has(file)) restored.push(file);
    }
    console.log(`已生成 ${files.length} 个文件，其中 ${written} 个有变化（基线 ${short(BASE)}）`);
    for (const f of restored) console.log(`  已还原为上游内容（这个文件没有译文）：${f}`);
  } else if (cmd === 'check') {
    const subset = args.length > 0; // 只校验给定译文文件涉及的源文件
    const tr = loadTranslations(args);
    const files = listFiles(BASE).filter(isRs);
    const texts = blobs(BASE, files);
    const stats = {}, problems = [], unhandled = [];
    let stale = 0;
    for (const file of files) {
      const text = texts.get(file);
      if (build(file, text).out.join('\n') !== text) problems.push(`解析后无法原样还原（工具缺陷）：${file}`);
      const r = build(file, text, tr.get(file));
      const top = file.startsWith('examples/') ? 'examples' : file.includes('/') && !/^(src|benches)\//.test(file) ? file.split('/')[0] : 'iced';
      const s = (stats[top] ??= { total: 0, done: 0, skipped: 0 });
      s.total += r.total; s.done += r.done; s.skipped += r.skipped;
      stale += r.todo.filter((t) => t.zh).length;
      for (const u of r.unhandled) unhandled.push(`${file}:${u.line}: ${u.text}`);
      if (subset && !tr.has(file)) continue;
      const abs = path.join(ROOT, file);
      if (!fs.existsSync(abs)) { problems.push(`缺少文件：${file}`); continue; }
      // 1) 工作区文件必须等于生成结果
      const cur = fs.readFileSync(abs, 'utf8').replace(/\r\n/g, '\n');
      if (cur !== r.out.join('\n'))
        problems.push(tr.has(file) ? `不一致（需要重新 apply）：${file}` : `与基线不同（这个文件没有译文，应当与上游完全一致）：${file}`);
      // 2) 基线的每一行都按顺序保留（标题行和行尾注释行被就地改为双语）
      const lines = cur.split('\n');
      let k = 0;
      for (const b of text.split('\n')) {
        const d = b.match(DOCLINE);
        if (d && d[1].trim() === '') continue;
        const j = lines.indexOf(b, k);
        if (j >= 0) { k = j + 1; continue; }
        const j2 = lines.findIndex((l, n) => n >= k && ((d && HEADING.test(d[1]) && l.endsWith(`（${d[1].match(HEADING)[2]}）`)) || (l.startsWith(b + '（') && l.endsWith('）'))));
        if (j2 >= 0) { k = j2 + 1; continue; }
        problems.push(`英文原文丢失：${file}: ${b}`);
      }
    }
    // 3) 除了生成的文件和本分叉自己的文件，其余都必须与上游基线完全一致
    if (!subset) {
      const md = fs.existsSync(MD_SRC) ? new Set([...fs.readFileSync(MD_SRC, 'utf8').matchAll(/^@@@ (\S+)/gm)].map((m) => m[1])) : new Set();
      const z = git('diff', '--name-status', '--no-renames', '-z', BASE, '--').split('\0');
      const reported = new Set();
      for (let k = 0; k + 1 < z.length; k += 2) {
        const [st, p] = [z[k], z[k + 1]];
        if (OWN.some((re) => re.test(p)) || reported.has(p)) continue;
        reported.add(p);
        if (st[0] === 'U') problems.push(`合并冲突还没有标记为已解决：${p}（确认内容无误后 git add；上游已删除的文件用 git rm）`);
        else if (st === 'A') problems.push(`基线里没有这个文件：${p}（上游已删除就用 git rm；本分叉自己添加的文件请加入 zh.mjs 的 OWN 列表）`);
        else if (st === 'D') problems.push(`缺少上游文件：${p}`);
        else if (!isRs(p) && !md.has(p)) problems.push(`与上游不一致（这个文件不应被修改）：${p}`);
      }
    }
    console.log(`基线：${short(BASE)}`);
    console.log('crate          单元   已译   跳过   剩余');
    const sum = { total: 0, done: 0, skipped: 0 };
    const row = (c, s) => console.log(`${c.padEnd(12)} ${String(s.total).padStart(6)} ${String(s.done).padStart(6)} ${String(s.skipped).padStart(6)} ${String(s.total - s.done - s.skipped).padStart(6)}`);
    for (const [c, s] of Object.entries(stats).sort()) {
      row(c, s);
      sum.total += s.total; sum.done += s.done; sum.skipped += s.skipped;
    }
    row('合计', sum);
    if (sum.total - sum.done - sum.skipped > 0 && !subset)
      console.log(`\n还有待翻译的单元${stale ? `（其中 ${stale} 个是原文有改动、译文已过期）` : ''}：运行 node zh-tools/zh.mjs todo 查看`);
    if (unhandled.length) {
      console.log(`\n含英文、但工具不处理的注释（夹在代码中间的 /* */ 或 /** */ 文档块注释）：${unhandled.length}`);
      for (const u of unhandled) console.log(`  ${u}`);
    }
    for (const p of problems) console.log(p);
    console.log(problems.length ? `\n发现 ${problems.length} 个问题` : '\n校验通过：源文件与生成结果一致，英文原文完整保留，其余文件与上游一致');
    process.exit(problems.length ? 1 : 0);
  } else if (cmd === 'migrate') {
    migrate(args);
  } else {
    console.log('用法: node zh-tools/zh.mjs extract|todo [路径...] | apply|check [译文文件...] | migrate [--dry-run] <提交>');
  }
} catch (e) {
  console.error(`错误：${e.message}`);
  process.exit(2);
}
