# financial-api-rs

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

面向[同花顺官方金融数据服务](https://github.com/HiThink-Tech/Financial-API)的异步 Rust
客户端。当前版本覆盖 `llms-full.txt` 中已经上线的 59 个 REST 端点，统一完成 API Key
注入、HTTP 传输、响应信封解码和业务错误分类。

安装 crates.io 版本：

```shell
cargo add financial-api
```

源码与问题跟踪位于
[Choi-Jungwoo/financial-api-rs](https://github.com/Choi-Jungwoo/financial-api-rs)。

## 快速开始

设置上游官方约定的环境变量：

```shell
export HITHINK_FINANCE_API_KEY='<your-api-key>'
```

然后创建客户端并调用端点：

```rust,no_run
use financial_api::{Client, Error, TickerSearchRequest};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::from_env()?;
    let request = TickerSearchRequest::new("贵州茅台")?;
    let response = client.tickers_search(&request).await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
```

也可以显式提供 Key，并配置私有部署或测试服务：

```rust,no_run
use financial_api::{ApiKey, Client, Error};
use std::time::Duration;

fn client() -> Result<Client, Error> {
    Client::builder(ApiKey::new("<your-api-key>")?)
        .timeout(Duration::from_secs(20))
        .build()
}
```

`ApiKey` 的调试输出始终脱敏，底层 HTTP Header 也被标记为敏感。不要在业务日志中自行输出
原始凭据。

## 端点示例

`examples/` 为当前支持的 59 个端点分别提供了可运行示例。文件名与
`SUPPORTED_ENDPOINTS` 中的端点名称一致，例如：

```shell
cargo run --example tickers_search
cargo run --example prices_historical
cargo run --example fund_performance_nav
cargo run --example market_dump_daily_k
```

示例会先尝试从仓库根目录的 `.env` 加载凭证，也兼容已经由 shell 设置的环境变量：

```dotenv
HITHINK_FINANCE_API_KEY='<your-api-key>'
```

这些示例会访问真实上游服务，需要相应账号权限和网络连接。开发质量门中的
`mise run check` 会编译全部示例，但不会发送网络请求。

## 响应与错误

所有端点先检查统一业务信封。HTTP 200 但 `code != 0` 会返回 `Error::Business`，不会被误当
作成功。`BusinessError::kind()` 提供认证、权限、未找到、数据未就绪、限流和上游不可用等
恢复类别；原始 `code`、`message` 与 `request_id` 仍可读取。

各端点返回 `Response<端点数据类型>`，例如标的检索返回 `Response<TickerData>`。响应
结构使用公开 DTO 表达；上游的 `null` 使用 `Option` 保留，数组顺序保持不变，不透明游标
使用 `Cursor` 原样回传：

```rust
use financial_api::{Error, TickerData, Response};

fn consume(response: Response<TickerData>) -> Result<(), Error> {
    for ticker in &response.data().item {
        println!("{} {}", ticker.thscode, ticker.name);
    }
    Ok(())
}
```

少数上游本身就是动态对象的字段仍使用 `JsonValue`，但不会把整个普通端点退化为无类型
JSON。全市场导出接口返回 `MarketDumpUrl`；其中预签名 URL 包装为 `SecretUrl`，调试输出
不会泄露签名，只有显式调用 `expose()` 才能取出。

估值端点的十进制数使用 `PreciseDecimal` 保留 JSON 源数字，不经过二进制浮点转换；可通过
`Display` 或 `as_number()` 读取其无损表示。

## 0.2 API 调整

为保留实时上游的实际响应语义，`AuctionSnapshotData::auction_phase` 改用响应专用的
`AuctionPhase`；涨停天梯日期统一为 `NaturalDate`；基金费率保留为带单位的文本；诊断分类
改用 `FundCategoryCode`；资产配置报告日和经理雷达节点字段改为可选值。迁移时应同步调整
显式类型标注，并在读取可选字段前处理 `None`。

## 领域类型

- `Thscode` 表示带市场后缀的完整标的代码。
- `AShareCode` 进一步限制为六位数字加 `.SH`、`.SZ` 或 `.BJ`。
- `NaturalDate` 验证 `YYYY-MM-DD` 和真实公历日期。
- `UnixMillis` 拒绝负时间戳。
- `FinancialRange` 通过受控构造让“最近 N 期”和“起止时间”互斥。
- 复权、报告频率、基金类型、排行榜周期、排序字段等有限集合均使用枚举。

端点方法直接接受标的代码、日期、报告类型和游标的字符串表示，并在方法边界完成规范化与
校验；调用者只在需要持久保存或复用已验证值时才需要显式构造领域类型。

无损转换实现 `From`；可能失败的外部文本转换实现 `FromStr` 或显式构造函数，避免绕过领域
校验。

## 已支持范围

| 分组 | 端点数 | 能力 |
| --- | ---: | --- |
| 元信息 | 2 | 标的检索、标的列表 |
| A 股 | 22 | 行情、复权、财报、交易日历、竞价、估值、异动、热榜、龙虎榜、涨跌停 |
| A 股指数 | 4 | 同花顺指数目录、成分股、行情快照、历史日线 |
| 基金 | 28 | 资料、持仓、业绩、持有人、经理、公司、财务、诊断、募集、资讯、行情 |
| 全市场导出 | 3 | 十年日 K、近十日日 K、复权因子下载 URL |

`SUPPORTED_ENDPOINTS` 提供当前编译版本的完整端点名称和路径，可用于能力展示或兼容性检查。

上游文档中明确标为“敬请期待”的 A 股基础信息、指数基础信息/权重，以及按个股反查同花顺
指数归属不在支持范围内。

## 开发与验证

工具链和任务统一由 `mise.toml` 管理：

```shell
mise install
mise run fmt
mise run check
mise run clippy
mise run test
mise run doc
mise run doc-test
```

默认测试只使用本地 mock server，不需要网络、真实账号或 API Key。
配置 `.env` 后，可用 `mise run test-examples-live` 顺序执行全部 59 个实时示例；任务会在请求
之间留出间隔，并在结束时汇总成功和失败数量。

## 契约与边界

- 具体请求参数、字段和错误语义以[完整聚合文档](https://fuyao.aicubes.cn/llms-full.txt)与
  [API 总览](https://fuyao.aicubes.cn/docs/api-reference/overview/)为准。
- 本 crate 不缓存、拼接或改写金融数据，也不承诺上游服务的可用性、完整性、延迟和权限。
- `null` 不补零；不透明游标原样回传；自然日不会被解释成交易日。
- 本软件及示例不构成投资建议。使用上游服务和数据时仍需遵守适用条款。

## 许可证

本仓库软件与文档使用 [MIT License](./LICENSE)。该许可证不授予同花顺服务、数据、商标或
其他上游资产的权利。
