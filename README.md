
# Snowflake
**a fast, minimal and strongly-typed programming language**

please view the [real snowflake](https://github.com/Brecert/snowflake)
this fork adds ideas to the language that may not be accurate to the creators vision

## What is snowflake?
I'm 90% sure that you won't like this
todo: actually describe the language

## What makes snowflake different?
todo: describe what makes snowflake different from other languages like c, cpp, rust, go, and others

## Why snowflake?
todo: describe when and why to pick snowflake

## Why not snowflake?
It's not meant for high level stuff silly
todo: flesh out why snowflake is not made for all uses

## Snowflake priorities
### Minimal
no safety outside of the minimal necessary for people to be able to use it (type safety, etc)

being correct is not an immediate goal of snowflake

### Readability
todo: describe the goals of readability

### Performance
todo: describe how/why performance is important

### Interoperability
high interoperability with both the hardware and other languages
todo: describe why interoperability is wanted

## Guiding Principles
Throughout the design and development of the language, the following principles should be adhered to.
todo: describe the guiding principles

## Examples

Here's an example of hello world in it
```sf
** this is a comment
fn main() {

  ** there is no print statement, instead IO's have the assignment operator that adds to the current stream
  std:out <- "hello world" <- newline
  ** => "hello world\n"
}
```

fibonacci sequence example
```sf
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
 ** => 987
}
```
