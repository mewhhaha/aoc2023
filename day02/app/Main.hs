{-# LANGUAGE OverloadedStrings #-}

module Main where

import Data.Bifunctor (Bifunctor (bimap, first))
import Data.Char (isDigit, isSpace)
import Data.Maybe (Maybe (Just, Nothing), catMaybes, mapMaybe)
import Data.Text as Text
import Data.Text.IO (getContents)
import GHC.List as List
import Prelude (Bool, Eq, Foldable (minimum), IO, Int, Read, Show (show), error, filter, fmap, fst, not, print, read, snd, sum, ($), (*), (+), (.), (<$>), (<=), (==))

data Color = Red | Green | Blue
  deriving (Eq, Show)

type Bag = [Color]

type Round = [(Int, Color)]

type Game = (Int, [Round])

tread :: (Read a) => Text -> a
tread = read . Text.unpack

parse :: Text -> Game
parse = bimap parseGame (parseRounds . Text.tail) . Text.breakOn ":" . Text.filter (not . isSpace)
  where
    parseGame = tread . Text.filter isDigit
    parseRounds = fmap parseRound . Text.splitOn ";"
    parseRound = fmap (bimap tread parseColor . Text.span isDigit) . Text.splitOn ","
    parseColor "green" = Green
    parseColor "red" = Red
    parseColor "blue" = Blue

part1 :: [Game] -> IO ()
part1 games = do
  let value = List.sum . fmap fst . List.filter possible $ games
  print $ "Part 1: " ++ show value
  where
    possible :: Game -> Bool
    possible (_, rs) = List.all (List.all enough) rs
    enough (count, Red) = count <= 12
    enough (count, Green) = count <= 13
    enough (count, Blue) = count <= 14

find :: (a -> Bool) -> [a] -> Maybe a
find p [] = Nothing
find p (c : cs) = if p c then Just c else Main.find p cs

part2 :: [Game] -> IO ()
part2 games = do
  let value = List.sum . fmap power $ games
  print $ "Part 2: " ++ show value
  where
    power :: Game -> Int
    power (_, rs) = count Red rs * count Green rs * count Blue rs
    count :: Color -> [Round] -> Int
    count c = List.maximum . fmap fst . mapMaybe (Main.find (\(_, c') -> c == c'))

main :: IO ()
main = do
  games <- fmap parse . lines <$> getContents
  part1 games
  part2 games
