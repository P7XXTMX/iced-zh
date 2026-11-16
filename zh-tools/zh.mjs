#!/usr/bin/env node
// iced-zh 双语注释工具。
//
//   node zh-tools/zh.mjs extract <路径前缀...>      列出待翻译单元（英文原文）
//   node zh-tools/zh.mjs apply   [译文文件...]      按译文重写源文件（默认 zh-tools/translations/*.txt）
//   node zh-tools/zh.mjs check   [译文文件...]      校验英文完整保留，并统计覆盖率
//
// 源文件总是从基线提交重新生成，因此 apply 可以反复执行。
// 译文文件格式：
//   @@ path/to/file.rs:行号
//   中文译文（Markdown，不带注释前缀；`=` 表示有意跳过）
import { execFileSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';

const BASE = 'b2fbd3cb6';
const ROOT = path.resolve(path.dirname(new URL(import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, '$1')), '..');
const TR_DIR = path.join(ROOT, 'zh-tools', 'translations');
const SUMMARY = '显示英文原文';

const git = (...args) => execFileSync('git', args, { cwd: ROOT, encoding: 'utf8', maxBuffer: 1 << 28 });
const baseline = (file) => git('show', `${BASE}:${file}`).replace(/\r\n/g, '\n').split('\n');

const DOC = /^(\s*)(\/\/\/|\/\/!)(?!\/)(.*)$/;
const PLAIN = /^(\s*)\/\/(?![\/!])(.*)$/;
const TRAIL = /^(\s*[^\s\/].*?[;,{})\]]\s+)\/\/ (.*)$/;
const REFDEF = /^\s*\[[^\]]+\]:\s/;
const HEADING = /^(\s*#+)\s+(.*)$/;
const hasWords = (s) => /[A-Za-z]{2}/.test(s);

// 把一个文件解析为「原样输出的行」和「可翻译单元」的序列。
function parse(file) {
  const lines = baseline(file);
  const items = []; // {raw: [...]} | {unit}
  const raw = (l) => {
    const last = items[items.length - 1];
    if (last && last.raw) last.raw.push(l);
    else items.push({ raw: [l] });
  };
  const unit = (u) => items.push({ unit: { ...u, id: `${file}:${u.line}` } });

  let i = 0;
  while (i < lines.length) {
    let m = lines[i].match(DOC);
    if (m) {
      const [, indent, marker] = m;
      const prefix = indent + marker;
      const block = [];
      while (i < lines.length) {
        const d = lines[i].match(DOC);
        if (!d || d[1] !== indent || d[2] !== marker) break;
        block.push({ line: i + 1, text: d[3], src: lines[i] });
        i++;
      }
      const context = (lines[i] ?? '').trim();
      // 切分：代码围栏 / 标题 / 散文段
      let fence = false;
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
        unit({ kind: 'doc', line: s[a].line, prefix, en: s.slice(a, t), tail, context, trailingBlank: s.length - b });
      };
      for (const l of block) {
        const t = l.text.trim();
        if (t.startsWith('```') || t.startsWith('~~~')) {
          flush();
          fence = !fence;
          raw(l.src);
        } else if (fence) {
          const c = l.text.match(/^(\s*)\/\/ (.*)$/);
          if (c && hasWords(c[2])) unit({ kind: 'codecmt', line: l.line, prefix: prefix + c[1], en: [l], context });
          else raw(l.src);
        } else if (HEADING.test(l.text)) {
          flush();
          unit({ kind: 'heading', line: l.line, prefix, en: [l], context });
        } else seg.push(l);
      }
      flush();
      const lastItem = items[items.length - 1];
      if (lastItem?.unit?.kind === 'doc') lastItem.unit.last = true;
      continue;
    }
    m = lines[i].match(PLAIN);
    if (m) {
      const indent = m[1];
      const block = [];
      while (i < lines.length) {
        const p = lines[i].match(PLAIN);
        if (!p || p[1] !== indent) break;
        block.push({ line: i + 1, text: p[2], src: lines[i] });
        i++;
      }
      if (block.some((l) => hasWords(l.text)))
        unit({ kind: 'plain', line: block[0].line, prefix: indent + '//', en: block, context: (lines[i] ?? '').trim() });
      else block.forEach((l) => raw(l.src));
      continue;
    }
    m = lines[i].match(TRAIL);
    if (m && hasWords(m[2]) && (m[1].match(/"/g) ?? []).length % 2 === 0) {
      unit({ kind: 'trail', line: i + 1, prefix: '', en: [{ line: i + 1, text: m[2], src: lines[i] }], context: m[1].trim() });
    } else raw(lines[i]);
    i++;
  }
  return items;
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
    case 'codecmt':
      return [...src, ...zh.map((s) => (s === '' ? P : `${P}${u.kind === 'codecmt' ? '// ' : ' '}${s}`))];
    case 'trail':
      return [`${src[0]}（${zh.join(' ')}）`];
  }
}

function loadTranslations(files) {
  if (files.length === 0)
    files = fs.existsSync(TR_DIR) ? fs.readdirSync(TR_DIR).filter((f) => f.endsWith('.txt')).map((f) => path.join(TR_DIR, f)) : [];
  const map = new Map(); // file -> Map(id -> zh lines)
  for (const f of files) {
    let cur = null;
    for (const line of fs.readFileSync(f, 'utf8').replace(/\r\n/g, '\n').split('\n')) {
      const m = line.match(/^@@ (\S+):(\d+)\s*$/);
      if (m) {
        const id = `${m[1]}:${m[2]}`;
        if (!map.has(m[1])) map.set(m[1], new Map());
        if (map.get(m[1]).has(id)) throw new Error(`重复的译文 ${id}（${f}）`);
        cur = [];
        map.get(m[1]).set(id, cur);
      } else if (cur) cur.push(line);
      else if (line.trim() !== '') throw new Error(`${f}: 译文出现在第一个 @@ 之前`);
    }
  }
  for (const m of map.values())
    for (const [id, zh] of m) {
      while (zh.length && zh[zh.length - 1].trim() === '') zh.pop();
      while (zh.length && zh[0].trim() === '') zh.shift();
      if (zh.length === 0) throw new Error(`译文为空：${id}`);
    }
  return map;
}

function build(file, tr) {
  const out = [];
  const used = new Set();
  let total = 0, done = 0, skipped = 0;
  for (const it of parse(file)) {
    if (it.raw) { out.push(...it.raw); continue; }
    const u = it.unit;
    total++;
    const zh = tr?.get(u.id);
    if (zh) used.add(u.id);
    if (!zh || (zh.length === 1 && zh[0].trim() === '=')) {
      if (zh) skipped++;
      out.push(...u.en.map((l) => l.src), ...(u.tail ?? []).map((l) => l.src));
      for (let k = 0; k < (u.trailingBlank ?? 0); k++) out.push(u.prefix);
      continue;
    }
    done++;
    out.push(...render(u, zh));
  }
  if (tr) for (const id of tr.keys()) if (!used.has(id)) throw new Error(`译文 ${id} 找不到对应的注释单元`);
  return { out, total, done, skipped };
}

const listFiles = (prefixes) =>
  git('ls-tree', '-r', '--name-only', BASE, '--', ...prefixes).split('\n').filter((f) => f.endsWith('.rs'));

const [cmd, ...args] = process.argv.slice(2);
if (cmd === 'extract') {
  for (const file of listFiles(args)) {
    let lastCtx = null;
    for (const it of parse(file)) {
      if (!it.unit) continue;
      const u = it.unit;
      console.log(`@@ ${u.id}  [${u.kind}]${u.context !== lastCtx || u.kind !== 'doc' ? `  >> ${u.context.slice(0, 90)}` : ''}`);
      lastCtx = u.context;
      for (const l of u.en) console.log(l.text.replace(/^ /, ''));
    }
  }
} else if (cmd === 'apply' || cmd === 'check') {
  const tr = loadTranslations(args);
  const stats = {};
  let bad = 0;
  const all = cmd === 'check' ? listFiles(['.']) : [...tr.keys()];
  for (const file of all) {
    const { out, total, done, skipped } = build(file, tr.get(file));
    const top = file.startsWith('examples/') ? 'examples' : file.includes('/') && !/^(src|benches)\//.test(file) ? file.split('/')[0] : 'iced';
    const s = (stats[top] ??= { total: 0, done: 0, skipped: 0 });
    s.total += total; s.done += done; s.skipped += skipped;
    if (!tr.has(file)) continue;
    const abs = path.join(ROOT, file);
    if (cmd === 'apply') {
      const eol = fs.readFileSync(abs, 'utf8').includes('\r\n') ? '\r\n' : '\n';
      fs.writeFileSync(abs, out.join(eol));
    } else {
      // 1) 工作区文件必须等于生成结果；2) 基线的每一行都按顺序保留（标题行和行尾注释行除外，它们被就地改为双语）
      const cur = fs.readFileSync(abs, 'utf8').replace(/\r\n/g, '\n');
      if (cur !== out.join('\n')) { console.log(`不一致（需要重新 apply）：${file}`); bad++; }
      const lines = cur.split('\n');
      let k = 0;
      for (const b of baseline(file)) {
        const d = b.match(DOC);
        if (d && d[3].trim() === '') continue;
        const j = lines.indexOf(b, k);
        if (j >= 0) { k = j + 1; continue; }
        const j2 = lines.findIndex((l, n) => n >= k && ((d && HEADING.test(d[3]) && l.endsWith(`（${d[3].match(HEADING)[2]}）`)) || (l.startsWith(b + '（') && l.endsWith('）'))));
        if (j2 >= 0) { k = j2 + 1; continue; }
        console.log(`英文原文丢失：${file}: ${b}`); bad++;
      }
    }
  }
  if (cmd === 'check') {
    console.log('crate          单元   已译   跳过   剩余');
    for (const [c, s] of Object.entries(stats).sort())
      console.log(`${c.padEnd(12)} ${String(s.total).padStart(6)} ${String(s.done).padStart(6)} ${String(s.skipped).padStart(6)} ${String(s.total - s.done - s.skipped).padStart(6)}`);
    console.log(bad ? `\n发现 ${bad} 个问题` : '\n校验通过：英文原文完整保留');
    process.exit(bad ? 1 : 0);
  } else console.log(`已重写 ${all.length} 个文件`);
} else {
  console.log('用法: node zh-tools/zh.mjs extract <路径...> | apply [译文文件...] | check [译文文件...]');
}
