open System
open System.IO

let numStrings = [|
    "one"
    "two"
    "three"
    "four"
    "five"
    "six"
    "seven"
    "eight"
    "nine"
|]

let rec findFirstNumber (str: ReadOnlyMemory<char>) =
    if Char.IsDigit str.Span[0] then
        int (str.Span[0] - '0')
    else
        match numStrings |> Array.tryFindIndex (fun x -> str.Span.StartsWith(x)) with
        | Some i ->
            i + 1
        | None ->
            findFirstNumber (str.Slice(1))

let rec findLastNumber (str: ReadOnlyMemory<char>) =
    if Char.IsDigit str.Span[str.Length - 1] then
        int (str.Span[str.Length - 1] - '0')
    else
        match numStrings |> Array.tryFindIndex (fun x -> str.Span.EndsWith(x)) with
        | Some i ->
            i + 1
        | None ->
            findLastNumber (str.Slice(0, str.Length - 1))

File.ReadLines "input.txt"
|> Seq.sumBy (fun line ->
    let first = findFirstNumber (line.AsMemory())
    let last = findLastNumber (line.AsMemory())
    first * 10 + last)
|> printfn "%i"

