#!/usr/bin/env node
// Markdown 双语生成：中文译文在前，基线里的英文原文整体折叠在后。
//   node zh-tools/md.mjs apply   按 markdown.zh.txt 重新生成 Markdown 文件
//   node zh-tools/md.mjs check   校验文件与生成结果一致、英文原文完整，并指出原文已更新的文件
//
// markdown.zh.txt 格式：每个文件以 `@@@ 路径 原文哈希` 开头，后面是该文件的完整中文译文。
// 「原文哈希」是翻译时英文原文的 git blob 哈希（前 12 位），用来发现上游更新了原文、译文没跟上的情况。
// 中文部分可以直接使用英文原文里定义的引用式链接（[文字][label]）。
// 基线提交记录在 zh-tools/BASE 里，由 `zh.mjs migrate` 修改。
import { execFileSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const BASE = fs.readFileSync(path.join(ROOT, 'zh-tools', 'BASE'), 'utf8').trim();
const SRC = path.join(ROOT, 'zh-tools', 'markdown.zh.txt');
const STALE_NOTE = '> **注意：** 英文原文已更新，下面的中文译文还没有同步，内容以折叠的英文原文为准。';
// 有意不翻译的 Markdown 文件
const SKIP = new Set([
  'CHANGELOG.md', // 提交标题和人名列表，上游每次发布都会改
  '.github/PULL_REQUEST_TEMPLATE.md',
  'widget/tests/fixtures/slow-incremental-markdown.md', // 测试输入
  'examples/markdown/overview.md', // 被 include_str! 编译进示例程序
]);
const git = (...args) => execFileSync('git', args, { cwd: ROOT, encoding: 'utf8', maxBuffer: 1 << 28, stdio: ['ignore', 'pipe', 'pipe'] });

const cmd = process.argv[2];
if (cmd !== 'apply' && cmd !== 'check') {
  console.log('用法: node zh-tools/md.mjs apply | check');
  process.exit(2);
}

const docs = [];
let cur = null;
for (const line of fs.readFileSync(SRC, 'utf8').replace(/\r\n/g, '\n').split('\n')) {
  const m = line.match(/^@@@ (\S+)(?:\s+([0-9a-f]{7,40}))?\s*$/);
  if (m) docs.push((cur = { file: m[1], hash: m[2] ?? '', zh: [] }));
  else if (cur) cur.zh.push(line);
}

const problems = [], stale = [];
let written = 0;
for (const d of docs) {
  let now;
  try {
    now = git('rev-parse', '--verify', `${BASE}:${d.file}`).trim();
  } catch {
    problems.push(`上游已删除这个文件：${d.file}（请从 markdown.zh.txt 里删掉它的译文，并 git rm 这个文件）`);
    continue;
  }
  const en = git('cat-file', 'blob', now).replace(/\r\n/g, '\n').replace(/\n*$/, '');
  const fresh = d.hash !== '' && now.startsWith(d.hash);
  if (!fresh) stale.push({ ...d, now: now.slice(0, 12) });
  const out = `${fresh ? '' : `${STALE_NOTE}\n\n`}${d.zh.join('\n').trim()}\n\n<details>\n<summary>显示英文原文</summary>\n\n${en}\n\n</details>\n`;
  const abs = path.join(ROOT, d.file);
  const text = fs.existsSync(abs) ? fs.readFileSync(abs, 'utf8') : null;
  if (cmd === 'apply') {
    const next = out.replace(/\n/g, text?.includes('\r\n') ? '\r\n' : '\n');
    if (text === next) continue;
    fs.mkdirSync(path.dirname(abs), { recursive: true });
    fs.writeFileSync(abs, next);
    written++;
  } else {
    const lf = (text ?? '').replace(/\r\n/g, '\n');
    if (lf !== out) problems.push(`不一致（需要重新 apply）：${d.file}`);
    if (!lf.includes(en)) problems.push(`英文原文丢失：${d.file}`);
  }
}

for (const p of problems) console.log(p);
for (const s of stale) {
  console.log(`英文原文已更新，译文需要同步：${s.file}`);
  if (s.hash) console.log(`  查看原文的改动：git diff ${s.hash} ${s.now}`);
  console.log(`  译文更新后，把 zh-tools/markdown.zh.txt 里对应的标题行改为：@@@ ${s.file} ${s.now}`);
}
if (cmd === 'check') {
  const known = new Set(docs.map((d) => d.file));
  const missing = git('ls-tree', '-r', '--name-only', '-z', BASE).split('\0').filter((f) => f.endsWith('.md') && !known.has(f) && !SKIP.has(f));
  for (const f of missing) console.log(`还没有翻译的 Markdown 文件：${f}（不打算翻译的话，加入 md.mjs 的 SKIP 列表）`);
}
if (cmd === 'apply') console.log(`已生成 ${docs.length} 个 Markdown 文件，其中 ${written} 个有变化（基线 ${BASE.slice(0, 9)}）`);
else if (problems.length) console.log(`\n发现 ${problems.length} 个问题`);
else console.log(`校验通过：${docs.length} 个 Markdown 文件与生成结果一致，英文原文完整保留${stale.length ? `；其中 ${stale.length} 个的译文需要同步` : ''}`);
process.exit(problems.length ? 1 : 0);
