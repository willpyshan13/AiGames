# 生成提示词

内置 image_gen 模式。每项生成单独一张效果图，未调用 CLI/API 回退路径。

## building_ui_cloud_build_prompt_v1

```text
Use case: ui-mockup, alternate state of the same game interface.
Input image 1 is the authoritative NEW Cloud Bridge design. Keep its exact visual identity, 16:9 full-screen format, same sky and clouds, same rounded toy island models, same small yellow robot, same grass/wood/stone material thumbnails, same six-stage top toolbar, same typography sizes and chrome. Produce the BUILD MODE screen for this same interface, not a redesign or another concept.
Keep top title 云桥工程师, progress 1 2 3 [4] 5 home, 第4关·云桥谷 and 怎么玩. Keep objective 星晶0/2 → 抵达终点.
In this alternate build state the bottom program dock is collapsed into a shallow warm-white rounded workbench occupying only the bottom 14% of the screen; this gives the main diorama MORE room. Preserve the two islands and the same partially built straight wood bridge, but frame them completely within the open main area so neither island is cut by panels.
Important scene correction: exactly TWO cyan collectible crystals must float immediately ABOVE the remaining translucent bridge guide cells at the walking-plane height, NOT below the bridge or down in the clouds. The golden star remains on the destination island. One bridge cell is lightly highlighted for placing a wood block. No new paths/platforms or characters.
Right toolbox: same three vertical material cards 木头 (selected blue border and check), 草地, 石头. At its base same two-segment 搭建 / 编程 selector but 搭建 is now blue selected. All cards comfortably fit above the bottom workbench.
Bottom workbench: heading 搭建云桥 at left. A simple instructional sentence 选择材料，再点格子搭桥. Three clearly distinct comfortably sized icon+text buttons 拆除, 撤销, 视角回正 in the middle. A prominent blue 编程 → button at far right, fully visible. No instruction sequence or execution buttons in this BUILD state, no redundant run button, no currency/timer/score additions. Premium playful 3D toy presentation, translucent ivory surfaces and blue accents, large legible Chinese text, same art direction and hierarchy as reference, crisp clean safe margins. No outside title, no device frame or design annotations.
```

## building_ui_island_build_prompt_v1

```text
Use case: ui-mockup / stylized-concept.
Asset type: complete landscape 16:9 reviewable game screen for 小小建造师 (the existing 磁力小岛 mode), BUILDING state. Input image 1 is the current game screen for gameplay/layout context. Input image 2 is the NEW Cloud Bridge art direction to establish a coherent family, but give this builder a different warm green and natural wood identity.
Primary request: substantially improve the game world, interface composition, material cards and control hierarchy. A premium miniature construction garden made of hand-crafted rounded voxel blocks, lush but restrained grass edging, warm layered clay and rock cliff sides, small wooden bridge segments and translucent square placement guides. Friendly small yellow cuboid robot with dark face and two cyan eyes, golden star on the destination island. Keep the actual two-island bridge-puzzle idea with a short straight gap; no extra buildings, no extra branching routes, no invented game mechanics. Material cube icons must look like the actual scene's blocks.
Art direction: a sunlit magical woodland valley seen as a refined 3D toy diorama. Calm desaturated forest canopy and distant soft waterfalls far behind the playable islands, very subtle atmospheric background, no busy temple wallpaper. Main two grassy buildable islands occupy the middle left 74% of screen; clearly delineated square block cells and a clear bridge gap. Entire playable islands visible, robot and target clearly separated. Rich tactile detail and soft contact shadows, not blurry, not generic low-poly green rectangles. Warm cream, muted jade green, honey wood, soft amber accents, deep green text.
Header: slim unified ivory-and-wood toolbar across top. Left back arrow then cube emblem and title exactly 小小建造师; a small subtitle 磁力小岛. Center shows 第4关 · 云桥谷 and six small progress markers (4 highlighted). Right three earned-star outlines, small 怎么玩 icon and an ellipsis menu. Put secondary existing profile/save/level settings behind this ellipsis concept; do not crowd main screen with them.
Below header at upper-left, compact objective card with two lines 本关目标 and 收集星晶，再到达星星. Include 星晶 0/2. Do NOT cover the robot or bridge. Exactly two cyan crystals float just above the bridge guide tiles, at the same walkable path height, never far below the bridge.
Right side a compact tool chest ~18% width, rounded light ivory panel with a thin warm-wood rim. Header 积木盒. Four material cards arranged a clean TWO-BY-TWO grid: 草地, 木头, 石头, 水晶. Wood selected with a jade-green outline and small check. Each card uses one attractive 3D cube and one legible concise Chinese label, not oversized pasted art.
Bottom shallow workbench ~15% screen height, warm ivory with subtle wood trim: left label 搭建模式 and tiny status 已自动保存; next a row of four large icon/text buttons 搭建 (green selected), 拆除, 撤销, 视角回正; far right a large jade-green 编程 → action with safe padding. A short hint 选材料，点格子放置 may fit unobtrusively above buttons, no verbose instructions. All controls fit without scrolling and have ample touch targets. The main scene dominates rather than UI panels.
This is one full 16:9 proposed gameplay screenshot, not a collage, no device outline, no external labels. Preserve existing game's mechanics, do not add coins, timer, experience bar, market, new tools or new quests. Crisp professional Chinese typography and consistent icon treatment; sophisticated playful children's educational game, not a generic business dashboard.
```

