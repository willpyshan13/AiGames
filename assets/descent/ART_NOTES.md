# 勇闯地下100层美术来源

确认稿：`designs/descent-approved-v2.png`。用户在全宽修订稿之后明确回复“做”，授权按该稿实现。

使用内置 `image_gen.imagegen`（非 CLI）从确认稿拆出独立背景和透明图集。运行时通过 `ImageNode.rect` 采样图集，不把静态设计稿当作游戏画面。图集原始尺寸 1247×1261，背景 1024×1536；采样矩形保存在 `src/descent.rs` 的 `source` 函数中。

## 最终背景提示词

Edit target: approved gameplay design. Extract ONLY its full-screen underground mine background as a clean reusable portrait game background. Preserve exact same pixel-art style, navy teal rock colors, dim wooden distant walkways, amber lanterns, depth and thin edge walls. Remove EVERY foreground platform, all springs, red spikes, player, all hearts, all HUD text, pause icon, arrow buttons, and ceiling spike row. Paint the removed areas with seamlessly continuing dim distant mine background. Preserve the full-width open play shaft, thin rock walls only at extreme 2% edges. No new foreground objects. No UI, no text, no characters, no platforms. Portrait 2:3.

输出：`assets/descent/mine-background.png`。

## 最终图集提示词

Edit target: approved pixel-art game screen. Extract its gameplay art into a reusable TRANSPARENT sprite sheet with exactly 4 columns and 4 rows of equal cells, each sprite centered inside its cell, separated with generous transparent padding, no grid lines or text. Preserve the reference's original art faithfully, colors and pixel shading and shapes, do not redesign. Row 1: ordinary mossy stone platform; cracked stone platform; cyan right-arrow conveyor stone platform; red spike-topped stone platform. Row 2: green spring platform with metal coil and bottom stone foot; ceiling row of white downward spikes; filled red heart; empty outlined heart. Row 3: orange scarf helmet explorer facing right standing; same explorer facing right running pose 1; same running pose 2; same explorer falling. Row 4: circular translucent dark left arrow control with silver border; matching right arrow control; square silver-border pause icon; amber rectangular button without any text. All platforms identical footprint except spring taller. Full character bodies contained in each cell, transparent background, no backdrop, no stray shadows outside sprites, no labels. Sheet used directly as 4x4 texture atlas.

输出：`assets/descent/mine-atlas.png`。
