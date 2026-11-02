# 遗迹淘金者资源来源

用户确认稿：`designs/gold-runner-approved-v1.png`，在展示效果图后回复“做”，授权实现。

使用内置 `image_gen.imagegen` 从确认稿制作独立背景和透明图集，未使用 CLI。标题、顶栏边框、方向键和左右挖掘按钮直接从 `approved.png` 采样；动态计数与生命状态由游戏绘制。角色、砖块、梯子、绳索和黄金从透明图集的原始像素矩形采样。

## 最终背景提示词

Edit target: approved game design. Produce ONLY the clean reusable background, preserving this exact image's landscape 16:10 composition, golden sunset sky, distant stepped temple silhouettes, teal jungle ravine and central waterfall, dark sandstone decorative side walls with torches and vines, lower dark stone scenery behind controls. Remove all five FOREGROUND horizontal playable platform rows, ALL ladders, ropes, gold piles, characters, hole surrounds and HUD and buttons and text. Seamlessly fill their areas with the existing distant temple/jungle background; top band clean dark stone. Keep the large exit arch at upper right in its original exact position. Keep side walls thin, full-width playable central area unobstructed. Do not add new decoration. Exact same pixel-art colors and scale. No characters, no gold, no platforms, no controls, no text. Not a redesign.

输出：`assets/gold_runner/temple-background.png`。

## 最终图集提示词

Edit target: approved game design. Extract faithful original pixel-art gameplay assets into one transparent sprite sheet, EXACTLY FOUR columns FOUR rows of equal cells, 16 sprites total, generous transparent padding around each, no labels or grid lines. Keep all silhouettes, colors and details identical to reference, no redesign. Row1: one golden sandstone square brick; one vertical wooden ladder segment; one short horizontal golden rope segment; small pile of gold. Row2: orange-scarf brown-fedora explorer idle facing right; explorer walking facing right frame1; explorer walking facing right frame2; explorer digging to right with pickaxe. Row3: teal-blue stone guardian idle facing right; guardian walking frame1; guardian walking frame2; explorer climbing ladder facing away. Row4: translucent square silver-bordered arrow UP; matching arrow DOWN; matching arrow LEFT; matching arrow RIGHT. Entire sprites contained in cells. Flat 2D side-view, exactly same crisp pixel-art style as reference. Genuine transparent background. No extra assets or scenery or text.

输出：`assets/gold_runner/temple-atlas.png`。

`approved.png` 是已确认的原始效果图。图集实际为1263×1246；采样坐标见 `src/gold_runner.rs` 的 `gold_rect`。运行时调整采样矩形与比例，未修改原始生成图片。
