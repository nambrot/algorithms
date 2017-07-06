module Main exposing (..)

import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (on, onClick)
import Json.Decode as Decode


main =
    Html.program
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        }


type alias Model =
    { mouse : Position
    , points : List Position
    , tree : Maybe KdTree
    }


init : ( Model, Cmd Msg )
init =
    ( { mouse = ( 0, 0 )
      , points = []
      , tree = Nothing
      }
    , Cmd.none
    )


type alias Position =
    ( Int, Int )


type Msg
    = UpdateMousePosition Position
    | AddPoint


view : Model -> Html Msg
view model =
    body
        [ on "mousemove" mouseMoveDecoder
        , onClick AddPoint
        ]
        [ h1 []
            [ text "KD Trees" ]
        , hr [] []
        , p []
            [ text <| "Mouse position is at x:" ++ (model.mouse |> x |> toString) ++ " and y: " ++ (model.mouse |> y |> toString) ]
        , div []
            (model.points |> List.map (blackPoint model.mouse))
        , div []
            [ (nearestPoint model.mouse model.tree |> Maybe.map (redPoint model.mouse)) |> Maybe.withDefault (text "No neighbor found") ]
        ]


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        UpdateMousePosition x ->
            ( { model | mouse = x }, Cmd.none )

        AddPoint ->
            ( { model | points = model.mouse :: model.points, tree = insert model.mouse True model.tree |> Just }, Cmd.none )


x : Position -> Int
x =
    Tuple.first


y =
    Tuple.second


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none


toPosition : Int -> Int -> Position
toPosition a b =
    ( a, b )


mouseMoveDecoder : Decode.Decoder Msg
mouseMoveDecoder =
    Decode.map2 toPosition (Decode.field "layerX" Decode.int) (Decode.field "layerY" Decode.int) |> Decode.map UpdateMousePosition


blackPoint =
    pointToHtml "black"


redPoint =
    pointToHtml "red"


pointToHtml : String -> Position -> Position -> Html Msg
pointToHtml color target point =
    div
        [ style
            [ ( "height", "10px" )
            , ( "width", "10px" )
            , ( "border-radius", "10px" )
            , ( "background-color", color )
            , ( "text-align", "center" )
            , ( "position", "absolute" )
            , ( "left", (point |> x |> toString) ++ "px" )
            , ( "top", (point |> y |> toString) ++ "px" )
            ]
        ]
        [ text <| toString <| dist point target ]


type KdTree
    = Node Bool Position (Maybe KdTree) (Maybe KdTree)


isLeft : KdTree -> Position -> Bool
isLeft (Node isXNode point _ _) target =
    if isXNode then
        x point > x target
    else
        y point > y target


insert : Position -> Bool -> Maybe KdTree -> KdTree
insert target isXDirection tree =
    case tree of
        Nothing ->
            Node isXDirection target Nothing Nothing

        Just ((Node isXNode point left right) as node) ->
            case ( point == target, isLeft node target ) of
                ( True, _ ) ->
                    node

                ( _, True ) ->
                    Node isXNode point (insert target (not isXNode) left |> Just) right

                ( _, False ) ->
                    Node isXNode point left (insert target (not isXNode) right |> Just)


dist : Position -> Position -> Int
dist a b =
    (y b - y a) ^ 2 + (x b - x a) ^ 2


canOtherSideBeatPoint : Position -> KdTree -> Int -> Bool
canOtherSideBeatPoint target ((Node isXDirection point _ _) as node) distance =
    if isXDirection then
        dist target ( x point, y target ) < distance
    else
        dist target ( x target, y point ) < distance


nearestPoint : Position -> Maybe KdTree -> Maybe Position
nearestPoint target tree =
    case tree of
        Nothing ->
            Nothing

        Just ((Node _ point _ _) as node) ->
            Just <| Tuple.first <| nearestPH target tree ( point, dist target point )



-- more iterative implementation
-- nearestPointH : Position -> KdTree -> Position -> Int -> Position
-- nearestPointH target (Node isXNode point left right as node) accPoint accDistance =
--        let
--           newCurrent1 = if (dist point target) < accDistance then point else accPoint
--           newDistance1 = dist target newCurrent1
--           newCurrent = if isLeft node target then nearestPointH target left newCurrent1 newDistance1 else nearestPointH target right newCurrent1 newDistance1
--           newDistance = dist target <| Maybe.withDefault (0, 0) <| newCurrent
--       in
--         if canOtherSideBeatPoint target node newDistance then
--           if isLeft node target then
--             nearestPointH target right newCurrent (Just newDistance)
--           else
--             nearestPointH target left newCurrent (Just newDistance)
--         else
--           newCurrent

-- pattern match implementation with explcit steps
-- type NearestPointStep
--     = Self
--     | Primary
--     | Secondary
-- nearestPH target step tree ( accPoint, accDistance ) =
--   case tree of
--     Nothing -> ( accPoint, accDistance )
--     Just (Node _ point left right as node) ->
--       case step of
--         Self ->
--           if dist point target < accDistance then
--               nearestPH target Primary tree ( point, dist point target )
--           else
--               nearestPH target Primary tree ( accPoint, accDistance )
--         Primary ->
--           if isLeft node target then
--               nearestPH target Self left ( accPoint, accDistance ) |> nearestPH target Secondary right
--           else
--               nearestPH target Self right ( accPoint, accDistance ) |> nearestPH target Secondary left
--         Secondary ->
--           if canOtherSideBeatPoint target node accDistance then
--               nearestPH target Self tree ( accPoint, accDistance )
--           else
--               ( accPoint, accDistance )

-- This is my last implementation of the neareat point method. It might be more concise, but it might not be the best.
-- Procedural at the end of the day is still most readable for most people
nearestPH : Position -> Maybe KdTree -> ( Position, Int ) -> ( Position, Int )
nearestPH target tree ( accPoint, accDistance ) =
    case tree of
        Nothing ->
            ( accPoint, accDistance )

        Just ((Node _ point left right) as node) ->
            let
                searchF =
                    nearestPH target

                primarySide =
                    if isLeft node target then
                        left
                    else
                        right

                secondarySide =
                    if isLeft node target then
                        right
                    else
                        left

                possiblyOverwriteAccWith ( a, b ) ( c, d ) =
                    if d < b then
                        ( c, d )
                    else
                        ( a, b )

                optionallySearch tree ( accPoint, accDistance ) =
                    let
                        needsSearch node =
                            if canOtherSideBeatPoint target node accDistance then
                                Just (searchF tree ( accPoint, accDistance ))
                            else
                                Nothing
                    in
                    Maybe.andThen needsSearch tree |> Maybe.withDefault ( accPoint, accDistance )
            in
            ( accPoint, accDistance )
                |> possiblyOverwriteAccWith ( point, dist point target ) -- check the current node
                |> searchF primarySide  -- search the primary side for candidates
                |> optionallySearch secondarySide -- only search the secondary side if its possible
