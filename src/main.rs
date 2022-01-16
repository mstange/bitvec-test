use bitvec::prelude::*;

fn main() {
    let mut args = std::env::args().skip(1);
    if args.len() < 1 {
        eprintln!("Usage: {} <entry>", std::env::args().next().unwrap());
        std::process::exit(1);
    }
    let entry = args.next().unwrap();
    let entry: u32 = if let Some(hexstr) = entry.strip_prefix("0x") {
        u32::from_str_radix(hexstr, 16).unwrap()
    } else {
        entry.parse().unwrap()
    };

    eprintln!(
        "24 bits: {}",
        extract_24(entry)
    );
}

#[inline(never)]
fn extract_24(val: u32) -> u32 {
    let bv: BitArray<u32, Msb0> = BitArray::new(val);
    bv[8..32].load_le()
}
