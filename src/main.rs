fn is_valid(value: u32) -> bool {
    let string_form = format!("{}", value);

    let mut chars = string_form.chars();
    let mut last_c = chars.next().unwrap();
    let mut found_pair = false;
    let mut count = 1;

    for c in chars {
        if c == last_c {            
            count += 1;
        } else {
            if count == 2 {
                found_pair = true;
            }
            count = 1;
        }

        if c < last_c {
            return false;
        }

        last_c = c;
    }

    if count == 2 {
        found_pair = true;
    }    

    found_pair
}

fn main() {
    let start : u32 = 136818;
    let end : u32 = 685979;

    let count = (start..end+1).filter(|x| is_valid(*x as u32)).count();
    println!("{}", count);
}
