# 灵活数据查询方案调研

本文档调研 AI 助手灵活查询数据库的各种方案，对比固定 Tool 与动态 SQL 查询的优缺点。

## 当前方案：固定 MCP Tools

当前项目使用预定义的 MCP 工具来查询数据：

```rust
// Boss MCP Tools
- query_orders          // 查询订单列表
- query_piece_records   // 查询计件记录
- get_worker_stats      // 获取员工产量统计
- get_overview          // 获取首页概览
- get_order_progress    // 获取订单进度
- get_unpaid_summary    // 获取待发工资汇总
```

**优点：**
- 安全性高：只能执行预定义的查询
- 结果结构化：返回类型明确，便于 UI 渲染
- 权限控制：每个工具内置了基于 `Claims` 的权限过滤

**缺点：**
- 灵活性差：用户只能查询预设的数据视角
- 开发成本：每个新查询需求都要开发新工具
- 无法处理临时查询："帮我查一下张三上个月做了多少件"

## 方案对比

### 方案一：SQL MCP Server

让 LLM 直接生成 SQL 查询数据库。

#### 开源实现

| 项目 | 支持数据库 | 特点 |
| ---- | ---------- | ---- |
| [MCP Alchemy](https://github.com/runekaagaard/mcp-alchemy) | PostgreSQL, MySQL, SQLite, Oracle, MSSQL 等 | 基于 SQLAlchemy，功能完善 |
| [PostgreSQL MCP](https://github.com/modelcontextprotocol/servers) | PostgreSQL | 官方参考实现，只读查询 |
| [SQLite MCP](https://github.com/modelcontextprotocol/servers) | SQLite | 官方参考实现 |
| [MSSQL MCP](https://github.com/RichardHan/mssql_mcp_server) | SQL Server | 安全的数据库交互 |

#### MCP Alchemy 功能

```json
// Claude Desktop 配置示例
{
  "mcpServers": {
    "my_postgres_db": {
      "command": "uvx",
      "args": ["--from", "mcp-alchemy", "--with", "psycopg2-binary", "mcp-alchemy"],
      "env": {
        "DB_URL": "postgresql://user:password@localhost/dbname"
      }
    }
  }
}
```

提供的工具：
- `all_table_names` - 列出所有表名
- `filter_table_names` - 搜索表名
- `schema_definitions` - 获取表结构（列、类型、主键、外键）
- `execute_query` - 执行 SQL 查询

**优点：**
- 极大的灵活性：任意 SQL 查询
- 零开发成本：不需要为每个查询写代码
- Schema 感知：LLM 可以理解表结构

**缺点：**
- 安全风险：SQL 注入、数据泄露
- 权限控制困难：难以实现行级权限（如只能看自己的数据）
- 性能风险：LLM 可能生成低效查询
- 结果格式不固定：UI 难以渲染

### 方案二：Text-to-SQL Agent

在后端实现 Text-to-SQL 转换，而不是让 LLM 直接访问数据库。

#### 架构

```
用户自然语言 → LLM 生成 SQL → 后端验证/改写 → 执行查询 → 返回结果
```

#### 开源框架

| 框架 | 描述 | 特点 |
| ---- | ---- | ---- |
| [Vanna](https://github.com/vanna-ai/vanna) | Chat with your SQL database | 支持多种 LLM 和数据库，RAG 增强 |
| [LangChain SQL Agent](https://python.langchain.com/docs/tutorials/sql_qa/) | SQL Q&A Agent | 完整的 Agent 工作流 |
| [SQLCoder](https://defog.ai/sqlcoder/) | 专门的 Text-to-SQL 模型 | 高准确率，可本地部署 |

#### Vanna 示例

```python
from vanna.openai import OpenAI_Chat
from vanna.chromadb import ChromaDB_VectorStore

class MyVanna(ChromaDB_VectorStore, OpenAI_Chat):
    def __init__(self, config=None):
        ChromaDB_VectorStore.__init__(self, config=config)
        OpenAI_Chat.__init__(self, config=config)

vn = MyVanna(config={'api_key': 'sk-...', 'model': 'gpt-4'})

# 连接数据库
vn.connect_to_postgres(host='localhost', dbname='stitchwork', user='user', password='pass')

# 训练（可选，提高准确率）
vn.train(ddl="CREATE TABLE orders (id UUID, product_name VARCHAR, ...)")
vn.train(question="查询本月订单", sql="SELECT * FROM orders WHERE created_at >= ...")

# 自然语言查询
sql = vn.generate_sql("帮我查一下张三上个月做了多少件")
# → SELECT SUM(quantity) FROM piece_records WHERE user_id = '...' AND ...
```

**优点：**
- 灵活性高：支持任意自然语言查询
- 可控性：后端可以验证、改写 SQL
- 可训练：通过示例提高准确率

**缺点：**
- 准确率问题：复杂查询可能出错
- 需要额外服务：依赖向量数据库存储训练数据
- 权限控制仍需自行实现

### 方案三：混合方案（推荐）

结合固定 Tools 和动态查询的优点。

#### 架构

```
┌─────────────────────────────────────────────────────────┐
│                     AI Assistant                        │
├─────────────────────────────────────────────────────────┤
│  固定 Tools（常用场景）    │   动态查询 Tool（灵活场景）  │
│  - get_overview           │   - flexible_query          │
│  - query_orders           │     ↓                       │
│  - get_my_earnings        │   Text-to-SQL 引擎          │
│       ↓                   │     ↓                       │
│   预定义查询逻辑           │   SQL 验证 & 权限注入        │
│       ↓                   │     ↓                       │
│     数据库                 │     数据库                   │
└─────────────────────────────────────────────────────────┘
```

#### 实现思路

1. **保留固定 Tools**：常用场景使用预定义工具，保证安全性和 UI 渲染

2. **添加 flexible_query Tool**：处理临时查询需求

```rust
// crates/server/src/mcp/flexible.rs
#[derive(Debug, Deserialize, JsonSchema)]
pub struct FlexibleQueryParams {
    #[schemars(description = "自然语言查询描述，如：查询张三上个月的计件记录")]
    pub query: String,
}

#[tool(description = "灵活查询数据，支持自然语言描述")]
pub async fn flexible_query(
    &self,
    Parameters(params): Parameters<FlexibleQueryParams>,
) -> Result<Json<FlexibleQueryResult>, ErrorData> {
    // 1. 调用 Text-to-SQL 引擎生成 SQL
    let sql = self.text_to_sql(&params.query).await?;

    // 2. 验证 SQL 安全性（只允许 SELECT）
    if !self.is_safe_query(&sql) {
        return Err(ErrorData::new("不安全的查询"));
    }

    // 3. 注入权限过滤（自动添加 WHERE boss_id = ? 或 user_id = ?）
    let safe_sql = self.inject_permission_filter(&sql)?;

    // 4. 执行查询
    let result = self.execute_query(&safe_sql).await?;

    Ok(Json(FlexibleQueryResult {
        sql: safe_sql,  // 返回实际执行的 SQL，便于调试
        data: result,
    }))
}
```

3. **SQL 安全验证**

```rust
fn is_safe_query(&self, sql: &str) -> bool {
    let sql_upper = sql.to_uppercase();

    // 只允许 SELECT
    if !sql_upper.trim().starts_with("SELECT") {
        return false;
    }

    // 禁止危险关键字
    let forbidden = ["INSERT", "UPDATE", "DELETE", "DROP", "TRUNCATE", "ALTER", "CREATE"];
    for keyword in forbidden {
        if sql_upper.contains(keyword) {
            return false;
        }
    }

    true
}
```

4. **权限注入**

```rust
fn inject_permission_filter(&self, sql: &str) -> Result<String, Error> {
    // 使用 SQL 解析器（如 sqlparser-rs）分析查询
    let ast = Parser::parse_sql(&GenericDialect, sql)?;

    // 根据用户角色注入过滤条件
    match self.claims.role {
        Role::Boss => {
            // 老板只能看自己的数据
            // 自动添加 WHERE boss_id = '{self.claims.sub}'
        }
        Role::Staff => {
            // 员工只能看自己的数据
            // 自动添加 WHERE user_id = '{self.claims.sub}'
        }
        Role::Admin => {
            // 超管可以看所有数据
        }
    }
}
```

## 安全考虑

### SQL 注入防护

| 层级 | 措施 |
| ---- | ---- |
| 输入验证 | 只允许 SELECT 语句 |
| 关键字过滤 | 禁止 INSERT/UPDATE/DELETE/DROP 等 |
| 参数化查询 | 使用预编译语句 |
| 权限注入 | 自动添加用户权限过滤 |
| 查询超时 | 设置执行超时时间 |
| 结果限制 | 限制返回行数（如最多 1000 行） |

### 数据访问控制

```rust
// 表级权限配置
const TABLE_PERMISSIONS: &[(&str, &[Role])] = &[
    ("orders", &[Role::Boss]),
    ("piece_records", &[Role::Boss, Role::Staff]),
    ("users", &[Role::Admin]),
    ("payrolls", &[Role::Boss, Role::Staff]),
];

// 敏感字段过滤
const SENSITIVE_COLUMNS: &[&str] = &[
    "password_hash",
    "phone",  // 脱敏处理
];
```

## 推荐方案

对于 StitchWork 项目，建议采用 **混合方案**：

### 短期（保持现状）

继续使用固定 MCP Tools，因为：
- 业务场景相对固定
- 安全性要求高
- UI 渲染需要结构化数据

### 中期（按需添加）

当用户提出新的查询需求时：
1. 评估是否为高频场景 → 是则添加新的固定 Tool
2. 低频临时查询 → 考虑实现 `flexible_query` Tool

### 长期（完整方案）

如果灵活查询需求增多：
1. 集成 Text-to-SQL 引擎（推荐 Vanna 或自研）
2. 实现完整的 SQL 安全验证和权限注入
3. 添加查询缓存和性能监控

## 相关资源

### MCP SQL Servers
- [MCP Alchemy](https://github.com/runekaagaard/mcp-alchemy) - 多数据库支持
- [官方 MCP Servers](https://github.com/modelcontextprotocol/servers) - PostgreSQL, SQLite 参考实现
- [MSSQL MCP Server](https://github.com/RichardHan/mssql_mcp_server) - SQL Server 支持

### Text-to-SQL 框架
- [Vanna](https://github.com/vanna-ai/vanna) - 开源 Text-to-SQL，支持 RAG
- [LangChain SQL Agent](https://python.langchain.com/docs/tutorials/sql_qa/) - Agent 工作流
- [SQLCoder](https://defog.ai/sqlcoder/) - 专用 Text-to-SQL 模型

### 参考文章
- [AWS: Build a robust text-to-SQL solution](https://aws.amazon.com/blogs/machine-learning/build-a-robust-text-to-sql-solution-generating-complex-queries-self-correcting-and-querying-diverse-data-sources/)
- [Google Cloud: Techniques for improving text-to-SQL](https://cloud.google.com/blog/products/databases/techniques-for-improving-text-to-sql)
- [K2View: MCP SQL server - Connecting LLMs to enterprise data](https://www.k2view.com/blog/mcp-sql-server/)

## 结论

| 方案 | 灵活性 | 安全性 | 开发成本 | 推荐场景 |
| ---- | ------ | ------ | -------- | -------- |
| 固定 Tools | ⭐⭐ | ⭐⭐⭐⭐⭐ | 中 | 业务场景固定，安全要求高 |
| SQL MCP | ⭐⭐⭐⭐⭐ | ⭐⭐ | 低 | 内部工具，开发/调试用 |
| Text-to-SQL | ⭐⭐⭐⭐ | ⭐⭐⭐ | 高 | 需要灵活查询，有资源投入 |
| 混合方案 | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | 中高 | 平衡灵活性和安全性 |

对于生产环境的移动应用，**固定 Tools + 按需扩展** 是最稳妥的选择。
