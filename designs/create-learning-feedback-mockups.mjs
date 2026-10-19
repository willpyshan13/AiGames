import fs from 'node:fs';
import path from 'node:path';

const here = path.dirname(new URL(import.meta.url).pathname);
const modes = {
  fire: {
    hint: '需要能从地面通到窗台的工具。',
    wrong: '气球到不了二楼窗台，再想想！',
    card: [127, 723, 469, 163],
    rim: ['#e7a553', '#a13f22', '#5c1d12'],
    trim: '#f1c06b', ink: '#6b260e', inside: ['#fffce9', '#ffe5af'],
  },
  detective: {
    hint: '走路会在地上留下它。',
    wrong: '糖果不是方向线索，再找找！',
    card: [99, 741, 458, 150],
    rim: ['#e7b964', '#70366d', '#32133f'],
    trim: '#e6bd65', ink: '#492045', inside: ['#fff9e9', '#f1d5ad'],
  },
  farm: {
    hint: '天空中暖暖亮亮的是什么？',
    wrong: '冰块会让小苗变冷，再试试！',
    card: [83, 732, 475, 157],
    rim: ['#8ce9ff', '#2774dc', '#123a9a'],
    trim: '#8bd5fb', ink: '#153b82', inside: ['#fffef4', '#dff5ff'],
  },
};

for (const [mode, theme] of Object.entries(modes)) {
  const original = fs.readFileSync(path.join(here, `../docs/screenshots/${mode}-live-v2.png`)).toString('base64');
  for (const state of ['hint', 'wrong']) {
    const [x, y, w, h] = theme.card;
    const title = state === 'hint' ? '小提示' : '再试一次';
    const emblem = state === 'hint' ? '✦' : '×';
    const border = state === 'wrong' ? `
      <rect x="${x}" y="${y}" width="${w}" height="${h}" rx="29" fill="none" stroke="#fff0a5" stroke-width="14" filter="url(#cardGlow)"/>
      <rect x="${x}" y="${y}" width="${w}" height="${h}" rx="29" fill="none" stroke="#f7532a" stroke-width="8"/>
      <circle cx="${x+w-15}" cy="${y+13}" r="34" fill="#8e2316" opacity="0.5"/>
      <circle cx="${x+w-17}" cy="${y+9}" r="31" fill="url(#wrongBadge)" stroke="#fff1b9" stroke-width="5"/>
      <text x="${x+w-17}" y="${y+24}" text-anchor="middle" font-size="47" font-weight="1000" fill="#fff8e8">×</text>` : '';
    const svg = `<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="1672" height="941" viewBox="0 0 1672 941">
  <defs>
    <linearGradient id="rim" x2="0.7" y2="1"><stop stop-color="${theme.rim[0]}"/><stop offset="0.48" stop-color="${theme.rim[1]}"/><stop offset="1" stop-color="${theme.rim[2]}"/></linearGradient>
    <linearGradient id="inside" x2="0.5" y2="1"><stop stop-color="${theme.inside[0]}"/><stop offset="1" stop-color="${theme.inside[1]}"/></linearGradient>
    <linearGradient id="emblem" x2="0.3" y2="1"><stop stop-color="#ffe965"/><stop offset="1" stop-color="#e98a14"/></linearGradient>
    <linearGradient id="wrongBadge" x2="0.3" y2="1"><stop stop-color="#ff9e3e"/><stop offset="1" stop-color="#d92420"/></linearGradient>
    <filter id="shadow" x="-0.1" y="-0.2" width="1.3" height="1.6"><feDropShadow dx="0" dy="11" stdDeviation="9" flood-color="#24110d" flood-opacity="0.55"/></filter>
    <filter id="cardGlow" x="-0.1" y="-0.2" width="1.3" height="1.4"><feGaussianBlur stdDeviation="10"/></filter>
  </defs>
  <image x="0" y="0" width="1672" height="941" preserveAspectRatio="none" xlink:href="data:image/png;base64,${original}"/>
  <g filter="url(#shadow)">
    <rect x="650" y="24" width="548" height="104" rx="25" fill="url(#rim)"/>
    <rect x="659" y="33" width="530" height="86" rx="19" fill="url(#inside)" stroke="${theme.trim}" stroke-width="4"/>
  </g>
  <circle cx="672" cy="77" r="4" fill="#fff0b5"/><circle cx="1176" cy="77" r="4" fill="#fff0b5"/>
  <circle cx="704" cy="76" r="31" fill="url(#emblem)" stroke="#fff1be" stroke-width="5"/>
  <text x="704" y="91" text-anchor="middle" font-family="PingFang SC, Hiragino Sans GB, sans-serif" font-size="44" font-weight="1000" fill="#fffaf0">${emblem}</text>
  <text x="754" y="73" font-family="PingFang SC, Hiragino Sans GB, sans-serif" font-size="21" font-weight="1000" fill="${theme.ink}">${title}</text>
  <text x="754" y="102" font-family="PingFang SC, Hiragino Sans GB, sans-serif" font-size="20" font-weight="900" fill="${theme.ink}">${theme[state]}</text>
  ${border}
</svg>`;
    fs.writeFileSync(path.join(here, `learning-ui-${mode}-${state}-precise-v2.svg`), svg);
  }
}
