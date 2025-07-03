# rust


## Study

🧱 学习资源推荐（高级开发者版）

📘 1. 《Rust Book》（官方文档）
地址：https://doc.rust-lang.org/book/
⭐ 建议快速扫一遍（尤其是所有权、生命周期、错误处理）
对你这种有 C/C++/Swift/TypeScript 背景的开发者来说，读起来不会太难。


📗 2. 《Rust by Example》
地址：https://doc.rust-lang.org/rust-by-example/
代码驱动型，适合你动手感受语法和惯用法。


📙 3. 《Rust for Rustaceans》
- 作者：Jon Gjengset（YouTube 上也很活跃）
- 适合掌握基础后进阶，深入理解：
  - 异步编程
  - 生命周期高级用法
  - 宏系统
  - Unsafe Rust`（必要时）`



## 🧪 实战项目推荐（结合你背景）

✅ 项目方向 1：用 Rust 写一个后端服务
你可以选择一个你熟悉的领域，比如：

写一个极简的 Web API server（使用 actix-web 或 axum）
数据缓存服务（类似 Redis）
高性能边缘函数（部署在小设备上）
Rust 框架推荐：
| 需求       | 框架                  |
| -------- | ------------------- |
| Web 服务   | `axum`, `actix-web` |
| 异步任务     | `tokio`             |
| 数据库访问    | `sqlx`, `diesel`    |
| HTTP 客户端 | `reqwest`           |
| 配置管理     | `config`, `dotenv`  |


✅ 项目方向 2：用 Rust 写一个 CLI 工具
比如：

- 一个 JSON/YAML 配置格式转换工具
- 文件同步或备份工具
- 性能监测工具（类似 top）

工具推荐：

| 功能       | Crate               |
| -------- | ------------------- |
| CLI 参数解析 | `clap`, `structopt` |
| 日志输出     | `log`, `env_logger` |
| 文件处理     | `walkdir`, `serde`  |
| 并发       | `rayon`, `tokio`    |

✅ 项目方向 3：WASM + Rust + 前端交互
如果你熟悉前端开发，可以试试：

- 用 wasm-bindgen + yew 构建前端组件
- Rust 写逻辑 + TS 写 UI

参考项目：使用 Rust 编写音频处理或视频编解码模块，导出成 WebAssembly 供浏览器调用。

🎯 学习建议（针对经验丰富者）

- 从项目出发，不要死磕语法：你可以直接选个小项目做，遇到再查文档。
- 关注底层原理：Rust 很适合锻炼底层认知（内存模型、并发、系统调用等）。
- 遇到 borrow checker 不要怕：刚开始卡很正常，慢慢你会养成写“结构更好”的代码。
- 多用 Cargo 和 crates.io：社区包管理成熟，很多轮子可以直接用。
