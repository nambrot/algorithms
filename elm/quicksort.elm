import Html exposing (..)
import Random exposing (..)

main =
  Html.program
    { init = init
    , view = view
    , update = update
    , subscriptions = subscriptions
    }

type alias Model =
  { unsortedArray: List Int
  }
init : (Model, Cmd Msg)
init =
  ({ unsortedArray = []
  }, Random.generate InitArray <| Random.list 10000 <| Random.int 0 10000)

type Msg =
  InitArray (List Int) |
  String

view : Model -> Html Msg
view model =
  div []
    [ h1 []
      [ text "Mergesort" ]
    , hr [] []
    , p []
      [ text "Original Array" ]
    , p []
      [ model.unsortedArray |> toString |> text ]
    , p []
      [ text "Built-in sort:" ]
    , p []
      [ model.unsortedArray |> List.sort |> toString |> text ]
    , p []
      [ text "Quick sort:" ]
    , p []
      [ model.unsortedArray |> quicksort |> toString |> text ]
    ]

update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
  case msg of
    InitArray a -> ({model | unsortedArray = a}, Cmd.none)
    _ -> (model, Cmd.none)

subscriptions : Model -> Sub Msg
subscriptions model =
  Sub.none

quicksort: List Int -> List Int
quicksort list =
  case list of
    [] -> []
    [a] -> [a]
    head :: tail ->
      let
        -- its too much of a pain to select a random pivot
        smallers = List.filter (\n -> n <= head) list |> without head |> quicksort
        largers = List.filter (\n -> n > head) list |> quicksort
      in
        smallers ++ [head] ++ largers

without: Int -> List Int -> List Int
without el list =
  case list of
    head :: tail -> if el == head then tail else head :: (without el tail)
    _ -> []

