module Main where

import           Control.Monad      (foldM)
import           Data.Char          (isAlpha)
import           Data.List          (find)
import           System.Environment (getArgs)
import           Text.Printf        (printf)

main :: IO ()
main = do
    raw : _ <- getArgs
    parseIO raw

parseIO :: String -> IO ()
parseIO raw = case parse raw of
    Right expr -> print expr
    Left err   -> putStrLn err

data Expr
    = Var Char
    | Astr Expr
    | Concat Expr Expr
    | Add Expr Expr

instance Show Expr where
    show (Var c)        = [c]
    show (Astr e)       = printf "%s*" (show e)
    show (Concat e1 e2) = printf "(%s.%s)" (show e1) (show e2)
    show (Add e1 e2)    = printf "(%s+%s)" (show e1) (show e2)

data Seg
    = Seg Char
    | Expr Expr
    deriving (Show)

isOperator :: Char -> Bool
isOperator c = c `elem` "+.*"

isOperatorSeg :: Seg -> Bool
isOperatorSeg (Seg c) = isOperator c
isOperatorSeg _       = False

isHigherOp :: Char -> Char -> Bool
isHigherOp '*' _ = True
isHigherOp '.' c = c == '+'
isHigherOp '+' _ = False

isExprOrUnary :: Seg -> Bool
isExprOrUnary (Seg '*') = True
isExprOrUnary (Expr _)  = True
isExprOrUnary _         = False

higher :: Char -> [Seg] -> Bool
higher c stack = case find isOperatorSeg stack of
    Just (Seg op) -> isHigherOp c op
    Nothing       -> True

parseError :: String -> [Seg] -> Either String a
parseError err stack = Left (printf "Error: %s\nStack: %s" err (show stack))

parse :: String -> Either String Expr
parse s = case fmap reduceAll (foldM go [] s) of
    Right [Expr e] -> Right e
    Right stack     -> parseError "Did not reduce to a single expression" stack
    Left err       -> Left err
    where
        go stack '(' = Right $ Seg '(' : stack
        go stack ')' = reduceParen (Seg ')' : stack)
        go stack c | isOperator c = if higher c stack
            then Right (Seg c : stack)
            else do
                stack' <- reduce stack
                go stack' c
        go stack c | isAlpha c = case stack of
            seg : _ | isExprOrUnary seg -> do
                stack' <- go stack '.'
                Right (Expr (Var c) : stack')
            _ -> Right (Expr (Var c) : stack)

reduce :: [Seg] -> Either String [Seg]
reduce (Seg '*' : Expr e : stack)            = Right (Expr (Astr e) : stack)
reduce (Expr e1 : Seg '.' : Expr e2 : stack) = Right (Expr (Concat e2 e1) : stack)
reduce (Expr e1 : Seg '+' : Expr e2 : stack) = Right (Expr (Add e2 e1) : stack)
reduce stack = parseError "No reduce rule applied" stack

reduceAll :: [Seg] -> [Seg]
reduceAll stack = case reduce stack of
    Left _       -> stack
    Right stack' -> reduceAll stack'

reduceParen :: [Seg] -> Either String [Seg]
reduceParen (Seg ')' : stack) = case reduceAll stack of
    Expr e : Seg '(' : stack' -> Right (Expr e : stack')
    stack'                    -> parseError "did not find ( following the single expression after reduce" stack'
reduceParen stack = parseError "did not find ( on the stack top" stack
