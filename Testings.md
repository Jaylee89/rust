
# 健康检查

```
curl http://localhost:3000/health
```

# 添加 todo

```
curl -X POST http://localhost:3000/todos \
  -H "Content-Type: application/json" \
  -d '{"title": "Learn Rust"}'
```

# 获取 todos

```
curl http://localhost:3000/todos
```

# print response time

```
curl -s -o /dev/null -w "Response time: %{time_total}s\n" http://localhost:3000/health
```

```
curl -s -o /dev/null -w "Response time: %{time_total}s\n" http://localhost:3000/todos
```