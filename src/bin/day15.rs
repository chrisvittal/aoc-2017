
struct DuelGen {
    prev: u64,
    factor: u64,
}

impl DuelGen {
    const MOD: u64 = 2147483647;
    const BITMASK: u64 = 0xFFFF;

    fn new(start: u64, factor: u64) -> Self {
        DuelGen {
            prev: start,
            factor,
        }
    }
}

impl Iterator for DuelGen {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        self.prev = (self.prev * self.factor) % Self::MOD;
        Some(self.prev & Self::BITMASK)
    }
}

struct FilteredDuelGen {
    gen: DuelGen,
    filter: u64,
}

impl FilteredDuelGen {
    fn new(start: u64, factor: u64, filter: u64) -> Self {
        FilteredDuelGen {
            gen: DuelGen {
                prev: start,
                factor,
            },
            filter
        }
    }
}

impl Iterator for FilteredDuelGen {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let f = self.filter;
        self.gen.find(|v| v % f == 0)
    }
}

const AIN: u64 = 289;
const BIN: u64 = 629;
const AMOD: u64 = 4;
const BMOD: u64 = 8;
const AMUL: u64 = 16807;
const BMUL: u64 = 48271;

fn main() {
    let a = DuelGen::new(AIN, AMUL);
    let b = DuelGen::new(BIN, BMUL);
    let cnt = a.zip(b).take(40_000_000).filter(|&(a,b)| a == b).count();
    println!("1: {}",cnt);

    let a = FilteredDuelGen::new(AIN, AMUL, AMOD);
    let b = FilteredDuelGen::new(BIN, BMUL, BMOD);
    let cnt = a.zip(b).take(5_000_000).filter(|&(a,b)| a == b).count();
    println!("2: {}", cnt);
}

