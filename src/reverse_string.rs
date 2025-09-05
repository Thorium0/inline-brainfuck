mod brainfuck;

fn main() {
    brainfuck! {
        > offset by 1
        + allow enter
        [>,] save input
        + allow enter loop
        [<.] print input in reverse
    }
}
