module Arithmatics where

data Term
    = TmTrue
    | TmFalse
    | TmIf Term Term Term
    | TmZero
    | TmSucc Term
    | TmPred Term
    | TmIsZero Term
    deriving (Show, Eq)

isNumerical :: Term -> Bool
isNumerical TmZero     = True
isNumerical (TmSucc t) = isNumerical t
isNumerical _          = False

isVal :: Term -> Bool
isVal TmTrue  = True
isVal TmFalse = True
isVal t       = isNumerical t

data NoRuleApplies = NoRuleApplies
    deriving (Show, Eq)

noRuleApplies :: Either NoRuleApplies a
noRuleApplies = Left NoRuleApplies

eval1 :: Term -> Either NoRuleApplies Term
eval1 (TmIf TmTrue t2 _) = return t2
eval1 (TmIf TmFalse _ t3) = return t3
eval1 (TmIf t1 t2 t3) = do
    t1' <- eval1 t1
    return $ TmIf t1' t2 t3
eval1 (TmSucc t) = do
    t' <- eval1 t
    return $ TmSucc t'
eval1 (TmPred TmZero) = return TmZero
eval1 (TmPred (TmSucc nv)) | isNumerical nv = return nv
eval1 (TmPred t) = do
    t' <- eval1 t
    return $ TmPred t'
eval1 (TmIsZero TmZero) = return TmTrue
eval1 (TmIsZero (TmSucc nv)) | isNumerical nv = return TmFalse
eval1 (TmIsZero t) = do
    t' <- eval1 t
    return $ TmIsZero t'
eval1 _ = noRuleApplies

eval :: Term -> Term
eval t = case eval1 t of
    Left _   -> t
    Right t' -> eval t'

evalB :: Term -> Either NoRuleApplies Term
evalB t | isVal t = return t
evalB (TmIf t1 t2 t3) = do
    v1 <- evalB t1
    case v1 of
        TmTrue  -> evalB t2
        TmFalse -> evalB t3
        _       -> noRuleApplies
evalB (TmSucc t) = do
    v <- evalB t
    if isNumerical v
        then return $ TmSucc v
        else noRuleApplies
evalB (TmPred t) = do
    v <- evalB t
    case v of
        TmZero    -> return TmZero
        TmSucc v' | isNumerical v' -> return v'
        _         -> noRuleApplies
evalB (TmIsZero t) = do
    v <- evalB t
    case v of
        TmZero    -> return TmTrue
        TmSucc v' | isNumerical v' -> return TmFalse
        _         -> noRuleApplies
