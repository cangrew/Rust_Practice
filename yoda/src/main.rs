use std::io::stdin;

fn main() {
    let mut a = String::new();
    stdin()
        .read_line(&mut a)
        .unwrap();
    let mut b = String::new();
    stdin()
        .read_line(&mut b)
        .unwrap();
    a = a.trim().to_owned();
    b = b.trim().to_owned();


    let  a1 = a.as_bytes();
    let  b1 = b.as_bytes();

    let a_len = a.len();
    let b_len = b.len();

    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    if a_len == b_len  {
        for i in 0..a_len {
            if a1[i] > b1[i] {
                vec1.push(a1[i] as char);
            }
            else if a1[i] == b1[i] {
                vec1.push(a1[i] as char);
                vec2.push(b1[i] as char);
            }
            else if a1[i] < b1[i] {
                vec2.push(b1[i] as char);
            }
        }
    }
    else if a_len > b_len {
        let dif = a_len - b_len;
        for i in 0..dif {
            vec1.push(a1[i] as char);
        }
        for i in dif..a_len {
            if a1[i] > b1[i - dif] {
                vec1.push(a1[i] as char);
            }
            else if a1[i] == b1[i - dif] {
                vec1.push(a1[i] as char);
                vec2.push(b1[i - dif] as char);
            }
            else if a1[i] < b1[i - dif] {
                vec2.push(b1[i - dif] as char);
            }
        }
    }
    else if a_len < b_len {
        let dif = b_len - a_len;
        for i in 0..dif {
            vec2.push(b1[i] as char);
        }
        for i in dif..b_len {
            if a1[i - dif] > b1[i] {
                vec1.push(a1[i - dif] as char);
            }
            else if a1[i - dif] == b1[i] {
                vec1.push(a1[i - dif] as char);
                vec2.push(b1[i] as char);
            }
            else if a1[i - dif] < b1[i] {
                vec2.push(b1[i] as char);
            }
        }
    }

    if vec1.is_empty() {
        print!("YODA");
    }
    else {
        let pa : String = vec1
            .iter()
            .collect();
        let pa : u32 = pa
            .parse()
            .unwrap(); 
        print!("{}",pa);
    }
    println!("");

    if vec2.is_empty() {
        print!("YODA");
    }
    else {
        let pb : String = vec2
            .iter()
            .collect();
        let pb : u32 = pb
            .parse()
            .unwrap(); 
        print!("{}",pb);
    }
    println!("");

}

