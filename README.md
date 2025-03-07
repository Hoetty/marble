# Marble

Marble is a compact and simple functional programming language.
It is inspired by Lambda Calculus, Haskell and other functional programming languages and tries to do as much as possible with a minimal set of features.

Try Marble at https://hoetty.github.io/marble/

## Syntax & Using Marble

### Literals

As in most programming languages, you can create number literals. However, unlike most programming languages, you cannot use digits for that.

These are some valid number literals:
```
Five
FortyTwo
TwoHundredTwentyOneMillionFiveHundredThirtyThousandEighteen
ThreePointOneFour
```

*You might argue, that this seems a little complex for the proclaimed minimal set of features, but it makes numbers follow the style of the rest of the language*

### Strings

String literals can be created by placing text between ```str``` and ```ing``` or using ```string``` to create an empty string.

```
str Hello World ing
string
str 😀🤪 ing
```

### Comments

Comments follow a similar design as strings: A single line comment is started by ```comment``` and goes until the next newline; Everything inside the keywords ```com``` and ```ment``` is part of a multiline comment.

```
comment This is a single line comment

com
This is a multiline comment
Hello!
ment
```

### Functions

As a functional language, marble also supports creating functions. Functions always accept one argument and must return exactly one value.

Here is a function, that directly returns the given argument:
```
fn X do
    X
end
```

Identifiers for function arguments may consist of any unicode character, except for space, tab, newline and the carriage return. This results from the fact, that the source code is parsed as words seperated by whitespace (with the expection of strings and comment). These identifiers are all valid:
```
X
Ö
ß
?
😀
-
```
*Note: Identifiers may shadow previous identifiers of the same name - you can declare the same variable name multiple times.*

You can call a function using the ```of``` operator:
```
PrintLn of str Hello World ing
```

Altough functions must accept exactly one argument, you can define functions that accept multiple values. This is possible through [```currying```](https://en.wikipedia.org/wiki/Currying) where a function returns another functions, which captures the argument of the first.

```
fn F X do
    F of X
end

fn F do
    fn X do
        F of X
    end
end
```
*The second expression shows, how the first function is interpreted*

This implies, that you call a function that accepts multiple arguments using multiple ```of```s:
```
Add of One of One
```

If you want to pass the result of a function call to a function, you can group it using a ```do``` block.

```
Add of Five of do Mul of Two of Two end
```

#### Builtin Functions

Marble offers several builtin functions for common tasks and arithmetic:
- Add/2: Adds its numerical arguments
- Sub/2: Subtracts its numerical arguments
- Mul/2: Multiplies its numerical arguments
- Div/2: Divides its numerical arguments
- Is/2: Tests if its arguments are equal
- IsNot/2: Tests if its arguments are not equal
- Print/1: Prints the argument
- PrintLn/1: Prints the argument and a newline
- And/2: Returns the second argument if the first argument is true, the first otherwise
- Or/2: Returns the first argument if the first argument is true, the second otherwise

All builtin functions support partial application.

### Let

Like in other procedural programming languages, you can put values into variables. To do that, you can use ```let X be Y in```, which assigns Y to X for the following expression:

```
let Greeting be str Hello World ing in
PrintLn of Greeting
```

Altough ```let``` may look like ordinary variable assignment, it actually wraps the expression after ```in``` into a function, that accepts the variables name as argument and calls it using the given value. This is the let expression from above:

```
fn Greeting do
    PrintLn of Greeting
end of str Hello World ing
```

This way variable assignment is actually an expression, that evaluates to the result of the following expression.

### Then

When programming, you often need to execute multiple statements after one another, for example printing multiple things. In Marble you can do this using ```then```:

```
Print of str Hello  ing then
Print of str World ing
```

Note: The left side of a then expression must return a function, that accepts another function. The returned function will then receive the right side of the then expression, and is reponsible for executing it.

### Control Flow

#### Conditional

```True``` and ```False``` are implemented as [Curch Booleans](https://en.wikipedia.org/wiki/Church_encoding#Church_Booleans), meaning they are both functions that accept two arguments, where true returns the first, and false the second:

```
comment True
fn L R do
    L
end

comment False
fn L R do
    R
end
```

You can use the ```If``` function to conditionally return values:
```
If of do Is of Ten of Ten end of One of Zero
comment One
```

As ```True``` and ```False``` themselves return the first or second argument, they already work like if/the ternary operator, so you can just omit the call to the ```If``` function entirely:
```
Is of Ten of Ten of One of Zero
comment One
```

> **Note**:
>
> In a normal programming language, all arguments to a function get evaluated before the function is called with the resulting values. This would make these control flow structures quite useless, as both arms would be executed, regardless of which one is chosen in the end. Therefore Marble employs [Lazy Evaluation](https://en.wikipedia.org/wiki/Lazy_evaluation), meaning values/expressions are only computed when their result is really needed, and values that are never used aren't executed either. Values are only ever forcefully evaluated when they are the return value of the program or the left hand side argument of a function call (when they are being called).

#### Looping

Looping (or repeated execution in general) is possible through the use of [combinators](https://en.wikipedia.org/wiki/Fixed-point_combinator), e. g. the Y-Combinator. The Y Combinator provides a function with a reference to itself, so that it can call itself recursively. Here is an example implementation of the recursive factorial function using the Y-Combinator:
```
let Y be fn F do
    let G be fn X do 
        F of do X of X end 
    end in
    G of G
end in

let Fact be fn Fact N do
    Is of N of Zero of do
        One
    end of do
        Mul of N of do Fact of do Sub of N of One end end
    end
end in

let Fact be Y of Fact in

Fact of Five
comment 120
```

### Pro Tips
- You can also use let and curried functions, to create functions, where one argument is already defined: 
    ```
    let AddThree be Add of Three in
    AddThree of Three
    ```

- If you don't like the number literals, you can just assign the ones you would like to use:
    ```
    let 3 be Three in
    Add of 3 of 3
    ```
- If you don't like the operator functions, you can just assign the ones you would like to use:
    ```
    let + be Add in
    + of Three of Three
    ```

### Design Notes

Through the implemention of ```let``` and ```then``` through function calls, every Marble program is only a single expression, comprised of only literals, functions and calls.

## Editor Support

A VS Code Extension for syntax hightlighting is available under ```/marble-language-support```

## Running Marble Programs

The marble executable lives under ```imarble```. Nacvigate there first:
```sh
cd imarble
```

You can then build the project using cargo and enter an interactive REPL using:
```sh
cargo run -r
```

If you want to execute a file, you should pass the path to the file as an argument:
```sh
cargo run -r -- path/to/my/file.mrbl
```

## Planned Improvements
- Error Messages: Currenly error messages are pretty minimal, and lack context such as line numbers or suggested fixes.
- Performance: The current implementation is pretty slow.