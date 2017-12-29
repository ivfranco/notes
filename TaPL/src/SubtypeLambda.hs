module SubtypeLambda where

import           Data.Map    (Map)
import qualified Data.Map    as Map
import           Text.Printf (printf)

data Type
    = TyRecord (Map String Type)
    | TyTop
    | TyArr Type Type
    | TyBool
    | TyBot
    deriving (Eq)

data Term
    = TmRecord (Map String Term)
    | TmProj Term String
    | TmVar Int Int
    | TmAbs String Type Term
    | TmApp Term Term
    | TmTrue
    | TmFalse
    | TmIf Term Term Term

subtype :: Type -> Type -> Bool
subtype tyS tyT | tyS == tyT = True
subtype (TyRecord fS) (TyRecord fT) = and $ Map.mapWithKey (inRecord fT) fS
    where
        inRecord m l t = case Map.lookup l m of
            Nothing -> False
            Just t' -> t == t'
subtype _ TyTop = True
subtype TyBot _ = True
subtype (TyArr tyS1 tyS2) (TyArr tyT1 tyT2) = subtype tyT1 tyS1 && subtype tyS2 tyT2
subtype _ _ = False

type Context = [(String, Binding)]

data Binding
    = NameBind
    | VarBind Type

join :: Type -> Type -> Type
join tyS tyT | tyS == tyT = tyS
join (TyRecord fS) (TyRecord fT) = TyRecord $ Map.intersectionWith join fS fT
join (TyArr s1 s2) (TyArr t1 t2) = case meet s1 t1 of
    Just j1 -> TyArr j1 (join s2 t2)
    Nothing -> TyTop
join _ _ = TyTop

meet :: Type -> Type -> Maybe Type
meet tyS tyT | tyS == tyT = Just tyS
meet (TyRecord fS) (TyRecord fT) = do
    fJ <- sequence (Map.unionWith meet' fS' fT')
    return $ TyRecord fJ
    where
        fS' = fmap Just fS
        fT' = fmap Just fT
        meet' (Just s) (Just t) = meet s t
        meet' _ _               = Nothing
meet (TyArr s1 s2) (TyArr t1 t2) = do
    j2 <- meet s2 t2
    return $ TyArr (join s1 t1) j2
meet _ _ = Nothing


addBinding :: Context -> String -> Binding -> Context
addBinding ctx x bind = (x, bind) : ctx

getTypeFromContext :: Context -> Int -> Type
getTypeFromContext ctx i = case getBinding ctx i of
    VarBind ty -> ty

getBinding :: Context -> Int -> Binding
getBinding ctx i = snd (ctx !! i)

typeError :: String -> Either String a
typeError = Left

typeof :: Context -> Term -> Either String Type
typeof _ TmTrue = return TyBool
typeof _ TmFalse = return TyBool
typeof ctx (TmRecord fields) = do
    ts <- mapM (typeof ctx) fields
    return $ TyRecord ts
typeof ctx (TmProj t1 l) = do
    rcd <- typeof ctx t1
    case rcd of
        TyRecord fields -> case Map.lookup l fields of
            Nothing -> typeError (printf "label %s not found" l)
            Just t2 -> return t2
        TyBot -> return TyBot
        _ -> typeError "Expected record type"
typeof ctx (TmVar i _) = return $ getTypeFromContext ctx i
typeof ctx (TmAbs x tyT1 t2) = do
    let ctx' = addBinding ctx x (VarBind tyT1)
    tyT2 <- typeof ctx' t2
    return $ TyArr tyT1 tyT2
typeof ctx (TmApp t1 t2) = do
    tyT1 <- typeof ctx t1
    tyT2 <- typeof ctx t2
    case tyT1 of
        TyArr tyT11 tyT12 -> if subtype tyT2 tyT11
            then return tyT12
            else typeError "parameter type mismatch"
        TyBot -> return TyBot
        _ -> typeError "arrow type expected in application"
typeof ctx (TmIf t1 t2 t3) = do
    ty1 <- typeof ctx t1
    case ty1 of
        TyBool -> do
            ty2 <- typeof ctx t2
            ty3 <- typeof ctx t3
            return $ join ty2 ty3
        TyBot -> return TyBot
        _ -> typeError "Expected boolean"
