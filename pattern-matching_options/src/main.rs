

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    addr: String
}

fn main() {
    let ip_addr = IpAddr{
        kind:IpAddrKind::V4,
        addr:String::from("127.0.0.1"),
    };
    let hm = ip_addr;
    println!("{}",hm.addr);
    // println!("{}",hm.kind);
    println!("Hello, world!");

    let some = Some(5);
    let ss = Some("Ansh");
    
}


fn plus_one(x: Option<i32>) ->Option<i32>{
    match x {
        None => Option::None,
        Some(i) => Option::Some(i+1)
    }
}