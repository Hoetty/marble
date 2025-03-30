var examples = {
    "Factorial": `let Y be Import of str lang/y ing in

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
`,

    "Arithmetic": `let 6 be Add of Three of Three in
let 25 be Sub of ThirtyOne of 6 in
let 100 be Mul of 25 of Four in
Div of 100 of Ten

comment 10
`,

    "Hello World": `PrintLn of str Hello World ing then Unit
`,
    "Simple Fibonacci": `let Y be Import of str lang/y ing in

let Fib be fn Fib N do
    Or of do Is of N of One end of do Is of N of Two end of do
        One
    end of do
        Add of do Fib of do Sub of N of One end end of do Fib of do Sub of N of Two end end
    end
end in

let Fib be Y of Fib in

Fib of Fourteen

comment 377
`,
    "Linear Fibonacci": `let Y be Import of str lang/y ing in

let Fib be fn Fib N do
    Is of N of Two of do
        Tuple of One of One
    end of do
        let Previous be Fib of do Sub of N of One end in
        Tuple of do Add of do TFirst of Previous end of do TSecond of Previous end end of do TFirst of Previous end
    end
end in

let Fib be Y of Fib in
let Fib be fn N do
    TFirst of do Fib of N end
end in

Fib of Fifty
comment 12586269025
`
}