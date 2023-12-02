open System
open System.IO
open FParsec

type Draw =
    | Red of int
    | Green of int
    | Blue of int
    
let colour =
    choice [|
        pstring "red" >>% Red
        pstring "green" >>% Green
        pstring "blue" >>% Blue
    |]

let number =
    manyChars digit
    |>> Int32.Parse

let gameNum =
    skipString "Game" >>. spaces >>. skipMany digit >>. skipChar ':' >>. spaces

/// 5 red -> Red 5
let colourCount =
    spaces >>. number .>> skipChar ' ' .>>. colour .>> spaces
    |>> fun (count, colour) -> colour count

/// 5 red, 4 blue -> [ Red 5; Blue 4 ]
let colourCounts = 
    sepBy colourCount (spaces >>. skipChar ',' >>. spaces)

/// 5 red, 4 blue; 1 green, 2 red -> [ [ Red 5; Blue 4 ]; [ Green 1; Red 2 ] ]
let draws =
    sepBy colourCounts (spaces >>. skipChar ';' >>. spaces)
    
/// Game 1: 5 red, 4 blue; 1 green, 2 red -> [ [ Red 5; Blue 4 ]; [ Green 1; Red 2 ] ]
let parser =
    gameNum >>. draws

File.ReadLines "input.txt"
|> Seq.sumBy (fun line ->
    match run parser line with
    | Success (draws, _, _) ->
        let r, g, b =
            draws
            |> Seq.concat
            |> Seq.fold
                (fun (r, g, b) draw ->
                    match draw with
                    | Red x -> max r x, g, b
                    | Green x -> r, max g x, b
                    | Blue x -> r, g, max b x)
                (0, 0, 0)
        r * g * b
    | Failure (err, _, _) ->
        failwith $"%s{err}"
    )
|> printfn "%i"
