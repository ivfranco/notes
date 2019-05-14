from pyDatalog import pyDatalog

pyDatalog.create_terms("""
    create,
    copy,
    store,
    load,
    pts,
    hpts,
    actual,
    dispatch,
    formal,
    cha,
    invokes,
    hType,
    ret,
    U,V,H,G,F,W,T,S,I,N,M""")

def rules():
    # create(H, T, V): a statement H: T V = new T()
    # copy(V, W): a statement V = W
    # store(V, F, W): a statement V.F = W
    # load(V, W, F): a statement V = W.F

    pts(V, H) <= create(H, T, V)
    pts(V, H) <= copy(V, W) & pts(W, H)
    hpts(H, F, G) <= store(V, F, W) & pts(W, G) & pts(V, H)
    pts(V, H) <= load(V, W, F) & pts(W, G) & hpts(G, F, H)

    invokes(S, M) <= dispatch(S, V, W, N) & pts(W, H) & hType(H, T) & cha(T, N, M)
    pts(V, H) <= invokes(S, M) & formal(M, I, V) & actual(S, I, W) & pts(W, H)

    pts(V, H) <= dispatch(S, V, W, N) & invokes(S, M) & ret(M, U) & pts(U, H)


def exercise_12_5_1():
    pyDatalog.clear()
    rules()

    + load(None, None, None)
    + copy(None, None)

    + formal(1, 0, 'this1')
    + formal(2, 0, 'this2')
    + formal(3, 0, 'this3')

    + pts(1, 'g')
    + pts(2, 'h')
    + pts(3, 'i')
     
    + ret(1, 1)
    + ret(2, 2)
    + ret(3, 3)

    + actual(5, 0, 'a')

    + cha('t', 'n', 1)
    + cha('s', 'n', 2)
    + cha('r', 'n', 3)

    + create('j', 't', 'a')
    + hType('j', 't')
    + dispatch(5, 'a', 'a', 'n')

    print(invokes(S, M))
    print(pts(V, H))

if __name__ == "__main__":
    exercise_12_5_1()