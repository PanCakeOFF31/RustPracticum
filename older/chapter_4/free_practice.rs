use std::string::ToString;

const GLOBAL: &str = "home work";

pub(crate) fn main() {
    // immutable var не могу: присвоить другое значение
    let a = 10;
    // a = 20;

    // mutable var могу: присвоить другое значение
    let mut b = 10;
    b = 20;

    // immutable var не могу: присваивать другое значение и изменять
    let s = "asd".to_string();
    // s.push_str("asd");

    // mutable var могу: присваивать другое значение и изменять
    let mut s = "asd".to_string();
    s.push_str("asd"); // изменяю непосредственно сам объект
    s = "gsd".to_string(); // изменяю указатель - присваиваю другой объект
}
