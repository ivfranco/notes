from nltk.parse.generate import generate
from nltk.parse import BottomUpChartParser
from nltk import CFG

grammar_a = """
S -> NP VP
NP -> Pronoun | Noun | Article Noun
VP -> Verb | VP PP | VP Adv Adv
PP -> Prep NP
Pronoun -> 'someone'
Verb -> 'walked'
Adv -> 'slowly'
Prep -> 'to'
Article -> 'the'
Noun -> 'supermarket'
"""

grammar_b = """
S -> NP VP
NP -> Pronoun | Noun | Article NP
VP -> Verb Vmod
Vmod -> Adv | Adv Vmod
PP -> Prep NP
Pronoun -> 'someone'
Verb -> 'walked'
Adv -> 'slowly' | PP
Prep -> 'to'
Article -> 'the'
Noun -> 'supermarket'
"""

grammar_c = """
S -> NP VP
NP -> Pronoun | Noun | Article NP
VP -> Verb Adv
PP -> Prep NP
Pronoun -> 'someone'
Verb -> 'walked'
Adv -> 'slowly' | PP | Adv Adv
Prep -> 'to'
Article -> 'the'
Noun -> 'supermarket'
"""

if __name__ == "__main__":
    for raw in [grammar_a, grammar_b, grammar_c]:
        grammar = CFG.fromstring(raw)
        parser = BottomUpChartParser(grammar)
        print('')
        for t in parser.parse('someone walked slowly to the supermarket'.split(' ')):
            print(t)
        for sentence in generate(grammar, n = 20):
            print(' '.join(sentence))