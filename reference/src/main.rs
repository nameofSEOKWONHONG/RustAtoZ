fn main() {
    let mut s = String::from("hello");

    // {
    //     let r1 = &mut s;
    //     println!("{}", r1);
    // } // 여기서 r1은 스코프 밖으로 벗어났으므로, 우리는 아무 문제 없이 새로운 참조자를 만들 수 있습니다.

    //let r1 = &mut s;
    //let r2 = &mut s;

    // 데이터 레이스 조건 (error)
    // 1.두 개 이상의 포인터가 동시에 같은 데이터에 접근한다.
    // 2.그 중 적어도 하나의 포인터가 데이터를 쓴다.
    // 3.데이터에 접근하는데 동기화를 하는 어떠한 메커니즘도 없다.

    //println!("{}, {}", r1, r2); //error


    let r1 = &s; //문제없음
    let r2 = &s; //문제없음
    let r3 = &mut s; //error (가변 변수를 불변 변수가 참조할 수 없음)

    no_dangle();
}

//error (즉, 참조를 반환하면 안됨. 스코프끝에서 참조 대상 변수가 drop됨
// fn dangle() -> &String { // dangle은 String의 참조자를 반환합니다
//
//     let s = String::from("hello"); // s는 새로운 String입니다
//
//     &s // 우리는 String s의 참조자를 반환합니다.
// } // 여기서 s는 스코프를 벗어나고 버려집니다. 이것의 메모리는 사라집니다.
// // 위험하군요!

//no error
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}