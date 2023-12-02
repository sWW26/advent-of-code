open System
open System.IO
open FParsec

type Draw =
    | Red of int
    | Green of int
    | Blue of int
    
[<Struct>]
type GameNumber = GameNumber of int

let colour =
    choice [|
        pstring "red" >>% Red
        pstring "green" >>% Green
        pstring "blue" >>% Blue
    |]

let number =
    manyChars digit
    |>> Int32.Parse

/// Game 1: -> GameNumber 1
let gameNum =
    skipString "Game" >>. spaces >>. number .>> skipChar ':' .>> spaces |>> GameNumber

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
    
/// Game 1: 5 red, 4 blue; 1 green, 2 red -> (GameNumber 1, [ [ Red 5; Blue 4 ]; [ Green 1; Red 2 ] ])
let parser =
    gameNum .>>. draws

File.ReadLines "input.txt"
|> Seq.sumBy (fun line ->
    match run parser line with
    | Success ((GameNumber gameNumber, draws), _, _) ->
        let hasInvalidDraw =
            draws
            |> Seq.concat
            |> Seq.exists (function
                | Red x when x > 12 -> true
                | Green x when x > 13 -> true
                | Blue x when x > 14 -> true
                | _ -> false)
        if hasInvalidDraw then 0 else gameNumber
    | Failure (err, _, _) ->
        failwith $"%s{err}"
    )
|> printfn "%i"
