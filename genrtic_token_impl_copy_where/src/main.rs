use std::ops::Add;//스텐다드에 있는 토큰 중 + 토큰만 사용 하려고 모듈을 불러왔다!
#[derive(Debug, Clone, Copy)]//구조채에는 이개 있어야 println!("{:?}", 확인할 바인드); 를 사용 할수 있다 거이 필수!!
struct Asd<A, B, C> {//구조채를 만든다 뭐 기계로 치면 도면? 실채가 없다
    q:A,
    w:A,
    e:B,
    r:B,
    t:C,
    y:C,
}
trait Lll<A, B, C> {//함수의 집합채 라고 보면 된다 그냥 어떤 함수 쓸건지 정의만 한다고 보면 된다 기계로 치면 재료? 순서? 정도 ㅎㅎ
    fn new(q:A, w:A, e:B, r:B, t:C, y:C) -> Self;//이 함수는 참조가 없다 전부 넘김이다!!
    fn get_list(&self) -> [A;3];//구조채는 참조이나 배열은 넘김이다!
    fn get_vec(&self) -> (Vec<&B>, &str);//전부 참조이다 &str 과 String은 생선 되는곳이 틀리다 C언어 처럼 read-only 메모리가 따로 없다! 단 불변과 가변 const 바인드가 있을뿐 하지만 &str은 불변이다 가변으로 생성하더라도 불변 속성이며 읽기 전용이다 
    fn get_tupl(&self) -> (&A, &A, &B, &B, &C, &C, f32, f64);//
}
impl<A, B, C> Lll<A, B, C> for Asd<A, B, C>//위에 것들은 정의만 한거면 여기 부터 본편 구현 단계 말 그대로 조립 및 재작 뭐..default 구현은 안했지만...
where A: Add<Output = A> + Copy,{//여러게 정의 해야 할때 ***T: Add<Output = T> + Copy, (다른 ops 정의 ex B:Add<Output = B> + Copy 이러면 B도 + 토큰 사용 가능하다) ***
    fn new(q:A, w:A, e:B, r:B, t:C, y:C) -> Self {// 이 리턴에 Self 가 Asd 구조채 자채 라고 보면된다 이함수의 인자들 또한 넘김으로 보낸거다 new() 함수는 불려오고 처리가 끝나면 파괴 시키기 때문이다
        Self {//for Asd<A, B, C> 위에 있기때문에 Self 로 불러 올수 있다 참조가 아니기에 넘김으로 보낸거다
            q,//이름이 같으면 q:q, 할 필요가 없다!!
            w,
            e,
            r,
            t,
            y,
        }
    }

    fn get_list(&self) -> [A;3] {//인자를 추가하여 값을 더하던지 빼던지 할수 있다 단 연산에 따라 토큰도 추가 해줘야 하며 디폴트 옵션 및 match 또는 if 를 사용하여 유효성 검사와 연산을 해줘야 한다!
        [self.q + self.w, self.q, self.w]//보는 바와 같이 &self 가 아니다 copy() 함수는 암묵적으로 value 를 복사한다 return 또한 &[A;3] 처럼 앞에 & 가 없음으로 복사 값을 넘기는것이다!
    }//구조채 Asd 안의 값은 여전히 존재 한다

    fn get_vec(&self) -> (Vec<&B>, &str) {//이 함수에 a:&str 인자 추가 해주면 및 에 Fun rust 말고 인자로 보내서 바꿔서 리턴 보낼수도 있다 그야 물론 유효성 검사를 해줘야 하지만...
        (vec![&self.e, &self.r], "Fun rust")//Null () 로도 보내려면 몇가지 수정해야 한다.. 받을때도 Option 처리해서 some() 으로 걸러야 한다는 점이 좀 불편 하긴 하지만..
    }

    fn get_tupl(&self) -> (&A, &A, &B, &B, &C, &C, f32, f64) {//튜플은 타입 및 값을 더 추가 하는대 부담이 덜 하다 타입 상관없이 리턴을 시키면되기에 좀 편하지만 리턴 받을때 타입을 확인이 힘들다!
        (&self.q, &self.w, &self.e, &self.r, &self.t, &self.y, 2.34f32, 4.67f64)//통으로 받기때문에 안에 들어 있는걸 일일이 타입을 확인해야 한다는 수고로움이 있다만.. match로 타입 분리 하면 되기는 한다..
    }//위의 get_list() 함수가 불려와도 복사 되어진 값을 보낸 것이기에 그대로 참조가 가능하다 
}

