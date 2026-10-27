# 小兔跑跳动作调整稿

状态：用户已确认加快稿，已接入游戏。跑动 0.44 秒循环；蓄力 0.06 秒、飞行 1.2 秒、落地 0.12 秒。

## 用户反馈后的加快稿

用户要求“动作要快点”。最新预览将跑动循环由 0.66 秒缩短为 0.44 秒，节奏提高 50%；跳跃飞行由 2 秒缩短为 1.2 秒，起跳压缩由 0.1 秒缩短为 0.06 秒，落地回弹由 0.2 秒缩短为 0.12 秒。当前游戏动画一侧保持原速，便于直接比较。

最新加快稿已确认。接入时同步调整空中推进速度，保留现有关卡所需的跨沟距离和最高点，不能只缩短滞空时间而导致水沟无法通过。下面的 2 秒飞行描述属于上一版预览。

## 与当前动画的差别

- 当前跑动在两张近似姿势之间切换，步频固定；跳跃全程使用同一姿势。
- 调整稿采用 8 个连续跑动姿势：落地、压缩、蹬地、伸展、腾空、收腿、前脚触地、缓冲。耳朵和围巾随姿势变化。
- 跳跃增加起跳前的短暂压缩，空中从伸展过渡到收腿，再伸脚落地；接地后用约 0.2 秒完成轻微压缩和回弹，接回跑动。
- 统一脚底基准和角色比例，减少切帧时的位置抖动；新增随离地高度变化的接地阴影。
- 预览保留 2 秒的跳跃飞行时间与相同最高点，仅调整动作、曲线和接地衔接。实际接入后再按现有关卡检查跳跃距离和判定。

动态对比支持跑跳循环、单独跑动、单独跳跃和慢放。确认后才允许接入游戏，并须再次实际运行、提供画面或录像对照。

## 设计资源与生成记录

图集：`designs/rabbit-motion-v2-atlas.png`，4 列 × 3 行，前 8 格为跑动，后 4 格为起跳、上升、顶点、落地。

使用内置 ImageGen，参考现有 `assets/rabbit/rabbit-sprint-sheet-v1.png`。原始资源未覆盖。

生成提示词：

> Create an ANIMATION DESIGN SPRITE SHEET from the reference rabbit, keeping the exact same fluffy white rabbit identity, pink inner ears, brown eyes, green scarf, hand-painted finish, right-facing profile and proportions. Transparent canvas in a strict 4-column by 3-row grid, 12 equal square cells, no text, no borders. Each full rabbit is centered with consistent scale, shared torso anchor and ground baseline. First 8 cells (rows 1 and 2, left to right) form ONE continuous rabbit bounding/running cycle: 1 compact landing crouch hind legs under body, 2 compressed loading with ears trailing, 3 rear legs forcefully pushing backwards and front paws forward, 4 stretched takeoff, 5 full airborne forward stretch, 6 flight tuck hind legs moving forward, 7 front paws reaching for ground, 8 paws contacting and hind legs folding into next crouch. The legs MUST change distinctly and physically across frames; don't repeat the same running pose. Final 4 cells (row 3) are a platform jump: 9 takeoff crouch, 10 upward elongated leap, 11 apex relaxed tuck with flowing scarf, 12 soft landing crouch. Delicate ear and scarf lag. No scenery, no ground, no shadows. This sheet is for a smooth animation preview, preserve the existing character, no redesign.
