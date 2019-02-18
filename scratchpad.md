
simple http server idea
```sf
import server from "http"

server := HTTP:Server() -> ctx {
  ctx.response["content-type"] = "text/plain"
  ctx.response.print("hello world")
}
server.tcp_port := 8080
server.listen()
```
