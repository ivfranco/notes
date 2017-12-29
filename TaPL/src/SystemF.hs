module SystemF where

import           Text.Printf (printf)

data Ty
    = TyVar Int Int
    | TyArr Ty Ty
    | TyAll String Ty
    | TySome String Ty
    deriving (Eq)

instance Show Ty where
    show (TyVar i _)       = show i
    show (TyArr tyT1 tyT2) = printf "(%s %s)" (show tyT1) (show tyT2)
    show (TyAll _ tyT)     = printf "(∀. %s)" (show tyT)
    show (TySome _ tyT)    = printf "(∃. %s)" (show tyT)



data Binding
    = NameBind
    | VarBind Ty
    | TyVarBind

type OnVar a = Int -> Int -> Int -> a
type OnType = Int -> Ty -> Ty

tymap :: OnVar Ty -> Int -> Ty -> Ty
tymap onvar c (TyVar x n) = onvar c x n
tymap onvar c (TyArr tyT1 tyT2) = TyArr (tymap onvar c tyT1) (tymap onvar c tyT2)
tymap onvar c (TyAll tyX tyT1) = TyAll tyX (tymap onvar (c+1) tyT1)
tymap onvar c (TySome tyX tyT1) = TySome tyX (tymap onvar (c+1) tyT1)

typeShiftAbove :: Int -> Int -> Ty -> Ty
typeShiftAbove c d = tymap onvar c
    where
        onvar c x n
            | x >= c = TyVar (x+d) (n+d)
            | otherwise = TyVar x (n+d)

typeShift :: Int -> Ty -> Ty
typeShift = typeShiftAbove 0

typeSubst :: Int -> Ty -> Ty -> Ty
typeSubst j tyS = tymap onvar 0
    where
        onvar c x n
            | x == c+j  = typeShift c tyS
            | otherwise = TyVar x n

typeSubstTop :: Ty -> Ty -> Ty
typeSubstTop tyS tyT = typeShift (-1) (typeSubst 0 tyS tyT)

data Term
    = TmVar Int Int
    | TmAbs String Ty Term
    | TmApp Term Term
    | TmTAbs String Term
    | TmTApp Term Ty
    | TmPack Ty Term Ty
    | TmUnpack String String Term Term
    deriving (Eq)

instance Show Term where
    show (TmVar i _) = show i
    show (TmAbs _ tyT t) = printf "(\\:%s. %s)" (show tyT) (show t)
    show (TmApp t1 t2) = printf "(%s %s)" (show t1) (show t2)
    show (TmTAbs _ t) = printf "(\\. %s)" (show t)
    show (TmTApp t tyT) = printf "%s[%s]" (show t) (show tyT)
    show (TmPack tyT1 t2 tyT3) = printf "{*%s, %s} as %s" (show tyT1) (show t2) (show tyT3)
    show (TmUnpack x y t1 t2) = printf "let {%s, %s} = %s in %s" x y (show t1) (show t2)

tmmap :: OnVar Term -> OnType -> Int -> Term -> Term
tmmap onvar ontype = walk
    where
        walk c (TmVar x n)     = onvar c x n
        walk c (TmAbs x tyT t) = TmAbs x (ontype c tyT) (walk (c+1) t)
        walk c (TmApp t1 t2) = TmApp (walk c t1) (walk c t2)
        -- since type and term free variables are defined on the SAME context
        walk c (TmTAbs tyX t) = TmTAbs tyX (walk (c+1) t)
        walk c (TmTApp t tyT) = TmTApp (walk c t) (ontype c tyT)
        walk c (TmPack tyT1 t2 tyT3) = TmPack (ontype c tyT1) (walk c t2) (ontype c tyT3)
        walk c (TmUnpack tyX x t1 t2) =
            TmUnpack tyX x (walk c t1) (walk (c+2) t2)

termShiftAbove :: Int -> Int -> Term -> Term
termShiftAbove c d = tmmap onvar ontype c
    where
        onvar c x n
            | x >= c = TmVar (x+d) (n+d)
            | otherwise = TmVar x (n+d)
        ontype = typeShiftAbove d

termShift :: Int -> Term -> Term
termShift = termShiftAbove 0

termSubst :: Int -> Term -> Term -> Term
termSubst j s = tmmap onvar ontype 0
    where
        onvar c x n
            | x == (c+j) = termShift c s
            | otherwise = TmVar x n
        ontype _ = id

termSubstTop :: Term -> Term -> Term
termSubstTop s t = termShift (-1) (termSubst 0 s t)

tytermSubst :: Int -> Ty -> Term -> Term
tytermSubst j tyS = tmmap onvar ontype j
    where
        onvar _ = TmVar
        ontype c = typeSubst c tyS

