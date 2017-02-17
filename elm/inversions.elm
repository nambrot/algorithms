import Html exposing (..)
import Random

main =
  Html.program
    { init = init
    , view = view
    , update = update
    , subscriptions = subscriptions
    }

type alias Model =
  { array: List Int
  }
init : (Model, Cmd Msg)
init =
  ({ array = []
  }, Random.generate InitArray <| Random.list 5 <| Random.int 0 20)


type Msg =
  InitArray (List Int)

view : Model -> Html Msg
view model =
  let
    (count, sorted) = numOfInversions model.array
  in
    div []
      [ h1 []
        [ text "Count number of inversions" ]
      , hr [] []
      , p []
        [ model.array |> toString |> text ]
      , p []
        [ text ("The number of inversions is: " ++ (toString count) ++ " and the sorted array is: " ++ (toString sorted))]
      ]

update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
  case msg of
    InitArray a -> ({model | array = a}, Cmd.none)

subscriptions : Model -> Sub Msg
subscriptions model =
  Sub.none

numOfInversions: List Int -> (Int, List Int)
numOfInversions list =
  case list of
    [] -> (0, [])
    [a] -> (0, [a])
    _ ->
      let
        len = (List.length list) // 2
        (leftInversions, leftSorted) = list |> List.take len |> numOfInversions
        (rightInversions, rightSorted) = list |> List.drop len |> numOfInversions
      in
        mergeHalves leftSorted rightSorted (leftInversions + rightInversions) []

mergeHalves: List Int -> List Int -> Int -> List Int -> (Int, List Int)
mergeHalves left right accNum accList =
  case (left, right) of
    (lh :: ltail, rh :: rtail) ->
      if lh <= rh then
        mergeHalves ltail right accNum (accList ++ [lh])
      else
        mergeHalves left rtail (accNum + (List.length left)) (accList ++ [rh])
    (_, []) -> (accNum, accList ++ left)
    ([], _) -> (accNum, accList ++ right)

