🧪 想测试你的服务极限？试试压测工具：

# 安装 wrk (高性能 HTTP 压测工具)
brew install wrk  # macOS

# 模拟 100 并发，持续 10 秒请求 /todos
wrk -t10 -c10000 -d10s http://localhost:3000/todos

你将看到：
- 请求总数（QPS）
- 平均延迟
- 最大延迟