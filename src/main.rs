use rustysolver::cfr::CFR;
use rustysolver::kuhn::Kuhn;
use rustysolver::history::History;

fn main() {
    let mut cfr = CFR::new(Kuhn::new());
    let ev = cfr.train(100000);
    println!("Expected value: {}", ev);
    cfr.print_strategy();

    let statistics = cfr.build_statistics();
    let statistics_ev = statistics.get_node_util(&History::new());
    println!("Statistics EV: {}", statistics_ev);
}
