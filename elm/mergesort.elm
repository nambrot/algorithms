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
  }, Random.generate InitArray <| Random.list 100 <| Random.int 0 1000)

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
      [ text "Merge sort:" ]
    , p []
      [ model.unsortedArray |> mergesort |> toString |> text ]

    ]

update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
  case msg of
    InitArray a -> ({model | unsortedArray = a}, Cmd.none)
    _ -> (model, Cmd.none)

subscriptions : Model -> Sub Msg
subscriptions model =
  Sub.none

mergesort: List Int -> List Int
mergesort list =
  case list of
    [a] -> list
    [] -> []
    _ ->
      let
        len = (List.length list) // 2
        left = list |> List.take len |> mergesort
        right = list |> List.drop len |> mergesort
      in
        merge left right

merge: List Int -> List Int -> List Int
merge left right =
  case (left, right) of
    (lh :: ltail, rh :: rtail) ->
      if lh < rh then
        lh :: (merge ltail right)
      else
        rh :: (merge left rtail)
    (_, []) -> left
    ([], _) -> right
