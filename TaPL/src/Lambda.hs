{-# LANGUAGE RecordWildCards #-}

module Lambda where

import           Text.Printf (printf)

data Term
    = TmVar
        { var  :: Int
        , size :: Int }
    | TmAbs
        { bound :: String
        , body  :: Term }
    | TmApp
        { func :: Term
        , para :: Term }
    | TmLet String Term Term

newtype Context = Context
    { unContext :: [(String, Binding)] }

data Binding = NameBind

ctxLength :: Context -> Int
ctxLength Context{..} = length unContext

index2name :: Context -> Int -> String
index2name Context{..} idx = fmap fst unContext !! idx

insertName :: String -> Context -> Context
insertName x (Context bs) = Context $ (x, NameBind) : bs

pickFreshName :: Context -> String -> (Context, String)
pickFreshName ctx@Context{..} x =
    if x `elem` fmap fst unContext
        then pickFreshName ctx (x ++ "'")
        else (insertName x ctx, x)

printTm :: Context -> Term -> String
printTm ctx TmAbs{..} =
    let (ctx', x') = pickFreshName ctx bound
    in printf "(lambda %s. %s)" x' (printTm ctx' body)
printTm ctx TmApp{..} = printf "(%s %s)" (printTm ctx func) (printTm ctx para)
printTm ctx TmVar{..}
    | ctxLength ctx == size = index2name ctx var
    | otherwise = "[bad index]"

termShift :: Int -> Term -> Term
termShift d t = walk 0 t
    where
        walk c t@TmVar{..}
            | var >= c = TmVar (var + d) (size + d)
            | otherwise = t
        walk c TmApp{..} = TmApp (walk c func) (walk c para)
        walk c TmAbs{..} = TmAbs bound (walk (c+1) body)

termSubst :: Int -> Term -> Term -> Term
termSubst j s t = walk 0 t
    where
        walk c t@TmVar{..}
            | var == c + j = termShift c s
            | otherwise = t
        walk c TmApp{..} = TmApp
            { func = walk c func
            , para = walk c para }
        walk c t@TmAbs{..} = t { body = walk (c+1) body }

termSubstTop :: Term -> Term -> Term
termSubstTop s t = termShift (-1) (termSubst 0 (termShift 1 s) t)

isVal :: Context -> Term -> Bool
isVal _ TmAbs{..} = True
isVal _ _         = False

data NoRuleApplies = NoRuleApplies

noRuleApplies :: Either NoRuleApplies a
noRuleApplies = Left NoRuleApplies

eval1 :: Context -> Term -> Either NoRuleApplies Term
eval1 ctx (TmApp (TmAbs _ t12) v2) | isVal ctx v2 = return $ termSubstTop v2 t12
eval1 ctx (TmApp v1 t2) | isVal ctx v1 = do
    t2' <- eval1 ctx t2
    return $ TmApp v1 t2'
eval1 ctx (TmApp t1 t2) = do
    t1' <- eval1 ctx t1
    return $ TmApp t1' t2
eval1 ctx (TmLet x v1 t2) | isVal ctx v1 = return $ termSubst (find x ctx) v1 t2
eval1 ctx (TmLet x t1 t2) = do
    t1' <- eval1 ctx t1
    return $ TmLet x t1' t2
eval1 _ _ = noRuleApplies

find :: String -> Context -> Int
find = undefined

eval :: Context -> Term -> Term
eval ctx t = case eval1 ctx t of
    Left _   -> t
    Right t' -> eval ctx t'

evalB :: Context -> Term -> Term
evalB ctx t@TmApp{..} = case evalB ctx func of
    TmAbs _ t12 -> evalB ctx $ termSubstTop (evalB ctx para) t12
    _           -> t
evalB _ t = t
