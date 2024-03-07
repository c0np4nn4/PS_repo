use std::io;

fn check_cond(v: &mut Vec<String>) -> bool {
    let mut vowel_cnt = 0;
    let mut conso_cnt = 0;

    for c in v.iter() {
        match c.as_str() {
            "a" => vowel_cnt += 1,
            "e" => vowel_cnt += 1,
            "i" => vowel_cnt += 1,
            "o" => vowel_cnt += 1,
            "u" => vowel_cnt += 1,
            //
            _ => conso_cnt += 1,
        }
    }

    if vowel_cnt < 1 || conso_cnt < 2 {
        return false;
    }

    true
}

fn find_combi(l: u64, c: u64, v: &mut Vec<String>, codes: Vec<String>, idx: usize) -> () {
    if v.len() == l as usize {
        if check_cond(v) == true {
            println!("{}", v.join(""));
        }
        ()
    }

    for i in idx..codes.len() {
        if v.contains(&codes[i]) == false {
            v.push(codes[i].clone());
        }
        find_combi(l, c, v, codes.clone(), i + 1);
        v.pop();
    }
}

fn main() -> io::Result<()> {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input)?;
    let mut l_and_c = input.split_ascii_whitespace();
    let l = l_and_c.next().unwrap().parse::<u64>().unwrap();
    let c = l_and_c.next().unwrap().parse::<u64>().unwrap();

    input.clear();
    io::stdin().read_line(&mut input)?;
    let mut codes: Vec<String> = input
        .split_ascii_whitespace()
        .map(|c| c.to_string())
        .collect();

    codes.sort();

    let mut combi = vec![];
    find_combi(l, c, &mut combi, codes, 0usize);

    Ok(())
}

