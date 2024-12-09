const H: [u32; 8] = [
    0x6a09e667,
    0xbb67ae85,
    0x3c6ef372,
    0xa54ff53a,
    0x510e527f,
    0x9b05688c,
    0x1f83d9ab,
    0x5be0cd19,
];

const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];
fn ochered(a:Vec<String>)->Vec<String>{
    let mut new_vec = Vec::with_capacity(64);
    let mut res:String = String::new();
    for i in a.into_iter(){
        if res.len() < 32{
            res.push_str(&*i)
        }
        else{
        new_vec.push(res.clone());
        res.clear();
        res.push_str(&*i);
        }
    };
    if !res.is_empty(){new_vec.push(res)}
    (0..64 - new_vec.len()).for_each(|_|new_vec.push(format!("{:032b}",0)));
    new_vec
}
fn rotate_right(value:&str,count:u32)->String{
    let bits = 32;
    let value_int = u32::from_str_radix(value,2).unwrap();
    let res = (value_int>>count)|(value_int<<(bits - count));
    let res = format!("{:032b}",res);
    res
}
fn rotate_left(value:&str,count:u32)->String{
    let bits = 32;
    let value_int:u128 = value.parse().unwrap();
    format!("{:032b}",(value_int<<count)|(value_int>>(bits - count)))
}
fn xor(a:String,b:String)->String{
    let value_a = u32::from_str_radix(&a,2).unwrap();
    let value_b = u32::from_str_radix(&b,2).unwrap();
    format!("{:032b}",value_a ^ value_b)
}
fn bin(a:Vec<u8>) ->Vec<String>{
    a.into_iter().map(|mut i|{
        let mut res = String::new();
        while i>0{
            res.push_str((i%2).to_string().as_str());
            i/=2;
        }
        while res.len()< 8{
        res.push('0');}
        res.chars().rev().collect::<String>()
    }).collect::<Vec<String>>()
}
fn leveling(mut a:Vec<String>)->Vec<String>{
    let len = format!("{:0b}",a.len() * 8);
    a.push(format!("{:08b}",1).chars().rev().collect());
    while a.len() < 64 - 1 {
        a.push(format!("{:08b}",0))
    }
    a.push(len);
    a
}


fn main() {
    let target= "hello world";
    let chars = target.chars().map(|i| i as u8).collect::<Vec<u8>>();
    let byte = bin(chars);
    let leveling = leveling(byte);
    let mut res = ochered(leveling);
    for i in 16..res.len(){
        let s0 = u32::from_str_radix(&xor(rotate_right(&res[i-15],7) , rotate_right(&res[i-15],18)),2).unwrap() ^  u32::from_str_radix(&res[i-15],2).unwrap() >> 3;
        let s1 = u32::from_str_radix(&xor(rotate_right(&res[i-2],17) , rotate_right(&res[i-2],19)),2).unwrap() ^ u32::from_str_radix(&res[i-2],2).unwrap() >> 10;
        res[i] = format!("{:032b}",u32::from_str_radix(&res[i - 16],2).unwrap().wrapping_add(s0).wrapping_add(u32::from_str_radix(&res[i - 7],2).unwrap()).wrapping_add(s1));
    }
    let mut a = H[0];
    let mut b = H[1];
    let mut c = H[2];
    let mut d = H[3];
    let mut e = H[4];
    let mut f = H[5];
    let mut g = H[6];
    let mut h = H[7];
    for i in 0..=63{
        let e2 = format!("{:032b}",e);
        let a2 = format!("{:032b}",a);
        let q1 = u32::from_str_radix(&xor(rotate_right(&e2,6) , rotate_right(&e2,11)),2).unwrap() ^ u32::from_str_radix(&rotate_right(&e2,25),2).unwrap();
        let q0 = u32::from_str_radix(&xor(rotate_right(&a2,2), rotate_right(&a2,13)),2).unwrap() ^ u32::from_str_radix(&rotate_right(&a2,22),2).unwrap();
        let majority = (a & b) ^ (a & c) ^ (b & c);
        let choice = (e & f) ^ ((!e) & g);
        let temp1 = h.wrapping_add(q1).wrapping_add(choice).wrapping_add(K[i]).wrapping_add(u32::from_str_radix(&res[i],2).unwrap());
        let temp2 = q0.wrapping_add(majority);
        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(temp1);
        d = c;
        c = b;
        b = a;
        a = temp1.wrapping_add(temp2);
    }
    let mut final_hash = [a,b,c,d,e,f,g,h];
    let final_hash = (0..final_hash.len()).into_iter().map(|i|format!("{:08x}",final_hash[i].wrapping_add(H[i]))).collect::<String>();
    println!("{:?}",final_hash)

}
