# Snowflake syntax overview

## variable
```sf
pi : float64 = 3.14

** or seperate
pi : float64
pi = 3.14

** or guess type and cast similar to Go
pi := 3.14
```

## string
```sf
** string
"hello world"

** char
'a'

** template literal
a := "world"
`hello ${a}`
** => hello world
```
all strings are an IO

## functions and calls
```sf
fn say(name: string) {
  std:print <- "say: " <- name
}

say("hello world")
** say: hello world
```

## loops
```sf
** there are no traditional loops, only recursion
fn rec(n) {
  when n >= 10 {
    break
  }
  
  std:out <- n <- " "
  
  rec(n + 1)
}

rec(10)
** 1 2 3 4 5 6 7 8 9 10 
```

## yields and blocks
```
fn double() {
  yield
  yield
}

double() -> {
  std:print <- "hi"
}
** hi
** hi
```

## types
[superwhiskers] TODO: explain types

## memory management
[superwhiskers] TODO: explain memory management  

## namespaces and libraries
```
lib math {
  PI := 3.1415
  
  fn add(a: int, b: int) {
    return a + b
  }

  export { PI, add }
}

math:PI
** 3.1415

math:add(1, 2)
** 3
```

## comments
```
** a comment, it spans one line
```

### doc tags
** doc tag list **
```
** bug: describing a bug
** todo: something that needs to be done eventually
** note: a note that's meant to be displayed as a note
** fixme: please fix me
** optimize: describing that the code can be optimized or how to optimize it
** depricated: a notice meant to warn and tell users what the replacement feature is
```
```
** adds two numbers together
** `add(1, 2) ** => 3`
fn add(a: int, b: int): int {
  a + b
}

** a minor doc tag
** todo: change from `std:out` to `std:print`
fn say(text) {
  std:out <- text <- newline
}

** a major doc tag
** TODO: Finish function
fn unfinished_function() {
}

** Assigning a doc tag to a person

** from [brecert] to [superwhiskers] 
** todo: please add while loops to the language
fn parser() {}
```
