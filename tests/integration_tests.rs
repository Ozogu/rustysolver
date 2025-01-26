use rustysolver::cfr::CFR;
use rustysolver::info_state::InfoState;
use rustysolver::kuhn::Kuhn;
use rustysolver::leduc::Leduc;

#[test]
fn test_kuhn_poker_ev() {
    let mut cfr = CFR::new(Kuhn::new());
    let ev = cfr.train_for_iters(12000);
    let ideal_ev = -1.0/18.0;
    let ev_diff = (ev - ideal_ev).abs();

    debug_assert!(ev_diff < 0.0011,
        "EV: {:.4}, Ideal: {:.4}, Diff: {:.4}",
        ev, ideal_ev, ev_diff);

    let statistics = cfr.build_statistics();
    let strategy_ev = statistics.node_util(&InfoState::new_empty());
    let strategy_ev_diff = (strategy_ev - ideal_ev).abs();

    debug_assert!(strategy_ev_diff < 0.0008,
        "Strategy EV: {:.4}, Ideal: {:.4}, Diff: {:.4}",
        strategy_ev, ideal_ev, strategy_ev_diff);

    let br_util = statistics.node_br_util(&InfoState::new_empty());
    let br_util_diff = (br_util - ideal_ev).abs();

    debug_assert!(br_util_diff < 0.01,
        "BR Util: {:.4}, Ideal: {:.4}, Diff: {:.4}",
        br_util, ideal_ev, br_util_diff);

    let exploitability = statistics.node_exploitability(&InfoState::new_empty());
    debug_assert!(exploitability < 15.0, "Exploitability: {:.4} %", exploitability);

}

// #[test]
// fn test_leduc_poker_ev() {
//     let mut cfr = CFR::new(Leduc::new());
//     let ev = cfr.train_for_iters(12000);
//     let ideal_ev = 0.0;
//     let ev_diff = (ev - ideal_ev).abs();

//     cfr.print_strategy();

//     debug_assert!(ev_diff < 0.0011,
//         "EV: {:.4}, Ideal: {:.4}, Diff: {:.4}",
//         ev, ideal_ev, ev_diff);
// }
