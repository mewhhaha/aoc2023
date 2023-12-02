{-# LANGUAGE OverloadedStrings #-}

module Main where

import Data.Bifunctor (Bifunctor (bimap))
import Data.Char (isDigit, isSpace)
import Data.Text as Text
import Data.Text.IO (getContents)
import GHC.List as List
import Prelude (Bool, Eq, IO, Int, Read, Show (show), error, filter, fmap, fst, not, print, read, sum, ($), (.), (<$>), (<=))

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

red = 12

green = 13

blue = 14

part1 games = do
  let value = List.sum . fmap fst . List.filter possible $ games
  print $ "Part 1: " ++ show value
  where
    possible :: Game -> Bool
    possible (_, rs) = List.all (List.all enough) rs
    enough (count, Red) = count <= 12
    enough (count, Green) = count <= 13
    enough (count, Blue) = count <= 14

main :: IO ()
main = do
  games <- fmap parse . lines <$> getContents
  part1 games
