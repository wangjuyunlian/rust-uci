use rust_uci::Uci;

fn main() {
    let mut uci = Uci::new().expect("unable to create UCI context");

    println!(
        "network.wan={}",
        uci.get("network.wan")
            .expect("unable to get value for network.wan")
    );
    // Get type of a section
    assert_eq!(
        uci.get("network.wan")
            .expect("unable to get value for network.wan"),
        "interface"
    );
    // Get value of an option, UCI's extended syntax is supported
    assert_eq!(
        uci.get("network.@interface[0].proto")
            .expect("unable to get value for network.@interface[0].proto"),
        "static"
    );
    assert_eq!(
        uci.get("network.lan.proto")
            .expect("unable to get value for network.lan.proto"),
        "static"
    );

    // Create a new section
    uci.set("network.newnet", "interface")
        .expect("unable to set network.newnet");
    uci.set("network.newnet.proto", "static")
        .expect("unable to set network.newnet.proto");
    uci.set("network.newnet.ifname", "en0")
        .expect("unable to set network.newnet.ifname");
    uci.set("network.newnet.enabled", "1")
        .expect("unable to set network.newnet.enabled");
    uci.set("network.newnet.ipaddr", "2.3.4.5")
        .expect("unable to set network.newnet.ipaddr");
    uci.set("network.newnet.test", "123")
        .expect("unable to set network.newnet.test");
    // Delete option
    uci.delete("network.newnet.test")
        .expect("unable to delete network.newnet.test");
    // IMPORTANT: Commit or revert the changes
    uci.commit("network").expect("unable to commit changes");
    uci.revert("network").expect("unable to revert changes");
}
