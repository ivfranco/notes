from pyDatalog import pyDatalog

pyDatalog.create_terms("create,copy,store,load,pts,hpts,V,H,G,F,W,T,S,I,N,M")

def rules():
    pts(V, H) <= create(H, T, V)
    pts(V, H) <= copy(V, W) & pts(W, H)
    hpts(H, F, G) <= store(V, F, W) & pts(W, G) & pts(V, H)
    pts(V, H) <= load(V, W, F) & pts(W, G) & hpts(G, F, H)

def exercise_12_4_1():
    print("Exercise 12.4.1")
    pyDatalog.clear()
    rules()

    + create('H', 'T', 'a')
    + create('G', 'T', 'b')
    + copy('c', 'a')
    + store('a', 'f', 'b')
    + store('b', 'f', 'c')
    + load('d', 'c', 'f')

    print(pts(V, H))
    print(hpts(H, F, G))

def exercise_12_4_3_a():
    print("Exercise 12.4.3.a")
    pyDatalog.clear()
    rules()

    + create('g', 'T', 'b')
    + create('h', 'T', 'a')
    + store('a', 'f', 'b')
    + copy('b', 'a')
    + load('b', 'b', 'f')

    print(pts(V, H))
    print(hpts(H, F, G))

def exercise_12_4_3_b():
    print("Exercise 12.4.3.b")
    pyDatalog.clear()
    rules()

    + create('g', 'T', 'b')
    + create('h', 'T', 'a')
    + store('a', 'f', 'b')
    + copy('c', 'a')
    + load('d', 'c', 'f')

    print(pts(V, H))
    print(hpts(H, F, G))

if __name__ == "__main__":
    exercise_12_4_1()
    exercise_12_4_3_a()
    exercise_12_4_3_b()