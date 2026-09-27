# uma-hachimi-mod

赛马娘线专用的 Hachimi-Edge 魔改版：**摘除翻译与贴图管线，只保留画面/功能增强**，并内建合并 `hachimi-ura` 相关的 hook。

> **私有仓，不对外发布。** 本仓库仅供仓主自用构建。

---

## 这是什么

源码拷贝自 [kairusds/Hachimi-Edge](https://github.com/kairusds/Hachimi-Edge)（Rust，GPLv3），**与上游无 fork 关系**，仅为内部基线。

在赛马娘这条线上，原版的两大管线是负资产：

| 上游能力 | 本仓处理 |
|---|---|
| 文本翻译（UI / master.mdb / 剧情 / 歌词） | **摘除** |
| 贴图、图集替换 | **摘除** |
| 画质与功能增强（帧率解锁、分辨率缩放、Race Stat HUD 等） | **保留** |
| `hachimi-ura` 合并的 hook | **保留** |

理由：赛马娘本体自带完整汉化，上游翻译管线既无收益，又带来 `rust-i18n`、`ureq`、`png`、`image`、`blake3` 等一整套依赖和运行时开销。

---

## 相对上游改了什么

当前魔改分支 `workbench/no-translation`，相对 `main` 领先 40 个提交。

### 摘除

- `assets/locales/ko.yml` 整份删除（-1065 行）
- `src/il2cpp/hook/UnityEngine_TextRenderingModule/TextGenerator.rs`（-200）
- `src/il2cpp/hook/umamusume/StoryTimelineData.rs`（-557）
- `src/il2cpp/hook/umamusume/UIManager.rs`（-119）
- `src/il2cpp/hook/umamusume/GameSystem.rs`（-113）
- `src/il2cpp/hook/umamusume/mod.rs`（-96）
- `create_release.yml` 上游的多平台发布逻辑（-206）

### 保留 / 调整

- `src/android/hook.rs`、`src/android/main.rs` —— Android 侧 hook 与入口
- `src/lib.rs`
- `Application.rs`、`TextMesh.rs`、`Text.rs` —— 保留渲染增强相关部分
- `create_release.yml` 改为只出 Android `.so`，`prerelease`，body 注明仅供仓主

---

## 构建与发布

CI 走 `workflow_dispatch`（`.github/workflows/create_release.yml`）：

```
Android: aarch64-linux-android，RELEASE=1 ./tools/android/build.sh
产物:   build/*.so + build/sha256.json
发布:   softprops/action-gh-release，tag 取 Cargo.toml 的 version，prerelease
```

**已知构建前提**：工作流会现场 clone `emilk/egui` 并 patch `combo_box.rs`（绕开 `set_min_width` 问题），再把路径依赖写进 `Cargo.toml`。

**版本号唯一来源**：`Cargo.toml` 的 `version`，当前 `0.31.2` → tag `v0.31.2`。

**注意**：release 只在 `workbench/no-translation` 上验证过；`main` 仍是未魔改的上游基线，不要从 `main` 发版。

---

## 分支说明

| 分支 | 内容 |
|---|---|
| `main` | 上游原样基线，勿改 |
| `workbench/no-translation` | **魔改实际所在**，改代码、发版都走这条 |

---

## 来源与许可

代码来自 kairusds/Hachimi-Edge，遵循其 [GNU GPLv3](LICENSE)。
本仓为私有衍生作品，不对外分发。
