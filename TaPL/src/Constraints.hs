module Constraints where

import           Control.Arrow (second, (***))

data Type
    = TyBool
    | TyArr Type Type
    | TyId String
    | TyNat
    deriving (Eq, Ord)

type Constraints = [(Type, Type)]

data Term
    = TmVar Int
    | TmAbs String Type Term
    | TmApp Term Term
    | TmZero
    | TmSucc Term
    | TmPred Term
    | TmTrue
    | TmFalse
    | TmIsZero Term
    | TmIf Term Term Term
    | TmLet String Term Term

type Context = ()

data Binding
    = VarBind Type
    | SchemeBind [String] Type

queryContext :: Int -> Context -> Either String Binding
queryContext = undefined

popContext :: (String, Binding) -> Context -> Context
popContext = undefined

substContext :: Substitution -> Context -> Context
substContext = undefined

elemContext :: Context -> String -> Bool
elemContext = undefined

collectVars :: Type -> [String]
collectVars (TyId x)        = [x]
collectVars (TyArr ty1 ty2) = collectVars ty1 ++ collectVars ty2
collectVars _               = []

generalize :: Context -> Type -> Binding
generalize ctx ty = SchemeBind (filter (not . elemContext ctx) (collectVars ty)) ty

recon :: Context -> [String] -> Term -> Either String (Type, [String], Constraints)
recon ctx nextvars (TmLet x t1 t2) = do
    (ty1, nextvars', c1) <- recon ctx nextvars t1
    subst <- liftMaybe "" $ unify c1
    let
        ctx' = substContext subst ctx
        pty1 = subS subst ty1
    recon (popContext (x, generalize ctx' pty1) ctx') nextvars' t2
recon ctx nextvars (TmVar x) = do
    binding <- queryContext x ctx
    case binding of
        VarBind ty -> return (ty, nextvars, [])
        SchemeBind xs ty -> do
            let
                subst = zipWith (curry (second TyId)) xs nextvars
                nextvars' = drop (length xs) nextvars
            return (subS subst ty, nextvars', [])
recon ctx nextvars (TmAbs x ty1 t2) = do
    (ty2, nextvars', c) <- recon (popContext (x, VarBind ty1) ctx) nextvars t2
    return (TyArr ty1 ty2, nextvars', c)
recon ctx nextvars (TmApp t1 t2) = do
    (ty1, nextvars', c1) <- recon ctx nextvars t1
    (ty2, nextvars'', c2) <- recon ctx nextvars' t2
    let
        (x : nextvars''') = nextvars''
        ty = TyId x
        c = c1 ++ c2 ++ [(ty1, TyArr ty2 ty)]
    return (ty, nextvars''', c)
recon _ nextvars TmZero = return (TyNat, nextvars, [])
recon _ nextvars TmTrue = return (TyBool, nextvars, [])
recon _ nextvars TmFalse = return (TyBool, nextvars, [])
recon ctx nextvars (TmSucc t1) = do
    (ty1, nextvars', c) <- recon ctx nextvars t1
    return (TyNat, nextvars', (ty1, TyNat) : c)
recon ctx nextvars (TmPred t1) = do
    (ty1, nextvars', c) <- recon ctx nextvars t1
    return (TyNat, nextvars', (ty1, TyNat) : c)
recon ctx nextvars (TmIsZero t1) = do
    (ty1, nextvars', c) <- recon ctx nextvars t1
    return (TyBool, nextvars', (ty1, TyNat) : c)
recon ctx nextvars (TmIf t1 t2 t3) = do
    (ty1, v1, c1) <- recon ctx nextvars t1
    (ty2, v2, c2) <- recon ctx v1 t2
    (ty3, v3, c3) <- recon ctx v2 t3
    let c = concat [c1, c2, c3, [(ty2, ty3), (ty1, TyBool)]]
    return (ty2, v3, c)

fv :: String -> Type -> Bool
fv s t = case t of
    TyId x      -> s == x
    TyArr t1 t2 -> fv s t1 || fv s t2
    _           -> False

subT :: String -> Type -> Type -> Type
subT x s t = case t of
    TyId x'     | x == x' -> s
    TyArr t1 t2 -> TyArr (subT x s t1) (subT x s t2)
    _           -> t

subS :: Substitution -> Type -> Type
subS subst t = foldr (\(x, s) t' -> subT x s t') t subst

subC :: String -> Type -> Constraints -> Constraints
subC x s = fmap (subT x s *** subT x s)

type Substitution = [(String, Type)]

unify :: Constraints -> Maybe Substitution
unify [] = return []
unify (c:cs) = case c of
    (s, t) | s == t -> unify cs
    (TyId x, t) | not (fv x t) -> do
        subst <- unify (subC x t cs)
        return $ (x, t) : subst
    (s, TyId x) | not (fv x s) -> do
        subst <- unify (subC x s cs)
        return $ (x, s) : subst
    (TyArr s1 t1, TyArr s2 t2) -> unify ((s1, t1) : (s2, t2) : cs)
    _ -> Nothing

liftMaybe :: b -> Maybe a -> Either b a
liftMaybe b Nothing  = Left b
liftMaybe _ (Just a) = Right a

genvars :: [String]
genvars = undefined

typeof :: Context -> Term -> Either String (Substitution, Type)
typeof ctx t = do
    (ty, _, c) <- recon ctx genvars t
    subst <- liftMaybe "Solution doesn't exist for constraints" $ unify c
    return (subst, subS subst ty)
