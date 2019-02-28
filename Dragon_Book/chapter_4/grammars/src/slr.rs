use crate::parse_table::{
    Production,
    Symbol::{self, *},
};
use crate::Grammar;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::{self, Debug, Formatter};
use std::hash::Hash;

type Nonterm = usize;
type State = usize;

const START: State = 0;

pub trait Token: Clone + Debug + Eq + Ord + Hash {}

impl Token for char {}
impl Token for String {}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Item<'a, T> {
    production: &'a Production<T>,
    dot: usize,
}

impl<'a, T> Item<'a, T> {
    fn new(production: &'a Production<T>, dot: usize) -> Self {
        Item { production, dot }
    }

    fn new_start(grammar: &'a Grammar<T>) -> Self {
        let start = &grammar.prod_map[&grammar.start][0];
        Item::new(start, 0)
    }

    fn head(&self) -> usize {
        self.production.head
    }

    fn increment(&self) -> Self {
        Item {
            production: self.production,
            dot: self.dot + 1,
        }
    }

    fn after_dot(&self) -> Option<&Symbol<T>> {
        self.production.body.get(self.dot)
    }

    fn nonterm_after_dot(&self) -> Option<Nonterm> {
        self.after_dot()
            .and_then(|symbol| if let N(n) = symbol { Some(*n) } else { None })
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
            "{} -> {}",
            N::<T>(self.production.head).to_string(rev_map),
            symbols.join(" ")
        )
    }
}

