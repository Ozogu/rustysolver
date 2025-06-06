use crate::game::Game;
use crate::pot::Pot;
use crate::postflop_holdem_config::PostflopHoldemConfig;
use crate::action::Action;
use crate::deal::Deal;
use crate::history::History;
use crate::deck::Deck;
use crate::street::Street;
use crate::history_node::HistoryNode;
use crate::player_cards::PlayerCards;

use rand::rngs::StdRng;

#[derive(Clone, Debug)]
pub struct PostflopHoldem {
    config: PostflopHoldemConfig,
}

impl PostflopHoldem {
    pub fn new(config: PostflopHoldemConfig) -> Self {
        PostflopHoldem {
            config,
        }
    }

    fn sizes_for_street(&self, street: Street) -> Vec<Action> {
        match street {
            Street::Flop(_) => self.config.flop_sizes.iter().map(|size| Action::Bet(size.clone())).collect(),
            Street::Turn(_) => self.config.turn_sizes.iter().map(|size| Action::Bet(size.clone())).collect(),
            Street::River(_) => self.config.river_sizes.iter().map(|size| Action::Bet(size.clone())).collect(),
            _ => panic!("Invalid street {}", street.to_string()),
        }
    }

    fn bets_to_raises(&self, bets: Vec<Action>) -> Vec<Action> {
        bets.into_iter().map(|bet| {
            match bet {
                Action::Bet(size) => Action::Raise(size),
                _ => panic!("Invalid action"),
            }
        }).collect()
    }
}

impl Game for PostflopHoldem {
    fn initial_pot(&self) -> Pot {
        let part = self.config.initial_pot/2.0;
        Pot::new(part, part)
    }

    fn deck(&self) -> Deck {
        Deck::new()
    }

    fn num_streets(&self) -> u8 {
        3
    }

    fn legal_actions(&self, history: &History) -> Vec<Action> {
        // TODO: Handle effective stack size
        let last = history.last().unwrap_or(&HistoryNode::Action(Action::Check)).action();
        let mut actions = vec![];
        match last {
            Action::Check => {
                actions.push(Action::Check);
                actions.extend(self.sizes_for_street(history.street().clone()));
            }
            Action::Bet(_) => {
                actions.push(Action::Fold);
                actions.push(Action::Call);
                actions.extend(self.bets_to_raises(self.sizes_for_street(history.street().clone())));
            }
            Action::Raise(_) => {
                actions.push(Action::Fold);
                actions.push(Action::Call);
                actions.extend(self.bets_to_raises(self.sizes_for_street(history.street().clone())));
            }
            Action::None => {
                actions.push(Action::Check);
                actions.extend(self.sizes_for_street(history.street().clone()));
            }
            _ => ()
        };

        actions
    }

    fn legal_first_actions(&self) -> Vec<Action> {
        let mut actions = vec![Action::Check];
        actions.extend(self.config.flop_sizes.iter().map(|size| Action::Bet(size.clone())));

        actions
    }

    fn generate_deals(&self) -> Vec<Deal> {
        let mut deals = Vec::new();
        let oop_range = self.config.oop_range().expand_meta_suits();
        let ip_range = self.config.ip_range().expand_meta_suits();

        for oop_card in &oop_range {
            for ip_card in &ip_range {
                let mut deck = self.deck();
                deck.remove(&oop_card.0.card1);
                deck.remove(&oop_card.0.card2);
                deck.remove(&ip_card.0.card1);
                deck.remove(&ip_card.0.card2);

                let mut history = History::new();
                history.push_street(Street::Flop(self.config.flop.clone()));
                deals.push(
                    Deal::new(
                        PlayerCards::new(ip_card.0.clone(), oop_card.0.clone()),
                        deck,
                        (*ip_card.1, *oop_card.1),
                        history,
                    ),
                );
            }
        }

        deals
    }

    fn deal(&self, _rng: &mut StdRng) -> Deal {
        Deal::new_default()
    }
}
