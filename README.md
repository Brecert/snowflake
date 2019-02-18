# snowflake
> a fast, minimal and strongly-typed programming language

I'm 90% sure that you won't like this

Here's an example of hello world in it

please view the [real snowflake](https://github.com/Brecert/snowflake)

this fork adds ideas to the language that may not be accurate to the creators ideas

```snf
** this is a comment
fn main() {

  ** there is no print statement, instead IO's have the assignment operator that adds to the current stream
  std:out <- "hello world" <- newline
}
```

```snf
** don't do this
** for example only
type int := int32 | int64
type float := float32 | float64
type number := int | float

fn fib(n: number) {
 when n > 2 { 
   return fib(n - 1) + fib(n - 2)
 }
 else {
   return n
 }
}

fn main() {
 std:out <- fib(16)
 ** 987
}
```
