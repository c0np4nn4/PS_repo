use std::{fmt::Display, io};

struct AnsFormat(Vec<u64>);

impl Display for AnsFormat {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        print!("<");
        for i in self.0.iter() {
            if i == self.0.last().unwrap() {
                print!("{}", i);
                break;
            }
            print!("{}, ", i);
        }
        print!(">");

        Ok(())
    }
}

fn circular(n: u64, k: u64) -> AnsFormat {
    let mut check: [bool; 5001] = [false; 5001];

    let mut idx: u64 = k;
    let mut cnt: u64 = 0;
    let mut tck: u64 = 0;

    let mut res: Vec<u64> = Vec::new();

    while cnt < n {
        if tck % k == 0 && check[idx as usize] == false {
            res.push(idx);

            check[idx as usize] = true;

            cnt += 1;
        }

        if check[(idx % n + 1) as usize] == true {
            idx = idx % n + 1;
        } else {
            idx = idx % n + 1;
            tck += 1;
        }
    }

    AnsFormat(res)
}

fn main() -> io::Result<()> {
    let mut input = String::from("");
    io::stdin().read_line(&mut input)?;

    let vec = input.trim().to_string();
    let vec = vec.split_whitespace().collect::<Vec<&str>>();

    let n: u64 = vec[0].parse::<u64>().unwrap();
    let k: u64 = vec[1].parse::<u64>().unwrap();

    let ans = circular(n, k);

    println!("{}", ans);

    Ok(())
}
