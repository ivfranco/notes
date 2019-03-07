use crate::parse_table::{
    Production,
    Symbol::{self, *},
};
use crate::slr::Token;
use crate::Grammar;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::{self, Debug, Formatter};

type Nonterm = usize;
type State = usize;

const START: State = 0;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
enum LookAhead<T> {
    Token(T),
    Empty,
}

impl<T: Debug> Debug for LookAhead<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            LookAhead::Token(t) => write!(f, "{:?}", t),
            LookAhead::Empty => write!(f, "$"),
        }
    }
}

impl<T> From<Option<T>> for LookAhead<T> {
    fn from(option: Option<T>) -> Self {
        option.map_or(LookAhead::Empty, LookAhead::Token)
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Item<'a, T> {
    production: &'a Production<T>,
    dot: usize,
    lookahead: LookAhead<T>,
}

impl<'a, T: Clone> Item<'a, T> {
    fn new(production: &'a Production<T>, dot: usize, lookahead: LookAhead<T>) -> Self {
        Item {
            production,
            dot,
            lookahead,
        }
    }

    fn new_start(grammar: &'a Grammar<T>) -> Self {
        let start = &grammar.prod_map[&grammar.start][0];
        Item::new(start, 0, LookAhead::Empty)
    }

    fn increment(&self) -> Self {
        Item {
            production: self.production,
            dot: self.dot + 1,
            lookahead: self.lookahead.clone(),
        }
    }

    fn after_dot(&self) -> Option<&Symbol<T>> {
        self.production.body.get(self.dot)
    }

    fn nonterm_after_dot(&self) -> Option<Nonterm> {
        self.after_dot()
            .and_then(|symbol| if let N(n) = symbol { Some(*n) } else { None })
    }

    fn tail(&self) -> Vec<Symbol<T>> {
        let mut tail = self.production.body[self.dot + 1..].to_owned();
        if let LookAhead::Token(t) = &self.lookahead {
            tail.push(Symbol::T(t.clone()));
        }

        tail
    }
}

impl<'a, T: Debug> Item<'a, T> {
    fn to_string(&self, rev_map: &HashMap<usize, String>) -> String {
        let mut symbols: Vec<String> = self
            .production
            .body
            .iter()
            .map(|symbol| symbol.to_string(rev_map))
            .collect();
        if symbols.is_empty() {
            symbols.push("ε".to_owned());
        }

        symbols.insert(self.dot, ".".to_owned());

        format!(
            "[{} -> {}, {:?}]",
            N::<T>(self.production.head).to_string(rev_map),
            symbols.join(" "),
            self.lookahead,
        )
    }
}

struct ItemSet<'a, T> {
    grammar: &'a Grammar<T>,
    items: BTreeSet<Item<'a, T>>,
}

impl<'a, T: PartialEq> PartialEq for ItemSet<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.items == other.items
    }
}

impl<'a, T: Eq> Eq for ItemSet<'a, T> {}

impl<'a, T: PartialOrd> PartialOrd for ItemSet<'a, T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.items.partial_cmp(&other.items)
    }
}

impl<'a, T: Ord> Ord for ItemSet<'a, T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.items.cmp(&other.items)
    }
}

impl<'a, T: Token> ItemSet<'a, T> {
    fn new_start_set(grammar: &'a Grammar<T>) -> Self {
        let mut items = BTreeSet::new();
        items.insert(Item::new_start(grammar));
        ItemSet { grammar, items }.closure()
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn closure(&self) -> Self {
        let mut closure = self.items.clone();
        let mut updated = true;

        while updated {
            updated = false;
            let mut inc = BTreeSet::new();
            for item in &closure {
                if let Some(n) = item.nonterm_after_dot() {
                    for p in &self.grammar.prod_map[&n] {
                        for b in self.grammar.string_first(&item.tail()) {
                            inc.insert(Item::new(p, 0, LookAhead::from(b)));
                        }
                    }
                }
            }
            let old_size = closure.len();
            closure.append(&mut inc);
            updated = updated || old_size < closure.len();
        }

        ItemSet {
            grammar: self.grammar,
            items: closure,
        }
    }

    fn goto(&self, symbol: &Symbol<T>) -> Self {
        let goto = self
            .items
            .iter()
            .filter_map(|item| {
                if item.after_dot() == Some(symbol) {
                    Some(item.increment())
                } else {
                    None
                }
            })
            .collect();

        let set = ItemSet {
            grammar: self.grammar,
            items: goto,
        };
        set.closure()
    }
}

impl<'a, T: Debug> Debug for ItemSet<'a, T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let rev_map = self.grammar.rev_map();
        for item in &self.items {
            writeln!(f, "{}", item.to_string(&rev_map))?;
        }
        Ok(())
    }
}

fn items<'a, T: Token>(grammar: &'a Grammar<T>) -> BTreeMap<ItemSet<'a, T>, State> {
    let alphabet = grammar.alphabet();
    let mut set_map = BTreeMap::new();
    let start = ItemSet::new_start_set(grammar);
    set_map.insert(start, START);
    let mut updated = true;
    let mut next_state = START + 1;

    while updated {
        let mut inc = BTreeMap::new();

        for set in set_map.keys() {
            for symbol in &alphabet {
                let dest = set.goto(symbol);
                if !dest.is_empty() && !set_map.contains_key(&dest) {
                    inc.insert(dest, next_state);
                    next_state += 1;
                }
            }
        }

        updated = !inc.is_empty();
        set_map.append(&mut inc);
    }

    set_map
}

pub struct CanonicalLR<'a, T> {
    grammar: &'a Grammar<T>,
    set_map: BTreeMap<ItemSet<'a, T>, State>,
}

impl<'a, T: Token> CanonicalLR<'a, T> {
    pub fn new(grammar: &'a Grammar<T>) -> Self {
        let set_map = items(grammar);
        CanonicalLR { grammar, set_map }
    }

    pub fn size(&self) -> usize {
        self.set_map.len()
    }
}

impl<'a, T: Debug> Debug for CanonicalLR<'a, T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        for (set, i) in &self.set_map {
            writeln!(f, "Item set {}:", i)?;
            write!(f, "{:?}", set)?;
        }
        Ok(())
    }
}

#[test]
fn canonical_lr_test() {
    let mut grammar = Grammar::parse("S", &["S -> C C", "C -> c C", "C -> d"]);
    let lr = grammar.canonical_lr();
    println!("{:?}", lr);
    assert_eq!(lr.size(), 10);
}
