mod a_options;
mod a2_enum;

fn main() {
    enum_base1();
    base_poker_card();
    enum_message();
}

#[derive(Debug)]
enum PokerSuit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

struct PokerCard {
    suit: PokerSuit,
    value: u8,
}

fn base_poker_card() {
    let c1 = PokerCard {
        suit: PokerSuit::Clubs,
        value: 1,
    };
    let c2 = PokerCard {
        suit: PokerSuit::Diamonds,
        value: 2,
    };
}

enum PokerSuit2 {
    Clubs(u8),
    Spades(u8),
    Diamonds(char),
    Hearts(char),
}

fn base_poker_card2() {
    let c1 = PokerSuit2::Clubs(10);
    let c2 = PokerSuit2::Diamonds('K');
}

enum Message {
    Quit,
    // Quit 没有任何关联数据
    Move { x: i32, y: i32 },
    // Move 包含一个匿名结构体
    Write(String),
    // Write 包含一个 String 字符串
    ChangeColor(i32, i32, i32),     // ChangeColor 包含三个 i32
}

fn enum_message() {
    let m1 = Message::Quit;
    let m2 = Message::Move { x: 1, y: 2 };
    let m3 = Message::Write("hello".to_string());
    let m4 = Message::ChangeColor(1, 2, 3);

    if let Message::Move { x, y } = m2 {
        println!("if conditions x: {}, y: {}", x, y);
    } else {
        println!("not move");
    }
}

fn print_suit(cared: PokerSuit) {
    println!("{:?}", cared);
}

fn enum_base1() {
    let heart = PokerSuit::Hearts;
    let diamond = PokerSuit::Diamonds;
    print_suit(heart);
    print_suit(diamond);
}
