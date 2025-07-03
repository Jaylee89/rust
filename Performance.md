🧪 想测试你的服务极限？试试压测工具：

# 安装 wrk (高性能 HTTP 压测工具)
brew install wrk  # macOS

# 模拟 100 并发，持续 10 秒请求 /todos
wrk -t10 -c10000 -d10s http://localhost:3000/todos

你将看到：
- 请求总数（QPS）
- 平均延迟
- 最大延迟

## performance principal

| 类型   | `RwLock<Vec<Todo>>` | `DashMap<u32, Todo>` |
| ---- | ------------------- | -------------------- |
| 并发写入 | 差                   | 优秀                   |
| 并发读取 | 一般                  | 优秀                   |
| 延迟   | 会等待写锁               | 几乎无等待                |
| 线程安全 | 手动处理锁               | 自动并发支持               |


```
Running 10s test @ http://localhost:3000/todos
  10 threads and 10000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   794.74ms  291.90ms   1.35s    66.80%
    Req/Sec    25.12     36.37   150.00     87.50%
  279 requests in 10.10s, 29.70KB read
  Socket errors: connect 0, read 27585, write 0, timeout 23
Requests/sec:     27.63
Transfer/sec:      2.94KB
```

```
Running 10s test @ http://localhost:3000/todos
  10 threads and 10000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   743.84ms  473.62ms   1.99s    78.33%
    Req/Sec    23.30     41.78   212.00     90.74%
  307 requests in 10.11s, 49.77KB read
  Socket errors: connect 0, read 28184, write 0, timeout 67
Requests/sec:     30.37
Transfer/sec:      4.92KB
```