fn main() {
    let mut c = Asd::new(123u32, 456u32, -123i32, -456i32, String::from("일단은 된다"), String::from("끝까지 한다"));//구현 되것을 실채화 했다 보면되다
    //mut 바인드 사용시 값을 변경할수 있다
    // let f = Asd::get_list(); 경로는 찾을 수 있으나 new() 함수의 Self 리턴 값이 없어서 Asd 는 없다!!
    println!("{:#?}", c);
    println!();
    println!("{1} + {2} = {0}", c.get_list()[0], c.get_list()[1], c.get_list()[2]);
    println!("{:#?}", c.get_list());
    println!();
    println!("{}, {}, {}", c.get_vec().0[0], c.get_vec().0[1], c.get_vec().1);
    println!("{:#?}", c.get_vec());
    println!();
    println!("{}, {}, {}, {}, {}, {}, {}, {}", c.get_tupl().0, c.get_tupl().1, c.get_tupl().2, c.get_tupl().3, c.get_tupl().4, c.get_tupl().5, c.get_tupl().6, c.get_tupl().7);
    println!("{:#?}", c.get_tupl());

    //c.new() 이함수는 불러올 수 없다 Self 반환이라 그런지 중간에 불러오기 및 새로 생성 하는개 안되고 바인드를 새로 생성해야 new() 함수를 불러올수 있다 c::new() 경로 로 찾을라 해도 안된다!
    //c::new(123u32, 456u32, -123i32, -456i32, String::from("안된다"), String::from("안되?")); 모듈을 찾을수 없다고 뜬다 rust 에서는 위에 c.new() 함수 포함 규칙때문에 안된다 라고 설명 하더라 

    //위에서 mut 바인드로 생성 했기때문에 구조체 안의 바인드를 따로 불러와 변결할수 있다! 근대 힙영역의 다이나믹 바인드는 변경 자채가 안되더라....아직 성능 시험중이라고 한다
    c.t="잘 되는군".to_owned();//new() 함수 말고는 접근이 가능하다 단 Self 리턴이 있는 함수가 반인드 생성과 동시에 불려 와서 Self 반환을 끝내놔야 한다!!
    c.y="으음 좋아~!".to_owned();//그리고 new() 이외에는 모든 함수인자가 &self 인자가 1개 로 설계 해 놔서 함수 불러서 따로 인자를 추가 할 수 없다! 
    println!("{:#?}", c);
    println!();
    println!("{1} + {2} = {0}", c.get_list()[0], c.get_list()[1], c.get_list()[2]);
    println!("{:#?}", c.get_list());
    println!();
    println!("{}, {}, {}", c.get_vec().0[0], c.get_vec().0[1], c.get_vec().1);
    println!("{:#?}", c.get_vec());
    println!();
    println!("{}, {}, {}, {}, {}, {}, {}, {}", c.get_tupl().0, c.get_tupl().1, c.get_tupl().2, c.get_tupl().3, c.get_tupl().4, c.get_tupl().5, c.get_tupl().6, c.get_tupl().7);
    println!("{:#?}", c.get_tupl());
    
}
/*
구조체 공부해서 혼자 해보니 엄청 빡세다 copy clone 차이를 구분 할수 있개 되었고 date와 value 의 차이도 확인 할 수 있었으며 
참조 때문에 빡치기도 여러번 빡첫고 하다보니 다중참조를 쓰고 있더라 흠 근대 3번 이상은 왠만하면 사용안하고 거이 대부분 1번만 참조 
하려고 하다보니 나중되니 어느정도 이해하면서 쓰긴 하더라 특히 역참조 때문에 많이 햇갈렷지 ㅎㅎ.. 그넘으 String array tupl 때문에 impl 에서 whewe 사용하다
머리에 지진 나고 제네릭 써본다고 생고생 다하면서 컴파일이랑 대화 하고 안저있고 하다보면 분명 오전에 시작 했는대 다음날 오전에 끝나기도 하고 ㅎㅎ 안풀리면 3일을 안자기도 하고
희안하개 그래도 잼있더라 ㅎㅎ 안질려 할때 만큼은 귀찬니즘이 좀 심해서 잘 안하기도 하지만.. 하면 기본 6시간은 붙들고 있으니 ㅎㅎ
*/