#[derive(Clone)]
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
        let mut closure: BTreeSet<Item<'a, T>> = self.items.clone();
        let mut added: BTreeSet<Nonterm> = BTreeSet::new();
        let mut updated = true;

        while updated {
            updated = false;
            let mut inc = BTreeSet::new();

            for n in closure.iter().filter_map(|item| item.nonterm_after_dot()) {
                if added.insert(n) {
                    inc.extend(self.grammar.prod_map[&n].iter().map(|p| Item::new(p, 0)));
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

pub struct Canonical<'a, T> {
    grammar: &'a Grammar<T>,
    set_map: BTreeMap<ItemSet<'a, T>, State>,
}

impl<'a, T: Token> Canonical<'a, T> {
    pub fn new(grammar: &'a Grammar<T>) -> Self {
        let set_map = items(grammar);
        Canonical { grammar, set_map }
    }

    pub fn size(&self) -> usize {
        self.set_map.values().max().unwrap() + 1
    }

    pub fn slr(&self) -> SLRTable<T> {
        let mut tables = vec![Table::new(); self.size()];
        let mut goto = BTreeMap::new();

        for (set, &i) in &self.set_map {
            for n in 0..self.grammar.nonterm_len() {
                let dest = set.goto(&N(n));
                if let Some(j) = self.set_map.get(&dest) {
                    goto.insert((i, n), *j);
                }
            }

            for item in &set.items {
                match item.after_dot() {
                    Some(t @ T(..)) => {
                        let dest = set.goto(t);
                        let j = *self
                            .set_map
                            .get(&dest)
                            .expect("Error: Canonical Set is not closed");
                        tables[i]
                            .entry(t.as_token())
                            .or_insert_with(Vec::new)
                            .push(Action::Shift(j));
                    }
                    None if item.head() == self.grammar.start => {
                        tables[i]
                            .entry(None)
                            .or_insert_with(Vec::new)
                            .push(Action::Accept);
                    }
                    None => {
                        for t in self.grammar.follow_nonterm(item.head()) {
                            tables[i]
                                .entry(t.as_ref())
                                .or_insert_with(Vec::new)
                                .push(Action::Reduce(item.production));
                        }
                    }
                    _ => (),
                }
            }
        }

        SLRTable {
            canonical: self,
            tables,
            goto,
        }
    }
}

impl<'a, T: Debug> Debug for Canonical<'a, T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        for (set, i) in &self.set_map {
            writeln!(f, "Item set {}:", i)?;
            write!(f, "{:?}", set)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
enum Action<'a, T> {
    Shift(State),
    Reduce(&'a Production<T>),
    Accept,
    Error,
}

impl<'a, T: Debug> Action<'a, T> {
    fn to_string(&self, rev_map: &HashMap<usize, String>) -> String {
        match self {
            Action::Reduce(p) => format!("Reduce({})", p.to_string(rev_map)),
            _ => format!("{:?}", self),
        }
    }
}

type Table<'a, T> = BTreeMap<Option<&'a T>, Vec<Action<'a, T>>>;

pub struct SLRTable<'a, T> {
    canonical: &'a Canonical<'a, T>,
    tables: Vec<Table<'a, T>>,
    goto: BTreeMap<(State, Nonterm), State>,
}

impl<'a, T> SLRTable<'a, T> {
    pub fn is_slr(&self) -> bool {
        self.tables
            .iter()
            .all(|table| table.values().all(|actions| actions.len() <= 1))
    }
}

impl<'a, T: Token> SLRTable<'a, T> {
    fn query(&'a self, state: State, token: Option<&'a T>) -> &'a Action<'a, T> {
        self.tables
            .get(state)
            .expect("Error: Out of bound state")
            .get(&token)
            .and_then(|v| v.first())
            .unwrap_or(&Action::Error)
    }

    fn goto(&self, state: State, nonterm: Nonterm) -> State {
        *self
            .goto
            .get(&(state, nonterm))
            .expect("Error: Undefined GOTO entry")
    }

    pub fn parse(&self, input: &[T]) -> bool {
        let mut stack = vec![START];
        let mut i = 0;

        loop {
            let s = *stack
                .last()
                .expect("Error: Stack exhaused during SLR parsing");

            match self.query(s, input.get(i)) {
                Action::Shift(t) => {
                    stack.push(*t);
                    println!("Shifted terminal: {:?}", input[i]);
                    i += 1;
                }
                Action::Reduce(p) => {
                    stack.truncate(stack.len() - p.body.len());
                    let t = *stack
                        .last()
                        .expect("Error: Stack exhaused during SLR parsing");
                    stack.push(self.goto(t, p.head));
                    println!(
                        "Reduced production: {}",
                        p.to_string(&self.canonical.grammar.rev_map())
                    );
                }
                Action::Accept => return true,
                Action::Error => return false,
            }
        }
    }
}

impl<'a, T: Debug> Debug for SLRTable<'a, T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self.canonical)?;

        let rev_map = self.canonical.grammar.rev_map();

        for (state, table) in self.tables.iter().enumerate() {
            for (nonterm, actions) in table {
                let symbol = nonterm.map_or("$".to_owned(), |n| format!("{:?}", n));
                writeln!(f, "ACTION[{}, {}]:", state, symbol)?;

                if actions.len() > 1 {
                    writeln!(f, "Warning: conflicts")?;
                }

                for action in actions {
                    writeln!(f, "    {}", action.to_string(&rev_map))?;
                }
            }
        }

        for ((state, nonterm), dest) in &self.goto {
            writeln!(
                f,
                "GOTO({}, {}) = {}",
                state,
                N::<T>(*nonterm).to_string(&rev_map),
                dest
            )?;
        }
        Ok(())
    }
}

pub fn tokenize(input: &str) -> Vec<String> {
    input.split_whitespace().map(|s| s.to_owned()).collect()
}

#[test]
fn canonical_set_test() {
    let mut grammar = Grammar::parse(
        "E",
        &[
            "E -> E + T",
            "E -> T",
            "T -> T * F",
            "T -> F",
            "F -> ( E )",
            "F -> id",
        ],
    );
    let canonical = grammar.canonical();

    assert_eq!(canonical.set_map.len(), 12);
}

#[test]
fn slr_parse_test() {
    let mut grammar = Grammar::parse(
        "E",
        &[
            "E -> E + T",
            "E -> T",
            "T -> T * F",
            "T -> F",
            "F -> ( E )",
            "F -> id",
        ],
    );
    let canonical = grammar.canonical();
    let slr = canonical.slr();
    assert!(slr.is_slr());

    let accepted_input: Vec<String> = "id * id + id"
        .split_whitespace()
        .map(|s| s.to_owned())
        .collect();
    let rejected_input: Vec<String> = "id * * id + id"
        .split_whitespace()
        .map(|s| s.to_owned())
        .collect();
    assert!(slr.parse(&accepted_input));
    assert!(!slr.parse(&rejected_input));
}
