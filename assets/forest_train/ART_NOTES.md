# 森林小火车美术记录

内置 imagegen 生成及编辑；未使用 CLI 或外部付费 API。确认稿：`designs/forest-train-v1/approved.png`，2026-10-29 用户回复“满意”。

## 运行资产

- `approved.png`：确认稿原件，1672×941。除动态区域以外直接绘制，不重画森林、车站、木桥、按钮或文字。
- `empty-tracks.png`：同尺寸空轨道补片，仅在两辆静态火车原位置和进度数字区域采样。
- `trains-exact.png`：同尺寸透明图集，保留原红车/蓝车所在区域；运行时裁取后分别移动，蓝车/红车需要反向时水平翻转。
- `controls.png`：同尺寸透明图集，提取金星和两个圆形岔道按钮，避免动态状态带出草地或轨道的方形背景。

## 最终处理提示词

### 空轨道底图（输入：已确认的原图）

Use case: precise-object-edit. Implementation asset extraction for an APPROVED game screen. Keep EXACT same landscape framing, dimensions, composition, colors, perspective, stations, scenery, rails, bridge, switches J1 and J2, HUD, buttons, and ALL Chinese text. Only remove the two toy trains (red train with rabbit driver on left tracks, blue train with squirrel on upper right tracks), their steam clouds and train shadows; inpaint the empty rail tracks, sleepers, grass, platform behind them, with continuous rail geometry. Do not remove the standing rabbit at left station. Also erase ONLY the 2 in top HUD 送达 2 / 6, filling with matching cream panel so actual runtime number can be drawn there. All other text including / 6 remains. Do not redesign, restyle, reposition or add ANYTHING. Output the approved screen with empty rails so moving trains can be rendered independently.

### 最终透明列车（输入：已确认的原图；透明背景）

Precise image extraction, NOT a new drawing. Preserve the original tiny trains from the supplied screenshot exactly, with their original 3/4 raised camera view, original proportions and shapes, original cream rabbit driver and orange squirrel driver. Remove everything EXCEPT the two train objects from this reference. Keep their ORIGINAL positions and ORIGINAL relative sizes on the SAME 1672x941 transparent canvas. Red train stays in its exact original bounding rectangle around x80..292,y290..410. Blue train stays exactly around x1218..1445,y195..303. Do not enlarge either train or turn either train into side view, do not center the trains, do not change composition. Discard smoke as well. Whole rest of canvas alpha zero. Output exact train cutouts at their original positions with accurate transparent silhouettes. No scenery, no rails, no shadow halos, no text.

首版居中双行列车图集因比例不符弃用，不接入游戏。

### 最终透明状态图集（输入：已确认的原图；透明背景）

Precise alpha extraction from an approved screenshot, NOT redraw. Same 1672x941 transparent canvas. Keep ONLY THREE existing objects at their original positions and original sizes: (1) the glowing GOLD STAR on grass at x257..321,y478..545, remove ALL grass and background from around the star, preserving only gold silhouette and translucent glow; (2) J1 round green switch button at x568..661,y392..477, keeping round wooden rim, green circle and white arrows, but NO J1 label, NO tracks or grass outside circular rim; (3) J2 round green switch button at x1080..1173,y391..477, keeping round wooden rim, green circle and white arrows, but NO J2 label or background outside its circular rim. ALL other pixels transparent alpha zero. Do NOT move, enlarge, center, rearrange or redraw these three objects. Do not include the top HUD stars. This is a cutout sheet at original coordinates for replacing runtime UI states seamlessly. No rectangular background patches, no text, no rails, no vegetation, no trains. Exactly original objects preserved.
