
/// This solution has been hevily optimized by hand disassmbling the input.
/// The input is reproduced below for reference

/**
```text
set b 84
set c b
jnz a 2
jnz 1 5
mul b 100
sub b -100000
set c b
sub c -17000
set f 1
set d 2
set e 2
set g d
mul g e
sub g b
jnz g 2
set f 0
sub e -1
set g e
sub g b
jnz g -8
sub d -1
set g d
sub g b
jnz g -13
jnz f 2
sub h -1
set g b
sub g c
jnz g 2
jnz 1 3
sub b -17
jnz 1 -23
```
*/

// TODO: add some parsing of the input to determine initial values of b and c
fn main() {
    println!("{}", run1(84));
    println!("{}", run2(84*100 + 100_000, 17_000, 17));
}

fn run2(mut b: i64, c_off: i64, step: i64) -> u32 {
    // NB: Hand disassembly
    let mut h = 0;
    let c = b + c_off;
    while b <= c {
        let up = (b as f64).sqrt() as i64 + 1;
        for d in 2..up {
            if b % d == 0 {
                h += 1;
                break;
            }
        }
        b += step
    }
    h
}

fn run1(b: i64) -> i64 {
    // NB: Hand disassembly of part 1.
    (b-2) * (b-2)
}
