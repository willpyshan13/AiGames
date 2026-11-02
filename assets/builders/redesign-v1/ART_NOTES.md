# 搭建游戏运行素材 v1

新增 `scenery.png`：2172×724透明图集，灌木、岩石、短围栏、藤蔓四项，使用内置 image_gen 生成。参考图：`designs/building-ui-redesign-v1/island-build.png`。源文件：`exec-468845a9-642b-4a43-8c08-c72dbd0f0600.png`，源目录同下。

完整提示词：

```text
Use case: compositing / stylized-concept. Production transparent scenery atlas matching the supplied already-approved 小小建造师 design reference. Create FOUR equal square cells in one horizontal row, isolated on true transparent alpha, no text, UI, platforms or ground shadows. Cell 1: one low rounded bright green leafy shrub dotted with three tiny white daisies, honey warm sunlight, exact miniature voxel/soft toy style of reference. Cell 2: a small cluster of three pale gray warm faceted rounded rocks, low profile, no grass platform. Cell 3: short warm honey wood garden fence, exactly TWO thick short posts and one horizontal rail between, with tiny green leaves wrapping its ends, orthographic view looking down, rail descends to the right with slope 0.4; preserve reference's little wooden island edge fences. Cell 4: thin short hanging green vine with little oval leaves and three tiny white blossoms, vertical cascade, transparent all around. Similar visual size and 10% margin within every cell, neat 4-column atlas. Crisp detailed but readable small. Use approved warm forest palette, no new visual direction. No background, no border, no labels. Objects only, true alpha.
```

日期：2026-10-29。使用内置 image_gen 工具，以 `designs/building-ui-redesign-v1/` 已展示的设计稿为参考，生成运行所需的分层素材。未使用 CLI/API 回退。

界面文字、按钮、格子、机器人位置、收集状态均由游戏实时绘制；背景不包含静态游戏局面。文件均已保存在本目录。用户已明确选择“按设计稿缩小岛屿，调整可搭建区域”，运行场景现已改为每侧3×5格的紧凑岛屿，并补入稿件中的灌木、岩石、围栏与藤蔓。

透明图集 `tiles.png` 为 1774×887，4 列×2 行，依次为草地、木头、石头、水晶建材、岩壁、树、机器人、收集星晶；`frames.png` 为 1672×941，保留透明背景。切片坐标见 `src/builder_redesign.rs`。

## cloud-background.png

生成源文件：`exec-cf6b7f37-5c5a-46b4-aa3a-b45b9fb176a1.png`。源目录：`$CODEX_HOME/generated_images/01a1821b-afdf-7541-9e47-0406a0351373/`。

完整提示词：

```text
Use case: precise-object-edit / background-extraction. Production background layer for an already approved game design. Edit the supplied 云桥工程师 screenshot: remove ALL foreground gameplay islands, bridge blocks, bridge guides, robot, star, collectible crystals, every HUD, all panels, toolbar, text, buttons, material rail and bottom dock. Fill removed areas seamlessly using ONLY the existing soft blue sky, cream clouds and distant tiny floating islands. Preserve the exact existing sky/cloud color palette, lighting, depth, distant floating-island style and spatial arrangement. Large calm open middle area for live game objects. Full 16:9 canvas, high resolution. No text, no borders, no new focal objects, no foreground platforms. This must be a reusable background, not a flattened game screen.
```

## island-background.png

生成源文件：`exec-aa0553cf-079c-4040-a40e-e25ed667e669.png`。源目录：`$CODEX_HOME/generated_images/01a1821b-afdf-7541-9e47-0406a0351373/`。

完整提示词：

```text
Use case: precise-object-edit / background-extraction. Production background layer for the approved 小小建造师 screenshot. Remove ALL foreground gameplay islands, bridge guide cells, robot, star, crystals, all UI chrome, material chest, workbench, toolbar, labels, arrows and buttons. Seamlessly restore ONLY the same softly focused woodland valley with distant rock islands, soft waterfalls, forest canopy, morning warm sun and mist. Preserve the existing scene's palette, painting, spatial composition and light; do not redesign the environment. Keep the central 75% calm and uncluttered for the live buildable scene, matching the screenshot's depth of field. Full 16:9 image, no words, no borders, no foreground paths/platforms or gameplay objects.
```

## tiles.png

生成源文件：`exec-57c9797e-5c80-4f26-9143-ba4ee66304dd.png`。源目录：`$CODEX_HOME/generated_images/01a1821b-afdf-7541-9e47-0406a0351373/`。

完整提示词：

```text
Use case: compositing / stylized-concept. Create production transparent sprites for the approved miniature voxel game screenshot. Match the reference's exact toy-model grass, warm honey wood with small metal screws, layered ochre-gray cliff stone, yellow cuboid robot with cyan eyes, and cyan crystal. Output a neat 4 COLUMN by 2 ROW sprite atlas on a genuinely TRANSPARENT background. Exactly EIGHT equal square cells, each object isolated with an even 8% transparent margin, no shadows outside objects, no labels, no UI, no grids. Row1 left to right: 1 a seamless grass-topped cube with a broad diamond top and short dirt-and-small-stone sides, tiny restrained grass at the edge, no rocks/tree sitting on its top; 2 a cubic wood-plank bridge block with four tiny screw heads on its top corners, no diagonal crate braces; 3 a pale rounded stone block; 4 clear cyan crystal building cube. Row2 left to right: 5 a grassless layered ochre-and-gray cliff cube, same exact dimensions/projection as row1 cubes for stacking beneath grass blocks; 6 one small leafy tree with short brown trunk and clustered green canopy, matching the screenshot; 7 the exact friendly small yellow CUBOID robot from the screenshot, full body, three-quarter view with dark flat rectangular face and two cyan rectangular eyes, white torso, small dark feet, NOT a round-headed new character; 8 one small cyan collectible faceted crystal, floating upright, no pedestal. All first five cubes use consistent orthographic isometric view with equal x/z axes and a broad diamond top face; face normals/edges consistent across cubes so tiles can join in a live grid. No perspective convergence, no cast ground shadows, no scenic backdrop. Crisp detailed but readable at small size. True transparent alpha, not white/checkerboard. Preserve approved art style, do not invent a new one.
```

## frames.png

生成源文件：`exec-436adc23-495d-4559-82fe-ec2638acf0f9.png`。源目录：`$CODEX_HOME/generated_images/01a1821b-afdf-7541-9e47-0406a0351373/`。

完整提示词：

```text
Use case: precise-object-edit / background-extraction. Production UI FRAME atlas extracted from the APPROVED 小小建造师 screenshot.
Preserve the exact original canvas size, 16:9 proportions and positions. Keep ONLY these FOUR empty outer interface frames with their existing warm honey wood rims, pale ivory interiors, tiny leaf trim, rounded corners and soft edge highlights: (1) full-width top navigation frame, (2) upper-left objective frame, (3) right-side material-chest outer frame, and (4) full-width bottom workbench outer frame.
REMOVE ALL text, labels, counters, glyphs, logos, stars, cube icons, card dividers, buttons, switches, robot, islands, scenery, clouds, foliage outside the four frames and every other visible object. Completely clear all interiors of the four frames to the same clean blank ivory, keeping only the outer wood/leaf framing details. Fill every area OUTSIDE the four retained frames with TRUE TRANSPARENT ALPHA. Do not leave the original landscape background behind. Do not draw any replacement buttons or content. The four frames remain at their precise original locations and sizes, not repacked or rearranged. Exact same approved wood material, no new flourishes, no white/checkerboard background. This is a reusable transparent UI frame atlas to put live game controls on top.
```
