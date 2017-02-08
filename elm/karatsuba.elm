module Greeting exposing (..)
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (onInput)
-- MAIN
main =
  Html.program
    { init = init
    , view = view
    , update = update
    , subscriptions = subscriptions
    }

-- MODEL
type alias Model =
  { a: Int
  , b: Int
  }
init : (Model, Cmd Msg)
init =
  ({ a = 1
   , b = 1
   }, Cmd.none)

-- VIEW
view : Model -> Html Msg
view model =
  div []
    [ h1 [] [(text "Karatsuba algorithm")]
      ,label []
        [ text "a:   "
        , input [ type_ "text"
                , value (toString model.a)
                , onInput ChangeA] []
        ]
      , br [] []
      ,label []
        [ text "b:   "
        , input [ type_ "text"
                , value (toString model.b)
                , onInput ChangeB] []
        ]
      , br [] []
      ,text ("The product of a times b is " ++ toString (model.a * model.b))
      , br [] []
      ,text ("Karutsba: " ++ (toString (karatsuba model.a model.b)))
    ]

toInt: String -> Int
toInt string = string |> String.toInt |> Result.withDefault 0

type Msg
    = ChangeA String
    | ChangeB String
update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
  case msg of
    ChangeA a ->
      ({ model | a = toInt a}, Cmd.none)
    ChangeB b ->
      ({ model | b = toInt b}, Cmd.none)

subscriptions : Model -> Sub Msg
subscriptions model =
  Sub.none

karatsuba: Int -> Int -> Int
karatsuba x y =
  case (x < 10, y < 10) of
    (True, True) -> x * y
    (_, _) ->
      let
        n = Basics.logBase 10 (toFloat (Basics.max x y)) |> Basics.ceiling
        m = Basics.floor (toFloat(n) / 2)
        x0 = x % (10 ^ m)
        x1 = (x - x0) // 10 ^ m
        y0 = y % (10 ^ m)
        y1 = (y - y0) // 10 ^ m
        z2 = karatsuba x1 y1
        z1 = (karatsuba x1 y0) + (karatsuba x0 y1)
        z0 = karatsuba x0 y0
      in
        z2 * 10 ^ (2 * m) + z1 * 10 ^ m + z0


