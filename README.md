# financial-api-rs

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

面向[同花顺官方金融数据服务](https://github.com/HiThink-Tech/Financial-API)的异步 Rust
客户端，提供已上线 REST API 的类型化访问。

## 安装

```shell
cargo add financial-api
```

使用前设置 API Key：

```shell
export HITHINK_FINANCE_API_KEY='<your-api-key>'
```

## 快速开始

```rust,no_run
use financial_api::{Client, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::from_env()?;
    let response = client
        .index_constituents_ths_stock_list("000300.SH")
        .await?;

    println!("{:#?}", response.data());
    Ok(())
}
```

## 支持范围

当前覆盖 59 个已上线端点，包括：

- 标的检索与列表
- A 股行情、财务、估值、竞价及特色数据
- 同花顺指数目录、成分股与行情
- 基金资料、持仓、业绩、经理、公司、资讯及行情
- 全市场日 K 与复权因子导出

上游标记为“敬请期待”的能力不在当前支持范围内。完整用法可查看
[API 文档](https://docs.rs/financial-api)和仓库中的
[`examples/`](https://github.com/Choi-Jungwoo/financial-api-rs/tree/master/examples)。

## 项目说明

- 使用本项目需要同花顺金融数据服务的有效账号、API Key 和对应权限。
- 数据内容、更新频率与服务可用性由上游提供方决定。
- 本项目及其示例不构成投资建议。
- 源码与问题跟踪位于
  [Choi-Jungwoo/financial-api-rs](https://github.com/Choi-Jungwoo/financial-api-rs)。

## 许可证

本项目使用 [MIT License](./LICENSE)。该许可证不授予同花顺服务、数据、商标或其他上游
资产的权利。
