#[cfg(test)]
mod tests {

    use pnet::datalink;

    #[test]
    fn test() {
        let interfaces = datalink::interfaces();
        let lan = interfaces
            .iter()
            .find(|iface| iface.name == "wlan0")
            .unwrap();

        println!("{} {:?}", lan.name, lan.ips);
    }
}
