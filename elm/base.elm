import Html exposing (..)

main =
  Html.program
    { init = init
    , view = view
    , update = update
    , subscriptions = subscriptions
    }

type alias Model =
  {}
init : (Model, Cmd Msg)
init =
  ({}, Cmd.none)


type Msg = String

view : Model -> Html Msg
view model =
  div []
    [ h1 []
      [ text "Im your title" ]
    , hr [] []
    , p []
      [ text "Im your paragraph" ]
    ]

update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
  case msg of
    _ -> (model, Cmd.none)

subscriptions : Model -> Sub Msg
subscriptions model =
  Sub.none
