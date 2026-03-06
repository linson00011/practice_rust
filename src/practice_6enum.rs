use practice_rust::printlns_simple;

pub fn practice_enum(){
    process_ip();
}

//1.there are 2 fns,  2.we have a vec, call correct method for eachone.
//  !!!!!!!!! enum is beautiful.
fn show_msg_ipv4(ipadd:&i32)->String{
    format!("v4:{}",ipadd)
}

fn show_msg_ipv6(ipadd:&str)->String{
    format!("let 's go.:{}",ipadd)
}

enum IpAddress {
    Ipv4(i32),
    Ipv6(String)
}

impl IpAddress {
    fn call2fns(&self){
        match self {
            IpAddress::Ipv4(ip_data)=>{
                printlns_simple!(show_msg_ipv4(ip_data));
            },
            IpAddress::Ipv6(ip_data)=>{
                printlns_simple!(show_msg_ipv6(ip_data));
            }
        }
    }
}

fn process_ip(){
    let mut ip_address:Vec<IpAddress>=Vec::new();
    ip_address.push(IpAddress::Ipv4(1));
    ip_address.push(IpAddress::Ipv4(2));
    ip_address.push(IpAddress::Ipv4(3));
    ip_address.push(IpAddress::Ipv4(4));

    ip_address.push(IpAddress::Ipv6("ipv61".to_string()));
    ip_address.push(IpAddress::Ipv6("ipv62".to_string()));
    ip_address.push(IpAddress::Ipv6("ipv63".to_string()));
    ip_address.push(IpAddress::Ipv6("ipv64".to_string()));

    ip_address.iter().for_each(|the_ipaddr|{the_ipaddr.call2fns();});
}