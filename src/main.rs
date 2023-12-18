fn main() {
    let p = 1511;
    let q = 2003;
    let d = 1234577;
    let n = p * q;
    // let dk
    let mp = odwrotnosc_multiplikatywna(q, p);
    let mq = odwrotnosc_multiplikatywna(p, q);
    let dp = modulo_euclid(d, p-1);
    let dq = modulo_euclid(d, q-1);
    println!("mp = {mp}");
    println!("mq = {mq}");
    println!("dp = {dp}");
    println!("dq = {dq}");

    let y = 152702;
    let x = potegowanie(y, d as u128, (n) as u128);
    let xp = potegowanie(y, dp as u128, p as u128) as i128;
    let xq = potegowanie(y, dq as u128, q as u128) as i128;
    println!("xp = {xp}");
    println!("xq = {xq}");
    let x_too = modulo_euclid(mp * q * xp + mq * p * xq, n);
    println!("x = {x}");
    println!("x_too = {x_too}");

    println!("Hello, world!");
}

fn potegowanie(a: u128, e: u128, n: u128) -> u128 {
    let e_binary = reverse(create_binary(e));
    let mut d = 1;
    let mut i = e_binary.len() as i32 -1;
    while(i >= 0){
        d = modulo_euclid(d*d, n as i128);
        if e_binary[i as usize] == 1{
            d = modulo_euclid(d * a as i128, n as i128)
        }

        i -= 1;
    }
    return d as u128;
}

fn create_binary(value: u128) -> Vec<u128>{
    let binary_string = format!("{:b}", value);
    let res = binary_string.chars().into_iter().map(|c| if c == '0' {0} else {1}).collect();
    return res;
}

fn reverse(vec: Vec<u128>) -> Vec<u128>{
    let mut vec_clone = vec.clone();
    vec_clone.reverse();
    // vec_clone.sort_by_key(|&num| (false , Reverse(num)));
    return vec_clone;
}


fn modulo_euclid(j: i128, k: i128) -> i128 {
    let res =  j % k;
    if res < 0 {return res + k} else {return res}
}

fn rozNWD(j: i128, k: i128) -> (i128, i128, i128) {
    if j == 0 {
        return (k, 0, 1)
    }
    // let r = k % j;
    let r = modulo_euclid(k, j);
    // let r = k % j;
    let (d, x_prim, y_prim) = rozNWD(r, j);
    let x = y_prim - (k/j) * x_prim;
    let y = x_prim;
    return (d, x, y);

    // (k, 1, 0)
}


fn odwrotnosc_multiplikatywna(j: i128, k: i128) -> i128 {
    // a to jest 17 czyli j
    // n = 101 czyli k
    let (_d, x, _y) = rozNWD(j, k);
    // println!("d = {d}, x = {x}, y = {y}");
    return modulo_euclid(x, k);
}
