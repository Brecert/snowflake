simple http server idea
```sf
import server from "http"

server = HTTP:Server() -> ctx {
  ctx.response["content-type"] = "text/plain"
  ctx.response.print("hello world")
}
server.tcp_port = 8080
server.listen()
```

psuedocode for each
```
lib each {
  fn in(iterable: Iterable, i : int = 0) {
    while i < iterable.size {
      yield iterable[i], i
      i += 1
    }
  }

  export { in }
} 

array = ['1', 2, 'hi'] of string, int

each:in(array) -> val, i {
  std:print <- `${i}. ${val}`
}
** 1. 1
** 2. 2
** 3. hi
```
