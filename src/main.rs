use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 
    {
        panic!("Insufficient arguments.");
    }

    let mut OpCharNum = 0;
    let mut seq = 0;
    let mut init = true;
    let mut OpChar: String = String::new();
    let mut count = 1;
    
    for c in args[1].chars()
    {
        let n = c.to_digit(16);
        OpCharNum += match n {
            Some(m) => if OpCharNum > 15 { m }
                       else { m << 4 },
            None    => 0 };
        match n {
            Some(m) => { if seq == 1 
                         { OpChar += char::from_u32(OpCharNum)
                             .unwrap().to_string().as_str(); }
                         seq = ( seq + 1 ) % 2; 0},
            None    => { OpCharNum = 0; 0 },
        };
        if ( count == args[1].chars().count() ) 
        { println!("{OpChar}"); }
        count += 1;
    }
}
