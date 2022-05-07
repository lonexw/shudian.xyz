
## shudian.xyz

书店指南：聚合书店信息的单页面网站应用。

### 核心技术栈

- Rust(1.56.0+) 、Cargo and WebAssembly（wasm）, no code of Javascript.
- [Trunk](https://trunkrs.dev/) is a WASM web application bundler for Rust.
- [Yew](https://yew.rs/) is a modern Rust framework for creating multi-threaded front-end web apps using WebAssembly.
- [axum](https://docs.rs/axum/latest/axum/) web application framework.

WASM 浏览器支持情况：https://caniuse.com/wasm

### 开发环境配置

1. 安装 rust 开发环境，推荐 [rustup](https://www.rust-lang.org/tools/install) 工具链；

2. 安装必要的开发工具：
```
# Install WebAssembly target
rustup target add wasm32-unknown-unknown

# WASM web application bundler for Rust
$ cargo install trunk --git https://github.com/thedodd/trunk  

# Adding dependencies to Cargo.toml using cargo add <dependency>
$ cargo install cargo-edit

# Restarting commands on file change 
$ cargo install cargo-watch
```

### 代码协作

[![OM0vF0.png](https://s1.ax1x.com/2022/05/07/OM0vF0.png)](https://imgtu.com/i/OM0vF0)
代码结构说明图示

- 本地开发预览：**bash dev.sh**
- release 版本：**bash prod.sh**

### 生产环境部署

```
$ server/target/release/server --static-dir ./dist --port 8080
```

### 数据库存储

EdgeDB 的选型及使用

### Assets 外部资源

```html
<link data-trunk rel="scss" href="assets/app.scss" />
```

- Trunk Asset 编译打包: https://trunkrs.dev/assets/
- Dart Sass: https://sass-lang.com/dart-sass

### 待办事项

[ ] api 请求的封装 📦 与数据库的 proxy

#### 参考资料

- https://github.com/rksm/axum-yew-setup