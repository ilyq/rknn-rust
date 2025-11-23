
# rust ARMv7 编译

```
# 查看已安装的工具链
rustup show

# 查看可用的目标平台
rustup target list

# 添加 ARMv7 目标
rustup target add armv7-unknown-linux-gnueabihf

# 在项目目录中编译
cargo build --target armv7-unknown-linux-gnueabihf

# 发布版本
cargo build --release --target armv7-unknown-linux-gnueabihf
```

# luckfox 编译工具链
```
GCC 下载地址：https://console.zbox.filez.com/l/H1fV9a (提取码是：rknn)

arm-rockchip830-linux-uclibcgnueabihf
```
