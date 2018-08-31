fn main() {
    eratosthenes_indexing(30);
}

fn eratosthenes (max: u32) {
    let mut list = Vec::new();
    let mut index: u32 = 2;
    let sqrt_max = (max as f32).sqrt().floor() as u32;

    while index <= max {
        list.push(index);
        index += 1;
    }

    println!("{:?}", list);

    for i in 0..sqrt_max {
        let step_size = list[i as usize] as usize;
        println!("step: {} {}", step_size, i);

        if step_size == 0 {
            continue;
        }

        for value in list[i as usize..].iter_mut().step_by(step_size).skip(1) {
            println!(" {}", *value);
            *value = 0;
        }
    }

    for value in list.iter() {
        if *value != 0 {
            println!("{}", value);
        }
    }
}

fn eratosthenes_indexing(max: u32) {
    let mut list = Vec::new();
    let sqrt_max = (max as f32).sqrt().floor() as u32;

    for i in 2..max + 1 {
        list.push(i);
    }

    for i in 0..sqrt_max {
        let mut j = i;
        while j < list.len() as u32 - 1 {
            if list[i as usize] == 0 {
                break;
            }

            j += list[i as usize];

            if j < list.len() as u32{
                list[j as usize] = 0;   
            }
            
        }
    }

    println!("{:?}", list);
}