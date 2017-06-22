module Main exposing (..)

import Array exposing (Array)
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (on, onClick)
import Html.Events.Extra exposing (targetValueIntParse)
import Json.Decode as Json
import String


main =
    Html.program
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        }


type alias Model =
    { data : UnionFind
    , a : Int
    , b : Int
    }


init : ( Model, Cmd Msg )
init =
    ( { data = construct 20
      , a = 0
      , b = 1
      }
    , Cmd.none
    )


type alias UnionFind =
    Array Int


type Msg
    = UpdateA Int
    | UpdateB Int
    | UnionAAndB


construct : Int -> UnionFind
construct n =
    Array.initialize n <| \n -> n


view : Model -> Html Msg
view model =
    div []
        [ h1 []
            [ text "UnionFind" ]
        , hr [] []
        , h3 []
            [ text "Datastructure" ]
        , table []
            [ thead []
                [ tr []
                    (model |> itemsFromModel |> List.map tdToHtml)
                ]
            , tbody []
                [ tr [] <| Array.toList <| Array.map tdToHtml <| model.data ]
            ]
        , h3 []
            [ text "Input" ]
        , ul []
            [ li []
                [ label []
                    [ text "a"
                    , select
                        [ value <| toString <| model.a
                        , on "change" (Json.map UpdateA targetValueIntParse)
                        ]
                        (model |> itemsFromModel |> List.map (\n -> option [ value <| toString <| n, selected (n == model.a) ] [ text <| toString <| n ]))
                    ]
                ]
            , li []
                [ label []
                    [ text "b"
                    , select
                        [ value <| toString <| model.b
                        , on "change" (Json.map UpdateB targetValueIntParse)
                        ]
                        (model |> itemsFromModel |> List.map (\n -> option [ value <| toString <| n, selected (n == model.b) ] [ text <| toString <| n ]))
                    ]
                ]
            ]
        , h3 []
            [ text <| "Are " ++ toString model.a ++ " and " ++ toString model.b ++ " connected?" ]
        , p []
            [ text <| toString <| connected model.data model.a model.b ]
        , h3 []
            [ text "Or connect them now" ]
        , p []
            [ button [ onClick UnionAAndB ] [ text "Union" ] ]
        ]


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        UpdateA number ->
            ( { model | a = number }, Cmd.none )

        UpdateB number ->
            ( { model | b = number }, Cmd.none )

        UnionAAndB ->
            ( { model | data = union model.data model.a model.b }, Cmd.none )


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none


find : UnionFind -> Int -> Int
find data a =
    case Array.get a data of
        Just b ->
            if a == b then
                a
            else
                find data b

        _ ->
            0


connected : UnionFind -> Int -> Int -> Bool
connected data a b =
    find data a == find data b


union : UnionFind -> Int -> Int -> UnionFind
union data a b =
    let
        parent =
            Array.get b data |> Maybe.withDefault 0

        newRoot =
            find data a
    in
    if parent == b then
        Array.set b newRoot data
    else
        union data a parent |> Array.set b newRoot


tdToHtml : Int -> Html msg
tdToHtml =
    toString
        >> text
        >> List.singleton
        >> td
            [ style
                [ ( "padding", "0px 9px" )
                , ( "text-align", "center" )
                ]
            ]


itemsFromModel : Model -> List Int
itemsFromModel model =
    (Array.length model.data - 1) |> List.range 0
