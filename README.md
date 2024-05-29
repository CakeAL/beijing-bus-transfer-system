# Beijing Bus Transfer System 北京公交换乘系统

## 简介

基于 Tauri 框架构建应用，前端使用 Vite + Vue，后端使用 Rust 进行调用 sqlite3 数据库，并进行推荐从 A 站到 B 站的换乘线路。 \
同时使用 python 进行爬取北京公交官网数据。 \
系 USTB 物联网工程大三下人工智能与机器学习大作业——公交换乘系统设计与开发（2 人组） \
***目前功能都没完成，***

## 功能

1. 搜三元桥->左家庄有惊喜

## 构建和运行

大概是 macOS，Windows 和 Linux 都能运行的。 \
首先确保你已经安装了[`Node.js`](https://nodejs.cn/download/)，[`pnpm`](https://www.pnpm.cn/)以及[`Rust环境`](https://www.rust-lang.org/zh-CN/tools/install)

```bash
# 安装 tauri
cargo install create-tauri-app --locked
# 运行
cargo tauri dev
# 或者
pnpm i
pnpm tauri dev
```

python 爬取数据使用方法（已经有爬好的数据，在`bus-data/bus.db`）： \
[`python-scripts/README.md`](python-scripts/README.md)
