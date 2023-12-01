open System
open System.IO

File.ReadLines "input.txt"
|> Seq.sumBy (fun line ->
    let first = line |> Seq.find Char.IsDigit
    let last = line |> Seq.findBack Char.IsDigit
    int (first - '0') * 10 + int (last - '0'))
|> printfn "%i"

