module TypedLambdaB where

type Context = [(String, Binding)]

data Binding
    = NameBind
    | VarBind Type

addBinding :: Context -> String -> Binding -> Context
addBinding ctx x bind = (x, bind) : ctx

getTypeFromContext :: Context -> Int -> Type
getTypeFromContext ctx i = case getBinding ctx i of
    VarBind ty -> ty

getBinding :: Context -> Int -> Binding
getBinding ctx i = snd (ctx !! i)

data Type
    = TyBool
    | TyArr Type Type
    deriving (Eq)

data Term
    = TmTrue
    | TmFalse
    | TmIf Term Term Term
    | TmVar Int Int
    | TmAbs String Type Term
    | TmApp Term Term
    | TmLet String Term Term

typeError :: String -> Either String a
typeError = Left

typeof :: Context -> Term -> Either String Type
typeof _ TmTrue = return TyBool
typeof _ TmFalse = return TyBool
typeof ctx (TmIf t1 t2 t3) = do
    tyT1 <- typeof ctx t1
    if tyT1 == TyBool
        then do
            tyT2 <- typeof ctx t2
            tyT3 <- typeof ctx t3
            if tyT2 == tyT3
                then return tyT2
                else typeError "arms of conditional do not match"
        else typeError "guard of conditional is not a boolean"
typeof ctx (TmVar i _) = return $ getTypeFromContext ctx i
typeof ctx (TmAbs x tyT1 t2) = do
    let ctx' = addBinding ctx x (VarBind tyT1)
    tyT2 <- typeof ctx' t2
    return $ TyArr tyT1 tyT2
typeof ctx (TmApp t1 t2) = do
    tyT1 <- typeof ctx t1
    tyT2 <- typeof ctx t2
    case tyT1 of
        TyArr tyT11 tyT12 -> if tyT11 == tyT2
            then return tyT12
            else typeError "parameter type mismatch"
        _ -> typeError "arrow type expected in application"
typeof ctx (TmLet x t1 t2) = do
    tyT1 <- typeof ctx t1
    typeof (addBinding ctx x (VarBind tyT1)) t2
