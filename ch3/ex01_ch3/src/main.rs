#![allow(unused_variables)]

type File = String;

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

#[allow(dead_code)]
fn read(f:&mut File, save_to:&mut Vec<u8>)->!{ 
    // ! 반환타입은 이함수가 어떤 값도 반환 하지 않는다고 컴파일러에게 알려주는 역할
    unimplemented!()
}

fn main() {
    let mut f1= File::from("f1.txt");
    open(&mut f1);
    //read(f1,vec![]);
    close(&mut f1);

}
