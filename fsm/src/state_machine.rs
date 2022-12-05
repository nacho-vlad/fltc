#[macro_export]
macro_rules! finite_state_machine {
    (
        states = {$($state:ident),+},
        init = $init:ident,
        accept = {$($accept:ident),+},
        alphabet = $alphabet:ty,
        transitions = {
            $($s:ident,$q:pat => $sq:ident),*,
        }
    ) => {

        type Alphabet = $alphabet;

        #[derive(PartialEq, Debug, Copy, Clone)]
        pub enum State {
            $($state,)+
            Invalid
        }

        impl State {
            pub fn accept(&self) -> bool {
                match self {
                    $(State::$accept => true,)+
                    _ => false
                }
            }

            pub fn transition(self, q: Alphabet) -> Self {
                match (self, q) {
                    $((State::$s, $q) => State::$sq,)*
                    _ => State::Invalid
                }
            }
        }

        #[derive(Debug, Clone)]
        pub struct StateMachine {
            pub state: State,
            symbols: Vec<Alphabet>,
            accepted: usize,
        }

        impl StateMachine {
            pub fn new() -> Self {
                StateMachine {
                    state: State::$init,
                    symbols: Vec::new(),
                    accepted: 0,
                }
            }

            pub fn transition(&mut self, q: Alphabet) {

                self.state = self.state.transition(q);

                if !self.valid() {
                    return;
                }

                self.symbols.push(q);

                if self.state.accept() {
                    self.accepted = self.symbols.len();
                }
            }

            pub fn valid(&self) -> bool {
                self.state != State::Invalid
            }

            pub fn accepted(&self) -> Vec<Alphabet> {
                self.symbols[..self.accepted].to_vec()
            }
        }


    };
}

/*
type Transition = char;

enum State {
    A,
    B,
    C,
    Invalid,
}

impl State {
    fn next(self, transition: Transition) -> Self {
        match (self, transition) {
            (State::A, 'a') => State::B,
            default => State::Invalid,
        }
    }
}

trait FiniteAutomata {
    fn terminal(&self) -> bool;

    fn transition(self, trans: Transition) -> Self;

    fn accepted(&self) -> Vec<Transition>;
}

struct StateMachine {
    state: State,
    symbols: Vec<Transition>,
    accepted: usize,
}

impl StateMachine {
    fn new() -> Self {
        StateMachine {
            state: State::A,
            symbols: Vec::new(),
            accepted: 0,
        }
    }

    fn terminal(&self) -> bool {}

    fn accepted(&self) -> Vec<Transition> {}
}
*/

/*
state_machine! {
    states = {A, B, C},
    init = A,
    final = {C, B},
    alphabet = char,
    transitions = {
        A, a -> B
        B, b -> C
    }
}
*/
