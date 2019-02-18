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
