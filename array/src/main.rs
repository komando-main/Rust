fn main() {
    let one = [1,2,3];
    let two:[u8;3] = [1,2,3];
    let blank1 = [0; 3];//0 이 3개 있는 배열
    let blank2:[u8; 3] = [0; 3];//위와 동일 하지만 u8 타입의 길이가 3인 매열을 생성하고 거기에 0을 자동으로 3개 채워 넣을때 사용하는 방식
    
    println!("{:?}", blank1);

    let arrays = [one, two, blank1, blank2];
    /*
    [
        [1,2,3], [1,2,3], [0; 3], [0; 3]
    ]
    */
    for a in &arrays {//2차원 배열 for문으로 순차적 데이터 가저오기
        print!("{:?}: ",a);
        for n in a.iter(){
            print!("\t{} + 10 = {} ", n, n+10);
        }

        let mut sum = 0;
        for i in 0..a.len(){
            sum+= a[i];
        }
        println!("\t(∑{:?} = {})", a, sum);
    }
}