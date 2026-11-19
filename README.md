# BambooLauncher

基于 [GPUI](https://gpui.rs)（Zed 的 GPU 加速 UI 框架）的启动器，当前为 Hello World 骨架。

## 构建与运行

```sh
cargo run
```

## Linux 依赖

链接需要系统库的开发包（以 Void Linux 为例）：

```sh
sudo xbps-install -S libxkbcommon-devel
```

其他发行版对应安装 `libxkbcommon` 的 devel/开发包即可；freetype、xcb 的开发包通常已随桌面环境存在。运行时需要 Vulkan 加载器（GPUI 通过 blade-graphics 使用 Vulkan 渲染），支持 Wayland 与 X11。

## 依赖说明

- `Cargo.lock` 将 `libc` 钉在 `0.2.189`：gpui 的传递依赖 `xattr 0.2.3` 引用 `libc::ENOATTR`，而 `libc 0.2.190` 起在 Linux 上移除了该常量，升级会导致编译失败。待上游（zed-async-tar / xattr）修复后可解除锁定。
