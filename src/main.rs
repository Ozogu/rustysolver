use rustysolver::cfr::CFR;
use rustysolver::leduc::Leduc;
use rustysolver::history::History;

fn main() {
    let mut cfr = CFR::new(Leduc::new());
    let ev = cfr.train_to_exploitability(10.0);

    cfr.print_strategy();
    println!("Expected value: {:.4}", ev);

    let statistics = cfr.build_statistics();
    let strategy_ev = statistics.node_util(&History::new());
    let br_ev = statistics.node_br_util(&History::new());
    let explitability = statistics.node_exploitability(&History::new());
    println!("Strategy EV: {:.4}, BR EV: {:.4} Exploitability: {:.2} %", strategy_ev, br_ev, explitability);
}
