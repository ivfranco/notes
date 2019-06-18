use crate::{Csp, Diff, Var};

pub fn map_init(colors: u32, edges: &[(u32, u32)]) -> (Vec<Var<u32>>, Csp<Diff>) {
    let csp = Csp::from_edges(
        edges
            .iter()
            .flat_map(|(from, to)| vec![(*from, *to, Diff), (*to, *from, Diff)]),
    );
    let domain: Var<u32> = (0..colors).collect();
    (vec![domain; csp.node_count()], csp)
}
