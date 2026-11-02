# 五款游戏视觉修订提案

状态：四款修订稿已获明确确认并接入。贪吃蛇原山水稿与第一张方形像素稿被用户否定，最终确认 `snake-pixel-large-approved.png`，同时选择慢速起步、碰边穿行、只在撞到自己时结束；最终稿已接入。下方 snake-phone.png 仅保留为历史设计。

| 游戏 | 修订范围 | 效果图 |
|---|---|---|
| 贪吃蛇手机端 | 延续宣纸山水与浅绿棋盘；分数压成顶部两格，正方形棋盘；开始按钮和四向键完整显示。 | snake-phone.png |
| 星球快递员 | 主画布按16:9等比居中适配20:9，两侧补星空，保留当前人物、星球、路线和操作资源的比例。 | courier-wide.png |
| 泡泡爆破王 | 生命栏由5槽改为3槽，其余沿用当前画面。 | bomber-three-hearts.png |
| 云桥工程师 | 已选指令与可添加指令分成两行，运行按钮固定在底栏右端，保留所有原有指令。 | cloudbridge-dock.png |
| 经典水果机 | 收紧右栏条目高度、间距和积分区，清空、开始、免费补充和说明完整显示。 | arcade-sidebar.png |

参考当前截图在 `output/visual-audit/`。使用内置 image_gen.imagegen 制作提案，非CLI。生成图中的数值仅演示布局，实际实现仍使用游戏实时状态。

## 最终提示词

### snake

参考：`/Users/will/project/AiGames/assets/snake/snake-reference-v1.png`

Use case: ui-mockup. Input is the existing approved LANDSCAPE snake visual design. Produce a PORTRAIT PHONE adaptation of this SAME design, 9:16 ratio, full single screen, no device frame. Preserve cream rice-paper texture, pale sage 12x12 square grid, deep forest green snake made of rounded square cells with two white eyes, red pixel apple, delicate Chinese ink-wash mountain and cottage scenery at edges, same typography family and restrained green cream palette. This is a layout adaptation, not a new theme. Top 8% title with small pixel snake logo '贪吃蛇', compact '重来' and '游戏库' buttons. Directly below, TWO compact side-by-side score capsules '本局 120' and '最佳 567'; no tall score panel. Main 12x12 square board spans 90% screen width, centered, board top at 17% and bottom at 67.6% screen height. Small 6-cell snake and red apple inside grid. Below board one wide forest-green '开始游戏' button, then FOUR comfortably large arrow buttons arranged in a compact diamond (up above left/down/right), all FULLY visible and comfortably above bottom 4% safe margin. Tiny note '方向键 / WASD · 空格暂停'. All content must fit one screen without scrolling, NO clipping. Main difference from landscape reference: score panel moves into slim top row, board remains square and controls all fit below. Do not add features or decorative objects. Professional faithful portrait mockup.

### courier

参考：`/Users/will/project/AiGames/output/visual-audit/courier/current.png`

Use case: ui-mockup. EDIT this current gameplay screenshot ONLY to demonstrate correct aspect-ratio fitting on a VERY WIDE 20:9 phone display. Output a SINGLE 20:9 landscape screen. Preserve ALL original game artwork, exact title, exact planet shapes, round helmets, character faces, ship silhouettes, route lines, panels, current counters 0/9 and 0/3, and current controls. Fit the whole original 16:9 composition UNSTRETCHED, uniformly scaled, centered in the wider screen with everything visible. Fill only the added left/right side strips with dark blue starfield smoothly matching existing space background. The added starfield strips occupy approximately 10% of total screen width on EACH side, leaving the existing whole game composition at original 16:9 aspect in central 80%. Do NOT stretch any element horizontally, do not rearrange interior elements or change their sizes relative to one another. NO new buttons, no new labels, no new planets, no in-image explanation, no phone frame. This is a constrained layout repair proposal, same existing game.

### bomber

参考：`/Users/will/project/AiGames/output/visual-audit/bomber/current.png`

Use case: ui-mockup. EDIT the supplied current game screenshot with ONE SINGLE change: the top status bar currently shows THREE filled red heart icons followed by TWO empty outlined heart icons. REMOVE only those TWO empty outlined hearts so the life display has EXACTLY THREE filled hearts total, reflecting three maximum lives. Preserve the three original filled heart icons, their size and position. Seamlessly fill removed icon areas with the existing dark navy top-bar background. Preserve every other pixel and all proportions and layout as closely as possible: top title and score, entire 17x9 board and all actors and obstacles, bottom mission panel, left joystick, right bubble action and existing text. Output single landscape 16:9 gameplay screen, same art style, no redesigned UI, no new effects, no annotations, no phone frame.

### cloudbridge

参考：`/Users/will/project/AiGames/output/visual-audit/cloudbridge/current.png`

Use case: ui-mockup. EDIT the provided cloud-bridge current gameplay screenshot. Preserve original 16:9 landscape screen, blue sky and clouds, two cubic grass islands and small yellow robot, stars, all top navigation and progress, goal panel and right material sidebar. Change ONLY the bottom cream programming dock to fix its clipped rightmost RUN button. Preserve current cream rounded panel, pastel blue/yellow/green/pink/lavender button style and dark blue-gray font. Within SAME bottom dock footprint, make a compact TWO-ROW layout: upper row is selected instruction sequence showing '+ 放木块 ×' then '↑ 前进 ×', followed by room for further selected cards; header says '编排机器人的行动' with '清空' at upper right. Lower row contains exactly SEVEN smaller available command buttons '+ 放木块', '↑ 前进', '左转', '右转', '↓ 后退', '跳跃', 'Ⅱ 停留', followed by a GREEN fixed '▶ 运行' button at the far right. ALL eight lower-row buttons must be completely visible WITH right padding; no clipping. Selected instructions are separate from available-command buttons instead of sharing one long row. Keep right material sidebar above/right as reference, no overlap with dock. No other layout or art change, no new features, no explanatory callouts, no phone frame. Proposed layout repair, not actual implemented screenshot.

### arcade

参考：`/Users/will/project/AiGames/output/visual-audit/arcade/current.png`

Use case: ui-mockup. EDIT current 16:9 landscape arcade fruit game screenshot. Preserve exact existing retro dark navy / brown wood / amber-gold style, left square fruit run-light board and central LUCKY 7 area, top title and navigation, existing fruit icon designs, all counter values. Change ONLY right sidebar vertical spacing to fit ALL controls inside the screen, because current bottom blue refill button is clipped. Keep right sidebar width and position. Reduce fruit-row heights, vertical gaps, and score-panel height moderately (roughly 15-20%) while retaining readable labels and generous touch targets. Sidebar top-to-bottom: score '积分 1000'; heading '选择水果 · 每点 +10'; five compact fruit entries '樱桃 ×3 投入0 ＋', '柠檬 ×4 投入0 ＋', '葡萄 ×5 投入0 ＋', '橙子 ×6 投入0 ＋', '幸运7 ×10 投入0 ＋'; '本次投入 0 点'; red '清空' and green '先选水果' buttons side by side; FULLY visible wide blue '免费补充 +1000' button; footer '仅使用免费虚拟积分'. Leave clear safe margin below footer, all controls visible without scrolling. No new features or style redesign. Do not shrink the entire game or alter left board. No explanatory labels or phone frame.
