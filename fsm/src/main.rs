use fsm::definition::*;

fn main() {
    let mut fsm = StateMachine::new();

    let string = "aaabd";

    for s in string.chars() {
        fsm.transition(s);
        println!("{:?}", fsm);
    }

    println!("{}", fsm.accepted().iter().collect::<String>());
}
