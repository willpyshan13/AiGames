# 搭建解谜 v2 运行素材

2026-10-29。工具：内置 image_gen，未使用 CLI/API 回退。依据已展示并由用户“你给我设计的这样，能给我按照你设计的做好吗”确认的 `designs/building-puzzle-v2/` 两张稿件拆分。

运行素材均已复制至本目录：

| 文件 | 内容 | 生成源文件 |
|---|---|---|
| cloud-scene.png | 固定云海和三个完整岛体、两块固定桥锚点 | exec-c549d762-fb0e-4fb6-9f4e-d5431d525d3e.png |
| island-scene.png | 固定森林和岛体，去掉三桥、机器人、门、机关、星星及UI | exec-bdaf78ee-5ec8-4791-a512-d4105e5730b0.png |
| actors.png | 桥块、星星、门开关两态、机关两态的透明图集；采用桥块修订版 | exec-905a84f3-f1fe-45a0-aa77-6b7967387ce1.png |
| frames.png | 四个空白木质界面框，透明背景 | exec-d1540842-ba0b-4dba-bb7f-2a68d59e2864.png |

默认生成源目录：`$CODEX_HOME/generated_images/01a1821b-afdf-7541-9e47-0406a0351373/`。初版附件图集 `exec-feaa48ed-5b0e-4468-8c31-a917ec4b0cc3.png` 的桥块有高立柱，已修订为稿件中的低矮实心桥块，未交付为最终运行图。

固定场景不含动态UI、角色、目标、材料选择、可放置桥块或可选收集物。桥梁和机器人位置、门的开关、机关亮灭、星晶是否收集、16槽、计数等均由实际关卡状态驱动。固定云桥锚点属于地图，不消耗库存。图集首格生成的机器人变体未采用；保留此前 `redesign-v1/tiles.png` 中已使用的黄色方头白胸机器人，按确认稿的小尺寸绘制。中文使用运行字体；方向、锁、灯泡、重复、播放等图标用原生UI图形绘制。

图集切片及UI位置见 `src/builder_puzzle_ui.rs`，没有在代码中使用 `.codex/generated_images` 路径。

## 云桥固定场景

```text
Use case: precise-object-edit. Extract the STATIC SCENE layer from this approved game screen. Preserve exact canvas proportions and ALL static scenery in exact existing positions: sky, clouds, distant islands, three complete foreground grass/rock islands, trees, shrubs, flowers, rocks, the two existing wood bridge ANCHORS at either end of the central direct route. Preserve the exact organically tapered rock silhouettes, grass texture, camera, lighting, all scene dimensions. Remove ALL UI: header, cards, text, material rail, bottom toolbar and every control, restoring the underlying sky/cloud scenery seamlessly. Also remove ONLY these dynamic gameplay items: yellow robot, gold star, cyan crystal, wind swirl, ALL cyan ghost bridge outlines/empty cubes on all routes. Fill them cleanly with their original grass surface or cloud/sky. Do not add terrain where the empty bridge slots were. Do not relocate, shrink, enlarge or redraw foreground island silhouettes. This is a gameplay scene layer with STATIC terrain ONLY, no characters, gates, collectibles, dynamic bridges, text, borders or controls. Full original 16:9 canvas. Exact visual fidelity to reference, not a reimagining.
```

## 建造师固定场景

```text
Use case: precise-object-edit. Extract the STATIC SCENE layer from this approved 小小建造师 screen. Preserve the exact canvas proportions and all STATIC landscape, woodland valley, waterfalls, distant islands, every foreground grass-topped tapered rock island, shrubs, rocks, tree, fence and vines at the precise original positions and sizes. Preserve natural narrow tapering cliffs, exact sunlight and camera. Remove ALL UI and panels: top header, upper-left objective card, right settings frame, bottom workbench, every text, icon and control; seamlessly reconstruct landscape underneath them. Remove ALL dynamic gameplay objects: yellow robot, golden goal star, brass circular switch, teal gate including pillars and key, floating connecting key and curve, and ALL THREE honey wood BRIDGE BLOCKS joining the islands (the pair near the start and the single near switch). Restore grass beneath robot, goal, gate and switch. Where bridge blocks were, show EMPTY air between the unchanged grass banks, do not fill gaps with land. Fixed islands and their grass surfaces remain exactly in place. This must be a reusable static level scene with no UI, text, characters, switches, gates, bridges, goal or collectibles. Original 16:9 full canvas, not a redesigned scene.
```

## 动态附件图集初版

```text
Use case: background-extraction / compositing. Create production transparent sprite atlas matching the supplied approved game screen EXACTLY. FOUR columns by TWO rows, eight equal square cells with clear transparent padding, genuine transparent alpha, no labels or backdrop. Every object is orthographic three-quarter matching the approved screen, warm sunlight, crisp toy materials. ROW1 left to right: 1 exact SMALL yellow CUBOID robot with cyan rectangular eyes and white chest and little feet, isolated full body, no ground; 2 exact golden five-point goal star, no pedestal; 3 one single honey wood BRIDGE BLOCK with four tiny screws and flat plank top, a low cube, no crate cross braces; 4 one gray STONE BRIDGE BLOCK matching dimensions and projection of wood block. ROW2: 5 the exact closed jade-teal stone gate with small brass key emblem and two short stone pillars, no ground; 6 same exact gate OPEN with a clearly empty center to walk through, same pillars and same width; 7 the round brass floor pressure switch UNPRESSED, dark muted key emblem; 8 same switch PRESSED glowing warm gold-green, exactly same size and camera. Preserve reference's forms, proportions and appearance; do not invent new robot or gate styling. No grass island pieces, no cast shadows outside each object, no extra props, no text. Keep all objects fully inside their cells.
```

## 木框图集

```text
Use case: background-extraction. Extract ONLY the four large empty wooden UI FRAMES from the provided approved screen, preserving their EXACT full-canvas positions, dimensions, rounded contours, honey wood rim, ivory fill, tiny leaf trims and highlights: (1) top navigation bar, (2) upper-left objective card, (3) right execution panel, (4) bottom full-width workbench. Delete ALL content inside them: ALL text, Chinese labels, stars, glyphs, buttons, command cards, rows, separators, counters, robot and illustrations. Interiors completely blank pale ivory. Delete all landscape and everything outside the four frames, leave genuine TRANSPARENT alpha outside frames. No new content, no scenic remnants inside frames. Keep original 16:9 canvas, do not repack frames. The original beautiful empty frames only, no labels, checkerboard or white background.
```

## 实心桥块修订

```text
Use case: precise-object-edit. Production atlas correction. Image 1 is the EDIT TARGET 4x2 transparent sprite atlas. Image 2 is the approved visual style reference. Change ONLY row 1 column 3 (wood bridge block) and row 1 column 4 (stone bridge block). Match Image 2's low solid CUBOID bridge blocks exactly: flat honey plank top with four small flush metal screws, short solid wood sides; gray stone equivalent. REMOVE the tall vertical corner POSTS and rail-like edges currently present; these are solid low bridge cubes, not fences, pallets or wooden doorframes. The top surface must slope down to the right matching the actual bridges in Image 2. Keep each object's same cell location and approximate alpha bounds and full atlas size. Preserve all other SIX atlas objects pixel-identically, including robot, star, both gates and both switches. Preserve genuine transparent background; no labels, extra objects or scenery. Exact approved material and small screws, no rail posts or crate braces.
```
