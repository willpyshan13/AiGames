# 贪吃蛇像素街机资源

用户已否定山水风格，选择像素街机，并进一步要求游戏区域更大、避免很快结束；最终明确确认 `designs/five-game-visual-fixes-v1/snake-pixel-large-approved.png`。采用18×24大棋盘（原12×12的3倍格数）、顶部小号开始/暂停、底部横排方向键，以及用户选定的慢速穿墙玩法。

`pixel-large-approved-v1.png`：确认稿原图，运行时采样标题、边框、操作按钮、蛇和红苹果；棋盘、分数、位置、按钮状态实时绘制。

`pixel-gold-apple-v1.png`：内置 image_gen.imagegen 生成的现有三分金果变体，透明图片，取样区域 `(407,380)-(849,897)`。未使用 CLI 或图像脚本编辑。

## 最终大棋盘提示词

Edit target: pixel arcade Snake design. KEEP the exact colorful pixel arcade art style, navy checker background, golden title and frame, vivid green pixel snake, red apple and blue beveled keys, but completely revise LAYOUT to give much MORE PLAYABLE AREA. Portrait 9:16. The old square-board and huge start-button layout is discarded. New layout MUST follow: very compact TWO-LINE header occupying ONLY top 10% of screen: small snake logo and title 贪吃蛇 at left, 重来 and 游戏库 small buttons right; second slim line has 本局 120 and 最佳 567 counters plus a small orange 开始 button. NO large start button anywhere below board. Main PLAYABLE BOARD is a TALL RECTANGLE, almost FULL screen width (96%), starts at 11% height and ends at 88% height, containing exactly 18 columns and 26 rows of SQUARE dark teal checker cells. Thin golden border only, no thick decorative side panels. This board must occupy around THREE QUARTERS of the entire screen surface and is the dominant feature. Draw a cheerful lime green 9-segment pixel snake with corners and expressive eyes aligned to this bigger field, one red pixel apple, no obstacles. At bottom, ONLY a single compact horizontal ROW of four blue beveled arrow buttons in order LEFT, UP, DOWN, RIGHT, all four visible; row between 90% and 97% height, do NOT use diamond layout. Tiny footer 碰边穿行 · 慢速起步 just below keys. No large logo, no tall score cards, no separate decorative bottom zone, no extra features, no mode selector, no phone frame. The approved play rules are relaxed pace and wrap-around edges, collisions with self only. This is revised design proposal only, not an implemented screenshot.

## 金果变体提示词

Extract the ONE red pixel apple sprite from the large-board Snake design and create its GOLDEN APPLE variant for the already existing +3 fruit reward. Preserve exact apple silhouette, tiny green leaf/stem, consistent chunky pixel scale, black outline, white square highlight and shading treatment. Change only the apple body from red to golden yellow with amber shadows. One small sprite centered, fully visible, generous transparent padding, no board, no text, no UI, genuine transparent background. Do not redesign shape.

