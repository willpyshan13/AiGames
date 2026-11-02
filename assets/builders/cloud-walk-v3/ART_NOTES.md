# Cloud bridge v3 production sprites

Approved 2026-10-29 after displaying `designs/cloudbridge-map-variety-v3/six-maps.png` and `manual-crossing.png`; these are layout/control diagrams, retaining the previously approved toy grass/rock/cloud style.

Generated with the built-in imagegen tool (`image_gen__imagegen`), native image editing, transparent background. Runtime cropping/scaling uses Bevy ImageNode; original generated files are preserved here without pixel edits. The clean sky and yellow robot reuse the existing v1 assets. `maps.json` mirrors the approved map definitions.

## island.png

Generated source: `exec-c09c7cae-86aa-4cc1-8b8e-bd4ff829ed24.png`, 1254 × 1254. Reference images: approved toy island style and `designs/cloudbridge-map-variety-v3/production/island-guide.png` geometry.

Exact submitted prompt:

> Use case: compositing / stylized-concept. Production transparent single island sprite for the confirmed 云桥工程师 game. Image1 is STYLE ONLY: preserve its beautiful sunlit miniature grass, tiny white flowers, ochre and pale gray organically tapered rock columns, hanging vines, soft realistic toy materials. Image2 is the EXACT GEOMETRY GUIDE on an 800x800 canvas. Render ONE isolated island matching that guide's top surface position and footprint: top diamond corners precisely at (400,102), (730,260), (400,418), (70,260). The top diamond center is (400,260), its width660 and height316; maintain this 2.083:1 orthographic ground projection. World height is constant, the entire diamond is one FLAT grass surface, not a dome or stairs. The island rock mass tapers naturally below the top down toward about (400,730), with uneven rounded rock pillars, natural silhouette, NOT a square slab or uniform cube wall. Paint vivid short green grass over the guide's entire diamond, tiny subtle flowers and restrained foliage along its rim, subtle 3-by-3 grass cell divisions optional. Walkable top must stay clear: no trees, rocks, props or tall shrubs covering it. NO characters, bridges, bridge anchors, goals, crystals, UI, text, geometric guide outlines or sky. Preserve exact top corners and ground anchor from geometry guide, while giving it the style of Image1. ORIGINAL800x800 square proportions, true TRANSPARENT alpha around the island. No cast shadow outside the object. The guide background must be fully removed.

## tiles.png

Generated source: `exec-9acdfccf-21c6-49af-b003-cf01ab58f2be.png`, 1774 × 887. Reference: approved toy island style. Left wood, right stone; both low bridge decks.

Exact submitted prompt:

> Use case: compositing / stylized-concept. Production transparent atlas with exactly TWO side-by-side isolated low bridge tiles, wood LEFT and stone RIGHT, each in its own equal square cell, 1200x600 canvas. Match the supplied approved toy-island art style, golden sunlight and restrained flush screws. All tiles use one exact orthographic isometric projection: top diamond width500px and height240px (2.083:1); top vertices of LEFT tile are (300,145),(550,265),(300,385),(50,265); RIGHT tile translated +600px. Add only a short 45px solid side thickness beneath each top face; no tall corner posts or fences. Keep top diamond geometry exact, flat and square in world space so many tiles tessellate without gaps. LEFT: honey wood plank bridge block, little wood grain and four small flush metal screws in its top corners. RIGHT: pale rounded gray stone bridge block, same exact dimensions, light stone seams. Ground top center at y265, not perspective convergence or a tilted random camera. No background, labels, UI, shadows outside object, grass, rails, tall supports or extra elements. Entire surrounding canvas genuine transparent alpha. Detailed readable material from reference. The actual outer alpha bounds of each tile should hug the requested diamond and its short side thickness.

## Ground alignment

Island grass caps, bridge cells, wind cells, input hit testing and actor feet all use one world-grid projection. The grassy rim has an organic silhouette; walkable cells and bridge boundaries remain regular. No baked foreground map is used. Each authored map is fitted to the same central play area without changing its topology.
