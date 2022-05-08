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

### 数据库存储

项目主要存储书店、图书和书评等基本信息，数据结构清晰, 相互之间的关系简单，采用新的数据库技术 [EdgeDB](https://www.edgedb.com/docs/intro)（a next-generation graph-relational database.）, 主要有以下好处：

- 在项目仓库中管理数据库模型文件；
- 和基础数据相关的公开查询逻辑直接写到数据库层，无需经过 api sever；
- 支持 HTTP 和 GraphQL 的查询；

本地安装 **edgedb** 命令行工具：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.edgedb.com | sh
```

- 管理 Model Schema：https://www.edgedb.com/docs/datamodel/index
- 通过 http 查询数据：https://www.edgedb.com/docs/clients/90_edgeql/index
	- http://<hostname>:<port>/db/<database-name>/edgeql

数据库实例 Docker 部署：EdgeDB Docker container requires 1GB RAM!
https://github.com/edgedb/edgedb-docker
裸机安装部署：https://www.edgedb.com/docs/guides/deployment/bare_metal

### Assets 外部资源

图片文件存储

```html
<link data-trunk rel="scss" href="assets/app.scss" />
```

- Trunk Asset 编译打包: https://trunkrs.dev/assets/
- Dart Sass: https://sass-lang.com/dart-sass

### 生产环境部署

[ ] 生产环境的最佳实践，性能优化

静态文件和可执行文件生成：
- release 版本：**bash prod.sh**

发布到云服务器
```
$ server/target/release/server --static-dir ./dist --port 8080
```

https 证书和服务器配置

### 应用维护和监控

- 数据备份
- Netdata

### SEO 优化管理

#### 参考资料

- https://github.com/rksm/axum-yew-setup