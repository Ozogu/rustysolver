use rustysolver::cfr::CFR;
use rustysolver::kuhn::Kuhn;
use rustysolver::history::History;

fn main() {
    let mut cfr = CFR::new(Kuhn::new());
    let ev = cfr.train(100000);
    println!("Expected value: {}", ev);
    cfr.print_strategy();

    let statistics = cfr.build_statistics();
    let strategy_ev = statistics.get_node_util(&History::new());
    let br_ev = statistics.get_node_br_util(&History::new());
    println!("Strategy EV: {}, BR EV: {}", strategy_ev, br_ev);
}
