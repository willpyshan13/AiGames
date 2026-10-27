# 安卓 0.1.9 发布验证

- 版本：0.1.9；内部版本号：16777481（上一版 16777480）。
- 发布包：output/magnet-island-0.1.9.apk，51,545,626 字节（49.16 MiB）。
- SHA-256：ef525792a507e13fe49878de71ef9845611a7d62e154c5f9a1deb71a0ce532e4。
- ARM64 单架构，最低 Android 8.0/API26，target API35。
- Release 配置 debug=0、strip=true；已额外 llvm-strip，成品主库无 .debug/.symtab 节。
- apksigner v2/v3 验证通过；zipalign 16 KiB 对齐验证通过；Manifest 没有启用 debuggable。
- 新小兔长地图、动作图集、道具及界面四份运行资源均已包含。排除40份未引用资源；PNG按已有确认的质量90 WebP方案打入，保留尺寸与透明通道。
- 155项库测试全部通过，含长地图两种速度五档无伤通关与连续跳跃。安卓使用同一长地图速度和空中推进参数，避免旧安卓短跳不能过河。
- 构建前清理target/android；修复旧构建目录缺少CACHEDIR.TAG导致新版cargo拒绝clean的问题。新包验证成功后脚本按规则清理旧交付APK。
- 沿用项目现有本地签名配置。未安装到设备，未启动游戏；安卓真机画面与手感尚未验证。
