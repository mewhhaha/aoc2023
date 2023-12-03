{-# LANGUAGE NamedFieldPuns #-}
{-# LANGUAGE OverloadedStrings #-}

module Main where

import Data.Bifunctor (Bifunctor (bimap, first))
import Data.Char (isDigit, isSpace)
import Data.Maybe (Maybe (Just, Nothing), catMaybes, mapMaybe)
import Data.Monoid as Monoid (Monoid, mappend, mempty)
import Data.Text as Text
import Data.Text.IO (getContents)
import GHC.List as List
import Prelude (Bool, Eq, Foldable (minimum), IO, Int, Read, Semigroup, Show (show), error, filter, fmap, fst, max, not, print, read, snd, sum, ($), (&&), (*), (+), (.), (<$>), (<=), (<>), (==))

data Round = Round
  { red :: Int,
    green :: Int,
    blue :: Int
  }
  deriving (Eq, Show)

applyRound f (Round r1 g1 b1) (Round r2 g2 b2) = Round (f r1 r2) (f g1 g2) (f b1 b2)

type Game = (Int, [Round])

tread :: (Read a) => Text -> a
tread = read . Text.unpack

parse :: Text -> Game
parse = bimap parseGame (parseRounds . Text.tail) . Text.breakOn ":" . Text.filter (not . isSpace)
  where
    parseGame = tread . Text.filter isDigit
    parseRounds = fmap parseRound . Text.splitOn ";"
    parseRound = List.foldl1 (applyRound (+)) . (fmap (parseColor . Text.span isDigit) . Text.splitOn ",")
    parseColor (n, "green") = Round {red = 0, green = tread n, blue = 0}
    parseColor (n, "red") = Round {red = tread n, green = 0, blue = 0}
    parseColor (n, "blue") = Round {red = 0, green = 0, blue = tread n}

part1 :: [Game] -> IO ()
part1 games = do
  let value = List.sum . fmap fst . List.filter possible $ games
  print $ "Part 1: " ++ show value
  where
    possible :: Game -> Bool
    possible (_, rs) = List.all enough rs
    enough Round {red, green, blue} = red <= 12 && green <= 13 && blue <= 14

find :: (a -> Bool) -> [a] -> Maybe a
find p [] = Nothing
find p (c : cs) = if p c then Just c else Main.find p cs

part2 :: [Game] -> IO ()
part2 games = do
  let value = List.sum . fmap power $ games
  print $ "Part 2: " ++ show value
  where
    power :: Game -> Int
    power (_, rs) =
      let Round {red, green, blue} = List.foldl' (applyRound max) (Round 0 0 0) rs
       in red * green * blue

main :: IO ()
main = do
  games <- fmap parse . lines <$> getContents
  part1 games
  part2 games
