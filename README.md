# BambooLauncher

基于 [gpui-kit](https://gpui-kit.com)（Longbridge 打包的 GPUI 全家桶：GPUI + Base + Component + 默认资源）的启动器，当前为 Hello World 骨架。

## 构建与运行

```sh
cargo run
```

## Linux 依赖

链接需要系统库的开发包（以 Void Linux 为例）：

```sh
sudo xbps-install -S libxkbcommon-devel
```

其他发行版对应安装 `libxkbcommon` 的 devel/开发包即可；freetype、xcb 的开发包通常已随桌面环境存在。运行时需要 GPU 驱动（gpui 0.3 通过 wgpu 渲染），支持 Wayland 与 X11。
