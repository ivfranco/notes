from pyDatalog import pyDatalog

pyDatalog.create_terms(
    "X, Y, Z, k," +
    "Spouse, Parent, Grandchild, Grandparent, Greatgrandparent, Ancestor," +
    "Sibling, Brother, Sister, Male, Female, Daughter, Son," + 
    "FirstCousin, BrotherInLaw, SisterInLaw, Aunt, Uncle"
)

def rules():
    Female(X) <= ~Male(X)
    Spouse(X, Y) <= Spouse(Y, X)
    Parent(X, Y) <= Parent(Z, Y) & Spouse(X, Z)
    Grandchild(X, Y) <= Parent(Y, Z) & Parent(Z, X)
    Grandparent(X, Y) <= Grandchild(Y, X)
    Greatgrandparent(X, Y) <= Grandparent(X, Z) & Parent(Z, Y)
    Ancestor(X, Y) <= Parent(X, Y)
    Ancestor(X, Y) <= Parent(X, Z) & Ancestor(Z, Y)
    Sibling(X, Y) <= Parent(Z, X) & Parent(Z, Y) & (X != Y)
    Brother(X, Y) <= Sibling(X, Y) & Male(X)
    Sister(X, Y) <= Sibling(X, Y) & Female(X)
    Daughter(X, Y) <= Parent(Y, X) & Female(X)
    Son(X, Y) <= Parent(Y, X) & Male(X)
    FirstCousin(X, Y) <= Grandparent(Z, X) & Grandparent(Z, Y) & ~Sibling(X, Y) & (X != Y)
    BrotherInLaw(X, Y) <= Spouse(Z, Y) & Brother(X, Z)
    SisterInLaw(X, Y) <= Spouse(Z, Y) & Sister(X, Z)
    Aunt(X, Y) <= Parent(Z, Y) & Sister(X, Z)
    Uncle(X, Y) <= Parent(Z, Y) & Brother(X, Z)

def data():
    + Parent("Diana", "William")
    + Parent("Diana", "Harry")
    + Parent("Anne", "Peter")
    + Parent("Anne", "Zara")
    + Parent("Andrew", "Beatrice")
    + Parent("Andrew", "Eugenie")
    + Parent("Edward", "Louise")
    + Parent("Edward", "James")
    + Parent("Spencer", "Diana")
    + Parent("Elizabeth", "Charles")
    + Parent("Elizabeth", "Anne")
    + Parent("Elizabeth", "Andrew")
    + Parent("Elizabeth", "Edward")
    + Parent("George", "Elizabeth")
    + Parent("George", "Margaret")

    + Spouse("Diana", "Charles")
    + Spouse("Anne", "Mark")
    + Spouse("Andrew", "Sarah")
    + Spouse("Edward", "Sophie")
    + Spouse("Spencer", "Kydd")
    + Spouse("Elizabeth", "Philip")
    + Spouse("George", "Mum")

    for male in [
        "William",
        "Harry",
        "Peter",
        "James",
        "Charles",
        "Mark",
        "Andrew",
        "Edward",
        "Spencer",
        "Philip",
        "George"
    ]:
        +Male(male)

if __name__ == "__main__":
    rules()
    data()

    print("Elizabeth's grandchildren:")
    print(Grandchild(X, "Elizabeth"))
    print("Diana's brothers-in-law:")
    print(BrotherInLaw(X, "Diana"))
    print("Zara's great-grandparents:")
    print(Greatgrandparent(X, "Zara"))
    print("Eugenie's ancestors:")
    print(Ancestor(X, "Eugenie"))