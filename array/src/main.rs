fn main() {
    let one = [1,2,3];
    let two:[u8;3] = [1,2,3];
    let blank1 = [0; 3];//0 이 3개 있는 배열
    let blank2:[u8; 3] = [0; 3];//위와 동일 하지만 u8 타입의 길이가 3인 배열을 생성하고 거기에 0을 자동으로 3개 채워 넣을때 사용하는 방식
    
    println!("{:?}", blank1);

    let arrays = [one, two, blank1, blank2];
    /*
    [
        [1,2,3], [1,2,3], [0; 3], [0; 3]
    ]
    */
    for a in &arrays {//2차원 배열 for문으로 순차적 데이터 가저오기
        print!("{:?}: ",a);//배열 안의 배열을 디버그로 표현하기 [1,2,3]
        for n in a.iter(){//가저온 배열안에 값 가저온다
            print!("\t{} + 10 = {} ", n, n+10);//가저온 값에 10을 더하여 출력한다
        }

        let mut sum = 0;//배열 값의 총 더한 값을 저장하기 위한 변환_바인더를 생성하여 초기화 한다
        for i in 0..a.len(){//배열 길이를 i값으로 대입한다
            sum+= a[i];//배열번호를 i값으로 선택하여 선택된 값을 전부 더하기
        }
        println!("\t(∑{:?} = {})", a, sum);//배열안의 값을 전부 더한 값을 출력하기
    }
    let mut str1 = "asd";//불변 속성의 스트링이다 단 바인드 재할당은 가능하다!!
    // str.push_str(" qwe"); 문자배열 자채에 추가 변경 삭재가 불가능하다!
    println!("{} {:p}", str1, str1);
    str1 = "asd qwe";//변경 한것처럼 보이지만 실재로는 새로 생성한 불변 문자열이다!
    println!("{} {:p}", str1, str1);

    let mut str2 = String::from("zxc");//힙영역에 생성된 가변 스트링이다
    
    println!("{} C는 확인할수 있지만 rust는 힙영역 주소값은 볼수 없다", str2);
    str2.push_str(" vbn");// 문자배열 자채에 추가 변경 삭재가 가능하다!
    println!("{} C는 확인할수 있지만 rust는 힙영역 주소값은 볼수 없다", str2);

}