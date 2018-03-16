module Lib where

import Data.List (nub, sort)
import Text.Printf (printf)
import Control.Monad (liftM2)

type Symbol = Char

data Prop = 
      T
    | F
    | Var Symbol
    | Not Prop
    | And Prop Prop
    | Or Prop Prop

instance Show Prop where
    show T = "T"
    show F = "F"
    show (Var s) = [s]
    show (Not p) = "~" ++ show p
    show (And p1 p2) = printf "(%s) ∧ (%s)" (show p1) (show p2)
    show (Or p1 p2) = printf "(%s) ∨ (%s)" (show p1) (show p2)

onVar :: (Symbol -> Prop) -> Prop -> Prop
onVar _ T = T
onVar _ F = F
onVar f (Var s) = f s
onVar f (Not p) = Not (onVar f p)
onVar f (And p1 p2) = And (onVar f p1) (onVar f p2)
onVar f (Or p1 p2) = Or (onVar f p1) (onVar f p2)

-- rewrite a single variable with a proposition
rewrite :: Symbol -> Prop -> Prop -> Prop
rewrite s t = onVar f
    where
        f s' | s' == s = t
        f s' = Var s'

rewriteAll :: [(Symbol, Prop)] -> Prop -> Prop
rewriteAll pairs p = foldl (\p (s, t) -> rewrite s t p) p pairs

-- collect all symbols in a proposition
collect :: Prop -> [Symbol]
collect T = []
collect F = []
collect (Var s) = [s]
collect (Not p) = collect p
collect (And p1 p2) = collect p1 ++ collect p2
collect (Or p1 p2) = collect p1 ++ collect p2

-- all distinct symbols in a proposition
symbols :: Prop -> [Symbol]
symbols = sort . nub . collect

boolToProp :: Bool -> Prop
boolToProp True = T
boolToProp False = F

evaluate :: Prop -> Either String Bool
evaluate T = return True
evaluate F = return False
evaluate (Var s) = Left $ "Error: Raw symbol " ++ show s
evaluate (Not p) = fmap not (evaluate p)
evaluate (And p1 p2) = liftM2 (&&) (evaluate p1) (evaluate p2)
evaluate (Or p1 p2) = liftM2 (||) (evaluate p1) (evaluate p2)

combinations :: Int -> [[Prop]]
combinations 0 = [[]]
combinations n =
    let prev = combinations (n - 1)
    in fmap (T:) prev ++ fmap (F:) prev

titleLine :: [Symbol] -> Prop -> String
titleLine ss p = unwords (fmap return ss) ++ " " ++ show p

valueLine :: [Symbol] -> Prop -> [Prop] -> String
valueLine ss p combo = case evaluate (rewriteAll (zip ss combo) p) of
    Left err -> err
    Right tf -> unwords $ fmap show (combo ++ [boolToProp tf])

printTruthTable :: Prop -> IO ()
printTruthTable p = do
    let ss = symbols p
    putStrLn $ titleLine ss p
    mapM_ (putStrLn . valueLine ss p) (combinations (length ss))