## building_ui_cloud_program_prompt_v2

```text
Use case: precise-object-edit / ui-mockup.
Input image 1 is the programming-mode proposal and is the edit target. Input image 2 is the corrected building-mode proposal, a supporting reference for crystal placement only.
Make ONE narrowly scoped correction to input image 1: move the TWO cyan collectible crystals out of the clouds below the bridge and place them immediately above the remaining two translucent bridge guide cells, at the same standing/walking elevation as the wood bridge and robot. The crystals must clearly be collectible along the straight bridge route, as in image 2. Remove their old instances entirely; exactly two remain. Do NOT add a platform or alter the gameplay.
Preserve everything else in input image 1 as closely as possible: 16:9 canvas, complete composition, two island sizes/positions, yellow robot, goal star, sky, materials palette, six-node header, 5/16 instruction counter, all sixteen numbered slots in two rows of eight, first five filled instruction cards, seven command palette buttons, green 运行 button. All Chinese labels and numbers remain verbatim. Same premium toy diorama style, same colors, typography and UI geometry. No other redesign.
```

## building_ui_island_program_prompt_v1

```text
Use case: ui-mockup, alternate interaction state.
Input image 1 is the authoritative NEW 小小建造师 BUILD-mode design. Create its PROGRAMMING-mode screen as part of the same proposal. Maintain the same 16:9 full-screen canvas, exact title 小小建造师 with small 磁力小岛 subtitle, wood-and-ivory top toolbar, progress 第4关·云桥谷 with 4 highlighted, star outlines and help/menu buttons, same warm jade green / honey wood / ivory design language, same small yellow robot and grassy miniature islands with a clear square grid, same calm woodland valley backdrop.
Preserve the two-island bridge-puzzle layout, robot, bridge guide cells, TWO cyan crystals floating just ABOVE bridge guide cells, and goal star. Scale/reframe the entire diorama coherently to fit above the taller program dock so the world remains clearly visible and is not covered by UI. Do not add paths, new platforms, new collectibles or decorations.
Keep the compact upper-left objective 本关目标 / 收集星晶，再到达星星 / 星晶0/2. 
Programming state changes:
1. Replace the right-side 积木盒 material palette with the SAME-sized wood-rimmed ivory card titled 执行设置. Inside, well-spaced legible controls: a counter labelled 重复次数 with minus button, large 5 and plus button; a two-line setting 结束后 / 前进一步 with a GREEN ON toggle; a muted button 示范路线; two equal outlined buttons 单步 and 复位 near the base. These are existing game functions, do not add other settings. 
2. Bottom 28% of screen is a polished ivory command workbench with the same restrained wood trim. Header 我的指令, counter 2/16 and subtle 清空 button at far right.
3. Draw all SIXTEEN numbered slots as TWO ROWS OF EIGHT, clean consistent size. First two filled cards: 1 放木块 with cube icon (amber) and 2 前进 with upward arrow (blue), both with a small removal ×. All fourteen other slots light empty outlines. No hidden horizontal content.
4. Below slots, SEVEN colorful icon+label command buttons in exactly this order: 放木块, 前进, 左转, 右转, 后退, 跳跃, 停留. A large jade-green ▶ 运行 button is fixed at the far right in its own column, fully visible, never clipped. A small two-segment 搭建 / 编程 mode selector with 编程 selected can be at the top of the right settings card, not duplicate the command buttons.
Typography: consistent friendly rounded Simplified Chinese, generous sizes, exact concise labels. Scene remains the focus, large icons and easy touch targets, no elaborate interface flourishes, no device frame, no outside annotations or collage. This is the same game's alternative mode, not a new scene or theme. No coins, timers, shops, new tools, extra missions or fake success feedback.
```


