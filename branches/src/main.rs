fn main() {
    let hoge = 10;
    let reference = &hoge;
    let &copied = reference;
    assert_eq!(copied, 10);
    println!("hoge:   {:p}", &hoge);
    println!("refarence:{:p}", reference);
    println!("copied: {:p}", &copied);
}