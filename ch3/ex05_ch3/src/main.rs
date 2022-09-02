/*
    전역변수 에서 오류 코드를 확인하는 러스트 비슷한 코드
*/

static mut ERROR:i32 = 0;
//..

fn main() {
    let mut f = File::new("something.txt");
    read(f,buffer);
    unsafe{
        if ERROR !=0 {
            panic!("An error has occurred while reading the file")
        }
    }

    close(f);
    unsafe{
        if ERROR !=0{
            panic!("An error has occurred while closing the file")
        }
    }
}