tytermSubstTop :: Ty -> Term -> Term
tytermSubstTop tyS t = termShift (-1) (tytermSubst 0 tyS t)

noRuleApplies :: Either String a
noRuleApplies = Left ""

type Context = [(String, Binding)]

getTypeFromContext :: Context -> Int -> Either String Ty
getTypeFromContext [] _ = typeError "Context exhausted"
getTypeFromContext ((_, b):_) 0 = case b of
    VarBind ty -> return ty
    _          -> typeError "Context mismatch: not a variable bind"
getTypeFromContext (_:bs) i
    | i < 0 = typeError "Scoping error: Nonsensical negative indices"
    | otherwise = getTypeFromContext bs (i-1)

addBinding :: Context -> String -> Binding -> Context
addBinding ctx x b = (x, b) : ctx

isval :: Context -> Term -> Bool
isval _ TmAbs{}  = True
isval _ TmTAbs{} = True
isval _ TmPack{} = True
isval _ _        = False

eval1 :: Context -> Term -> Either String Term
eval1 ctx (TmApp (TmAbs _ _ t1) v2) | isval ctx v2 = return $ termSubstTop v2 t1
eval1 ctx (TmApp v1 t2) | isval ctx v1 = do
    t2' <- eval1 ctx t2
    return $ TmApp v1 t2'
eval1 ctx (TmApp t1 t2) = do
    t1' <- eval1 ctx t1
    return $ TmApp t1' t2
eval1 ctx (TmTApp (TmTAbs _ t1) tyT2) = return $ tytermSubstTop tyT2 t1
eval1 ctx (TmTApp t1 tyT2) = do
    t1' <- eval1 ctx t1
    return $ TmTApp t1' tyT2
eval1 ctx (TmPack tyT1 t2 tyT3) = do
    t2' <- eval1 ctx t2
    return $ TmPack tyT1 t2' tyT3
eval1 ctx (TmUnpack _ _ (TmPack tyT1 v1 _) t2) | isval ctx v1 =
    return $ tytermSubstTop tyT1 (termSubstTop (termShift 1 v1) t2)
eval1 ctx (TmUnpack tyX x t1 t2) = do
    t1' <- eval1 ctx t1
    return $ TmUnpack tyX x t1' t2

typeError :: String -> Either String a
typeError = Left

typeof :: Context -> Term -> Either String Ty
typeof ctx (TmVar i _) = getTypeFromContext ctx i
typeof ctx (TmAbs x tyT1 t2) = do
    let ctx' = addBinding ctx x (VarBind tyT1)
    tyT2 <- typeof ctx' t2
    return $ TyArr tyT1 tyT2
typeof ctx (TmApp t1 t2) = do
    tyT1 <- typeof ctx t1
    tyT2 <- typeof ctx t2
    case tyT1 of
        TyArr tyT11 tyT12
            | tyT11 == tyT2 -> return tyT12
            | otherwise -> typeError "Type mismatch: parameter mismatch"
        _ -> typeError "Type mismatch: expected arrow type"
typeof ctx (TmTAbs tyX t1) = do
    let ctx' = addBinding ctx tyX TyVarBind
    tyT1 <- typeof ctx' t1
    return $ TyAll tyX tyT1
typeof ctx (TmTApp t1 tyT2) = do
    tyT1 <- typeof ctx t1
    case tyT1 of
        TyAll _ tyT11 -> return $ typeSubstTop tyT2 tyT11
        _             -> typeError "Type mismatch: expected universal type"
typeof ctx (TmPack tyT1 t2 tyT3) = case tyT3 of
    TySome x tyT31 -> do
        tyT2 <- typeof ctx t2
        if typeSubstTop tyT1 tyT2 == tyT31
            then return tyT3
            else typeError "Type mismatch: unmatched type parameter in existential type"
    _ -> typeError "Type mismatch: expected existential type"
typeof ctx (TmUnpack tyX x t1 t2) = do
    tyT1 <- typeof ctx t1
    case tyT1 of
        TySome _ tyT11 -> do
            let ctx' = addBinding ctx tyX TyVarBind
            let ctx'' = addBinding ctx' x (VarBind tyT11)
            tyT2 <- typeof ctx'' t2
            return $ typeShift (-2) tyT2
        _ -> typeError "Type mismatch: expected existential type"



-- t = d (\x: X. \y: Y d) in a context R |- c, d, Y, X, a, b
-- or 4 (\. \. 6) in nameless form
testCase :: Term
testCase = TmApp (TmVar 4 6) (
    TmAbs "x" (TyVar 2 6) (
        TmAbs "y" (TyVar 4 7) (
            TmVar 6 8)))

-- do [4 -> 5]testCase, should return 5 (\. \. 7)
-- this shows the definition on TaPL produces wrong substution result
test :: Term
test = termSubst 4 (TmVar 5 6) testCase
