use crate::network::*;
use petgraph::prelude::*;

pub mod burglary {
    use super::*;

    pub const T: Value = 1;
    pub const F: Value = 0;

    /// return the network, and a sequence of nodes in following order:
    /// [burglary, earthquake, alarm, john_calls, mary_calls]
    pub fn burglary_network() -> (Network, [NodeIndex; 5]) {
        let mut network = Network::new();
        let burglary = network.add_node(Variable::new_const(vec![1.0 - 0.001, 0.001]));
        let earthquake = network.add_node(Variable::new_const(vec![1.0 - 0.002, 0.002]));

        let mut alarm_cpt = Full::new(&[burglary, earthquake]);
        alarm_cpt.insert_row(&[(burglary, T), (earthquake, T)], &[1.0 - 0.95, 0.95]);
        alarm_cpt.insert_row(&[(burglary, T), (earthquake, F)], &[1.0 - 0.94, 0.94]);
        alarm_cpt.insert_row(&[(burglary, F), (earthquake, T)], &[1.0 - 0.29, 0.29]);
        alarm_cpt.insert_row(&[(burglary, F), (earthquake, F)], &[1.0 - 0.001, 0.001]);
        let alarm = network.add_node(Variable::new(CPT::Full(alarm_cpt), 2));

        let john_calls = network.add_node(Variable::binary_single_parent(alarm, 0.9, 0.05));
        let mary_calls = network.add_node(Variable::binary_single_parent(alarm, 0.7, 0.01));

        network.add_edge(burglary, alarm);
        network.add_edge(earthquake, alarm);
        network.add_edge(alarm, john_calls);
        network.add_edge(alarm, mary_calls);

        (
            network,
            [burglary, earthquake, alarm, john_calls, mary_calls],
        )
    }
}