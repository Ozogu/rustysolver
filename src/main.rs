use rustysolver::cfr::CFR;
use rustysolver::kuhn::Kuhn;
use rustysolver::leduc::Leduc;
use rustysolver::info_state::InfoState;

fn main() {
    let mut cfr = CFR::new(Kuhn::new());
    // let ev = cfr.train_to_exploitability(26.0);
    let ev = cfr.train_for_iters(1000);

    cfr.print_strategy();
    println!("Expected value: {:.4}", ev);

    let statistics = cfr.build_statistics();
    let strategy_ev = statistics.node_util(&InfoState::new_empty());
    let br_ev = statistics.node_br_util(&InfoState::new_empty());
    let explitability = statistics.node_exploitability(&InfoState::new_empty());
    println!("Strategy EV: {:.4}, BR EV: {:.4} Exploitability: {:.2} %", strategy_ev, br_ev, explitability);
}
