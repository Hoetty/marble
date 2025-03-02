var examples = {
    "Factorial": `let Y be fn F do
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
`
}