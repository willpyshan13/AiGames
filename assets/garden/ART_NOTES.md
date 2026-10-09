# 花园守卫美术

使用内置图像生成工具生成，未调用外部 CLI/API。参考为本聊天中用户认可的《花园守卫》设计稿。原始输出保留在 Codex generated_images 中，本目录副本供游戏离线加载。

## 资产与最终提示词

### backdrop-v1.png

Use case: precise-object-edit. Asset type: shipped garden defense game background. Image 1 is the approved UI design and edit target. Create a clean empty playable BACKDROP preserving its beautiful warm hand-painted cartoon garden, sunlight, cottage at left, fence/right dirt path, leaves/flowers and stone framing. Wide 16:9 image. Remove ALL plants, monsters, bullets, health bars, sunshine pickups, cursor effects and ALL text/numbers/icons/UI cards. Make the whole center lawn EMPTY, a perfectly regular subtle 9-column by 5-row checkerboard extending horizontally from 11% to 89% and vertically from 21% to 89% of image. Rectangular horizontal rows, no perspective distortion. Retain a simple long wooden toolbar spanning the top 3% to19% (without slots/text/icons) and a small blank wooden instruction plank centered bottom92%-99%. Beautiful detailed painterly original backdrop, lush mint and lime checkerboard, golden natural lighting, gentle readable lawn texture. The empty playing lawn must be flat and clear. No characters and no text anywhere.

### plants-v1.png

Use case: stylized-concept. Asset type: transparent sprite animation sheet for this approved garden defense game. Image1 is STYLE reference only. Create ONE STRICT rectangular sprite atlas with EXACTLY 5 columns and4 rows,20 separate sprites. Transparent background, no ground/no background/no labels/no grid lines. Each cell exactly equal size, centered full body sprite, generous transparent margins no overlap. Columns lefttoright:1 smiling golden sunflower with green leafbase,2 cute limegreen pea cannon plant facingRIGHT with a round head and tubular muzzle,3 tall golden brown walnut defender with expressive eyes green sprout,4 iceblue crystalline flower cannon facingRIGHT with green leafbase,5 mischievous red chili pepper standing on leaves. Rows are four animation poses of SAME characters: row1 neutral idle, row2 gentle squash down/anticipation, row3 stretched bright active firing pose (cannon muzzle open, sunflower petals sway, nut alarm eyes, chili curved), row4 recoil relaxed pose. Preserve identical character size/design within each column. Professional hand-painted cartoon game sprite art, dimensional shaded forms, bold warm dark olive outlines, golden highlights, crisp details, polished mobile game art. Facing RIGHT. No projectile or explosion added. Exact5columns×4rows sprite atlas, all20 characters whole and uncropped. Wide5:4 canvas.

### enemies-v1.png

Use case: stylized-concept. Asset type: transparent enemy walking animation sprite sheet for this approved garden defense game. Image1 STYLE reference only. Create ONE STRICT4column4row square sprite atlas,16 equal cells. Transparent background no lawn no background no labels no lines. All16 sprites fully separated centered padded and whole uncropped. Columnslefttoright:1 small cute grumpy shaggy leafgreen garden goblin with brown boots,2 smaller fast red radish runner with leafy tuft and skinny boots,3 medium blue goblin wearing oversized dented silver bucket helmet,4 large bulky mossgreen garden troll carrying wooden stump with purple shorts. All enemies faceLEFT (side threequarter profile), amusing not frightening. Rows1to4 are four WALK CYCLE frames, alternating forward foot/back foot, recoil foot contact and stepping, distinct arms/legs phases. Keep SAME character details/colors/scale between frames in column, head tilted LEFT and mouth expressive. Match approved artwork: premium hand-painted cartoon game sprites, warm crisp dark olive outlines, rounded dimensional shaded forms, luminous detailed foliage/hair, polished expressive faces, painterly highlights. Exactly4columns×4rows with no overlap, clean genuine transparent alpha.

### props-v1.png

Use case: stylized-concept. Asset type: shipped garden defense game transparent items atlas. Image1 STYLE reference only. Create a strict FOUR COLUMN ONE ROW sprite sheet. Transparent background. Four equally spaced square cells separated with transparent margins. Cell1 a luminous golden sun collectable with smooth round amber glowing core and twelve pointed light rays, shining white highlight, no face. Cell2 a small charming red garden lawnmower cart facing RIGHT side view with big two dark wheels, green leaf handle and wooden trim, fully whole. Cell3 glossy bright green spherical pea projectile, dimensional lime highlights and dark green edge. Cell4 glossy icy blue spherical pea projectile, cyan highlights tiny sparkles. Premium hand-painted cartoon mobile game asset style matching approved image, crisp warm dark outlines, vivid shaded depth. No text/labels/background or additional items. Strict4columns1row each equal cell. Wide4:1 canvas.

## 运行处理

图集绘制时使用每个角色的实际透明边界，而不是假定生成结果严格等分；四帧以脚底对齐并按中性帧比例绘制，避免邻帧漏出、漂浮和表情挤压。透明通道原样保留。单独使用平滑纹理采样。植物呼吸、射击后坐、怪物四帧走路/啃咬、冰冻、命中粒子和阳光收集轨迹由运行时绘制。
