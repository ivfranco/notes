module Main where

import Lib (Prop(..), printTruthTable)
import Control.Monad (forM_)

main = forM_ props $ \p -> do
    printTruthTable p
    putStrLn ""

props :: [Prop]
props = 
    let
        p = Var 'P'
        q = Var 'Q'
        r = Var 'R'
        s = Var 'S'
    in
        -- 3g
        [ And (Or p (Not q)) r
        -- 3h
        , Or (Not p) (Not q)
        -- 3i
        , And p (Or q r)
        -- 3j
        , Or (And p q) (And p r)
        -- 3k
        , And p p
        -- 3l
        , Or (And p q) (And r (Not s))]