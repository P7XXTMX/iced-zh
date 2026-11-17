#!/usr/bin/env node
// Markdown 双语生成：中文译文在前，基线英文原文整体折叠在后。
//   node zh-tools/md.mjs apply   按 markdown.zh.txt 重写 Markdown 文件
//   node zh-tools/md.mjs check   校验工作区文件与生成结果一致、英文原文完整
// markdown.zh.txt 格式：`@@@ 路径` 开头，后面是该文件的完整中文译文。
// 中文部分可以直接使用英文原文里定义的引用式链接（[文字][label]）。
import { execFileSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';

const BASE = 'e5c350f27';
const ROOT = path.resolve(path.dirname(new URL(import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, '$1')), '..');
const SRC = path.join(ROOT, 'zh-tools', 'markdown.zh.txt');

const docs = new Map();
let cur = null;
for (const line of fs.readFileSync(SRC, 'utf8').replace(/\r\n/g, '\n').split('\n')) {
  const m = line.match(/^@@@ (\S+)\s*$/);
  if (m) docs.set(m[1], (cur = []));
  else if (cur) cur.push(line);
}

const cmd = process.argv[2];
let bad = 0;
for (const [file, zh] of docs) {
  const en = execFileSync('git', ['show', `${BASE}:${file}`], { cwd: ROOT, encoding: 'utf8' }).replace(/\r\n/g, '\n').replace(/\n*$/, '');
  const out = `${zh.join('\n').trim()}\n\n<details>\n<summary>显示英文原文</summary>\n\n${en}\n\n</details>\n`;
  const abs = path.join(ROOT, file);
  if (cmd === 'apply') {
    const eol = fs.readFileSync(abs, 'utf8').includes('\r\n') ? '\r\n' : '\n';
    fs.writeFileSync(abs, out.replace(/\n/g, eol));
  } else if (cmd === 'check') {
    const now = fs.readFileSync(abs, 'utf8').replace(/\r\n/g, '\n');
    if (now !== out) { console.log(`不一致（需要重新 apply）：${file}`); bad++; }
    if (!now.includes(en)) { console.log(`英文原文丢失：${file}`); bad++; }
  }
}
console.log(cmd === 'apply' ? `已重写 ${docs.size} 个 Markdown 文件` : bad ? `发现 ${bad} 个问题` : `校验通过：${docs.size} 个 Markdown 文件的英文原文完整保留`);
process.exit(bad ? 1 : 0);
