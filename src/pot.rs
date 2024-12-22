use std::collections::HashMap;
use crate::player::Player;
use crate::action::Action;

#[derive(Clone, Debug)]
pub struct Pot {
    pot: HashMap<Player, f64>,
}

impl Pot {
    pub fn new(ip: f64, oop: f64) -> Self {
        Pot {
            pot: HashMap::from([(Player::IP, ip), (Player::OOP, oop)]),
        }
    }

    pub fn get_win_amount(&self, player: Option<Player>) -> f64 {
        match player {
            Some(player) => self.pot[&player.opponent()],
            // In case of draw, win amount is same between IP and OOP.
            None => self.pot[&Player::IP],
        }
    }

    pub fn total(&self) -> f64 {
        self.pot[&Player::IP] + self.pot[&Player::OOP]
    }

    pub fn update(&mut self, player: Player, action: Action) {
        match action {
            Action::Check => (),
            Action::Bet(amount) => {
                *self.pot.get_mut(&player).unwrap() +=
                    self.bet_amount(self.total(), amount);
            },
            Action::Raise(amount) => {
                let to_call = self.to_call();
                *self.pot.get_mut(&player).unwrap() += 
                    self.bet_amount(self.total() +  to_call, amount) + to_call;
            },
            Action::Call => {
                self.pot.insert(player, self.pot[&player.opponent()]);

            },
            Action::Fold => {
                panic!("Fold should not reach Pot::update");
            },
        }
    }

    fn to_call(&self) -> f64 {
        (self.pot[&Player::IP] - self.pot[&Player::OOP]).abs()
    }

    fn bet_amount(&self, pot: f64, bet_size: u32) -> f64 {
        let bet_fraction = (bet_size as f64) / 100.0;
        pot * bet_fraction
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_new() {
        let pot = Pot::new(1.0, 1.0);
        assert_eq!(pot.pot, HashMap::from([(Player::IP, 1.0), (Player::OOP, 1.0)]));
    }

    #[test]
    fn test_get_win_amount() {
        let pot = Pot::new(1.0, 2.0);
        assert_eq!(pot.get_win_amount(Some(Player::IP)), 2.0);
        assert_eq!(pot.get_win_amount(Some(Player::OOP)), 1.0);
    }

    #[test]
    fn test_get_win_amount_in_draw() {
        let pot = Pot::new(1.0, 1.0);
        assert_eq!(pot.get_win_amount(None), 1.0);
    }

    #[test]
    fn test_total() {
        let pot = Pot::new(1.0, 1.0);
        assert_eq!(pot.total(), 2.0);
    }

    #[test]
    fn test_update_bet() {
        let mut pot = Pot::new(1.0, 1.0);
        pot.update(Player::OOP, Action::Bet(50));
        assert_eq!(pot.pot, HashMap::from([(Player::IP, 1.0), (Player::OOP, 2.0)]));
    }

    #[test]
    fn test_update_raise() {
        let mut pot = Pot::new(3.0, 1.0);
        pot.update(Player::OOP, Action::Raise(50));
        assert_eq!(pot.pot, HashMap::from([(Player::IP, 3.0), (Player::OOP, 6.0)]));
    }

    #[test]
    fn test_update_call() {
        let mut pot = Pot::new(2.0, 1.0);
        pot.update(Player::OOP, Action::Call);
        assert_eq!(pot.pot, HashMap::from([(Player::IP, 2.0), (Player::OOP, 2.0)]));
    }

    #[test]
    #[should_panic]
    fn test_update_fold() {
        let mut pot = Pot::new(1.0, 1.0);
        pot.update(Player::OOP, Action::Fold);
    }

    #[test]
    fn test_update_bet_call() {
        let mut pot = Pot::new(1.0, 1.0);
        pot.update(Player::OOP, Action::Bet(50));
        pot.update(Player::IP, Action::Call);
        assert_eq!(pot.pot, HashMap::from([(Player::IP, 2.0), (Player::OOP, 2.0)]));
    }

    #[test]
    fn test_update_bet_raise() {
        let mut pot = Pot::new(1.0, 1.0);
        pot.update(Player::OOP, Action::Bet(100));
        pot.update(Player::IP, Action::Raise(50));
        assert_eq!(pot.pot, HashMap::from([(Player::IP, 6.0), (Player::OOP, 3.0)]));
    }

    #[test]
    fn test_update_bet_raise_call() {
        let mut pot = Pot::new(1.0, 1.0);
        pot.update(Player::OOP, Action::Bet(100));
        pot.update(Player::IP, Action::Raise(50));
        pot.update(Player::OOP, Action::Call);
        assert_eq!(pot.pot, HashMap::from([(Player::IP, 6.0), (Player::OOP, 6.0)]));
    }
}