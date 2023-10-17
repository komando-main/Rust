#![allow(unused_mut)]
#![allow(unused_must_use)]
use std::any::type_name;
fn type_of<T>(_: &T) -> &str {
    type_name::<T>()
}
fn a000() -> u8 {
    A123
}

fn a123() -> u8 {
    let a123:u8 = 100; //함수 안에 값을 생성하고
    a123 //이때 값을 넘긴다
}

fn a111( c:u8 ) -> u8 {
    c+100
}

fn a222() -> u8 {
    let s:u8 = 20;
    A123+s //10+30
}

// let a123:u8 = 10; 오로지 함수 안에서만 생성 가능하다 let mut 포함!!
static mut B123:u8 = 10;//스테틱과 컨스트는 타입을 무조건 지정해줘야 한다!!!
static mut C123:u8 = 10;
const A123:u8 = 10;//어느영역 이든 생성 가능하다 이 경우는 프로그램이 종료 될때까지 유지된다 상수는 값이 정해지면 불변이다
const A:u8 = 110;

fn main() {
    let x:u8 = 5;// 1바이트 (8비트)
    let p = &x as *const u8;
    println!("{:?}", p);
    let p_plus_one = unsafe { p.offset(1) };//1바이트 이동
    println!("{:?}", p_plus_one);// 1바이트 (8비트)
    println!();
    println!();

    let x1:u32 = 5;//4바이트 (32비트)
    let p1 = &x1 as *const u32;
    println!("{:?}", p1);
    let p_plus_one1 = unsafe { p1.offset(1) };//4바이트 이동
    println!("{:?}", p_plus_one1);// 4바이트 (32비트)
    println!();
    println!();
    
    let x2:u64 = 5;//8 바이트 (64비트)
    let p2 = &x2 as *const u64;
    println!("{:?}", p2);
    let p_plus_one2 = unsafe { p2.offset(1) };//8바이트 이동
    println!("{:?}", p_plus_one2);//8 바이트 (64비트)
    println!();
    println!();
    
    let x3:u128 = 5;
    let p3 = &x3 as *const u128;
    println!("{:?}", p3);
    let p_plus_one3 = unsafe { p3.offset(1) };//16바이트 이동
    println!("{:?}", p_plus_one3);//16 바이트 (128비트)
    println!();
    println!();
    
    let mut num1 = 0;
    unsafe{//안전하지 않은 코드를 실행해야 하는 경우 사용
        println!("110 {} {:p}", A, &A);
        println!("10  {}  {:p}", A123, &A123);
        println!("10  {}  {:p} {}", B123, &B123, type_of(&B123));//코드가 안전하지 않음으로 프로그래며가 직접 책임져야 한다!
        println!("10  {}  {:p} {}",C123, &C123, type_of(&B123));
        println!("10  {}  {:p} {}",C123, &B123, type_of(&B123));
        if type_of(&B123) == "u8" {
            num1+=B123;
        }
    }
    println!("{}", num1);
    
    let mut asd:&u8 = &A;// 변수 옵션에 참조로 받는다라고 설정 했기때문에 참조로 값을 설정해줘야 한다(주소값을 받는 바인드)
    let mut a1=A;
    println!("{}", a1);
    a1 = 150;
    println!("{} {:p}", asd, asd);
    println!("{}", a1);
    asd = &200;
    let asd1:&u8 = asd;
    let asd2:u8 = *asd;
    println!("{}", asd1);
    println!("{}", asd2);
    
    println!("{} {:p} {:p} {:p} {}", asd, &asd, *(&asd), &110, &110);
    println!("{} {:p} {:p} {:p} {}", asd, &asd, *&asd, &110, &110);

    // if 110 != &110 { 타입이 같은경우에만 실행되며 타입이 다른경우 오류난다
    //     println!("110 == &110 같지 안다");
    // }

    // if 1 == '1' { 타입이 같은경우에만 실행되며 타입이 다른경우 오류난다
    //     println!("같지 안다")
    // }
    
    asd = &10;
    println!("{}", asd);

    println!("전 영역 상수 바인드 {} 주소   {:p}", A123, &A123);
    
    if A123 == a000() {
        println!("A123과 a000()의 값은 값다 {} {}", A123, a000());
    }



    let ppp1 = &a000();
    // if &&A123 == &ppp1 { //상수라서 주소비교를 못하개 막아 놧나보다
    //     println!("A123과 a000()의 주소는 다르다 {:p} {:p}", &A123, ppp1);
    // }

    println!("함수 return 값 {} 함수의 주소 {:p} 위와 주소는 다르다", a000(), &a000());
    println!("함수 return 값 {} 함수의 주소 {:p} 위와 주소는 다르다", a000(), &a000());//함수는 불려올때마다 주소가 바뀐다
    println!("1번 ppp1이 담고있는 &a000   주소 {:p} ppp1의 주소    {:p}", ppp1, &ppp1);
    
    println!("2번 ppp1이 담고있는 &a000   주소 {:p} &a000()의 주소 {:p}", ppp1, &a000());
    // if &A123 == &a000() {//뭐냐? != 이개 안먹어?
    //     println!("하지만 A123과 a000()의  주소는 다르다 A123 {:p} != a000() {:p}", &A123, &a000());
    // }

    println!("함수안의 불변 바인드 {} 주소 {:p}", a123(), &a123());
    println!("상수 바인드를 인자로 넘기는 것도 가능 {} 주소 {:p}", a111(A123), &a111(A123));
    println!("매인을 거치지안고 바로 함수영역에 들어 가는것도 가능 {} 주소 {:p}", a222(), &a222());
    let n = 32i32;
    let c = &n;
    let m = &n;
    let mut x = &n;
    let mc = &m;
    let m1 = &mc;
    let m2 = &m1;
    let m3 = &m2;
    let cc = &c;
    let xc = &x;

    println!("c 가 담고있는 주소    {:p} \n&n을 직접 주소로 표현 {:p}", c, &n);
    if c == &n {
        println!("c == &n {:p} {:p} \n보는 바와 같이 바인드가 담고있걸 보여주냐? 바인드주소를 직접 표현 하느냐에 차이일뿐 &가 있고 없고의 차이가 있다 \n그야 물론 &c는 자신의 주소를 표현 하라 고 하기때문에 틀리다! {:p}", c, &n, &c);
    }else{
        println!("잘못 이해 했나?");
    }

    let (qwe, asd, zxc) = (-10i32, 10u8, 1000u64);
    println!("{:p}, {:p}, {:p}", &qwe, &asd, &zxc);
    let (lll1, lll2, lll3) = (&qwe, &asd, &zxc);
    println!("{} {} {}", qwe, asd, zxc);
    println!("{:p}\n{:p}\n{:p}", lll1, lll2, lll3);

    let [qwe1, qwe2, qwe3] = [15i32;3];
    let [lll0, lll9, lll8] = [&lll1, &qwe2, &qwe3];
    println!("{} {} {}", lll1, qwe2, qwe3);
    println!("{:p}\n{:p}\n{:p}", lll0, lll9, lll8);

    // let [qaz, wsx, edc] = [&lll1, &qwe1, &lll3];//let [qaz, wsx, edc] = [&lll1, &qwe1, &lll3];
    // println!("{:p}\n{:p}\n{:p}", qaz, wsx, edc);//                                     ^^^^^ expected `&i32`, found `&&u64` 배열은 무조건 모든 타입이 같아야 한다!

    let (qaz5, wsx6, edc7) = (&lll2, &qwe1, &lll3);
    println!("{:p}\n{:p}\n{:p}", qaz5, wsx6, edc7);
    println!("{}\n{}\n{}", qaz5, wsx6, edc7);

    println!("c가 바라보는 주소     {:p}", c);
    println!("m이 바라보는 주소     {:p}", m);
    println!("x가 바라보는 주소     {:p}\n", x);
    println!("mc가 바라보는 m  주소 {:p}", mc);
    println!("m1이 바라보는 mc 주소 {:p}", m1);
    println!("m2가 바라보는 m1 주소 {:p}\n", m2);
    println!("c가  생성된 주소      {:p}", cc);
    println!("x가  생성된 주소      {:p}", xc);
    println!("mc가 생성된 주소      {:p}", m1);
    println!("m1이 생성된 주소      {:p}", m2);
    println!("m2가 생선된 주소      {:p}\n", m3);
    println!("모든 변수의 값 {} {} {} {} {} {} {} {} {}\n", n, m, mc, m1, m2, c, cc, x, xc);
    println!("모두 더한값 {}", n + *m + **mc + ***m1 + ****m2 + *c + **cc + *x + **xc);
    println!("모두 더한값 {}", n + m + *mc + **m1 + ***m2 + c + *cc + x + *xc);

    let a1 = n + *m + **mc + ***m1 + ****m2 + *c + **cc + *x + **xc;//이걸 기준 삼아야 것다 햇갈릴 수 있으니
    let a2 = n + m + *mc + **m1 + ***m2 + c + *cc + x + *xc;//1번 참조정도는 그냥 무시해도 되나보다!

    println!("모두 더한값 {} {}", a1, a2);
    
    let mut zz = 10;
    zz+=1;//적용 되어진후 aaa에 적용
    zz+1;//매인 함수 매모리상 어딘가에 값이 생성되어 있다 let _ = aa+1; 이거와 같은 상태 즉 새로운 바인드가 매인 어딘가에 생성되어저 있다는 경고를 보낸다
    let aaa = &zz;
    //zz+=1; //바인드가 이미 위에 빌려저 있기때문에 오류난다 `zz` is assigned to here but it was already borrowed 가변 바인드의 참조는 1개만 사용할수있다 *mut
    //aaa+=1;//불변 바인드 이기에 값이 변경되어지진 않는다 오류난다 컴파일 불가
    aaa+1;//매인 함수 매모리상 어딘가에 값이 생성되어 있다 let _ = aaa+1; 이거와 같은 상태 즉 새로운 바인드가 매인 어딘가에 생성되어저 있다는 경고를 보낸다
    println!("{}", aaa);//빌드는 가능 하지만 aaa+1 적용 되지 않는다
    

    let a = 42;
    let r = &a;//a를 참조한다(매모리 주소값)
    let b = a+*r;//a에 r(이미 참조되어있는 r을 역참조 해서 얻음)를 더하고 이를 b에 할당한다
    println!("{}+{}={}", a, r, b);

}
//신박한 참조 reference (리패어런스) C로 치면 포인터 개념인대 다르다 완전 다르다 똑같다고 생각 했는대 아녀 완전 틀려 깊게 파고드니 다르더라...
//C의 포인터는 중간에 참조 되어진 변수가 해재되면 쓰래기 값을 포인터가 지정하는대(댕글링 포인터) 
//러스트의 리페어런스는 중간에 참조되어진 바인드가 해재되면 아예 빌드가 안됨...참 안전하더라.... 근대 개 빡세다...
//왜냐고? 참조가 많이 필요한 코드에서 바인드의 움직임 파악 못 하면 빌드 조차 안된다... rust는 한번 사용된 바인드(변수)는 매모리를 해재 시켜버림 매모리 절약해서 좋은대
//바인드(변수) 움직임을 생각 하면서 참조 쓸라 치면 그냥 뒤짐...
//쉐도윙 오너쉽 라이프타임 보로윙이라는 개념을 이해 하면 변수가 이동한다 라고 생각 하게 될꺼임..(그야 뭐 단순 해재 생성임으로 실재로는 이동하는개 아니긴 하다만...)
//와 나도 이개 뭔가 했다가 1번 사용된 바인드(변수)은 컴파일이 알아서 매모리 해재 시켜버림...좋긴 한대 참조 역참조 쓰다보면 골때리더라...참조도 다양하고 함수는 말할것도 없고..
//불변참조 불변역참조 가변참조 가변역참조 & / * / &mut / *mut 와 C의 포인터 하고는 상상 할 수 없을 정도로 다르더라... ㅎㅎㅎㅎ 근대 익숙 해지면 상당히 안전성 있개 프로그래밍이 가능하더라
//참고로 역참조는 불변 쓰더라도 값이 변하는 경우가 있다 역참조 사용시 주의 해야 한다 아놔....
//참조 없이 함수에 인자 값으로 그냥 넘겨보면 인자로 넘겨진 바인드(변수)가 인자 속에 복사? 되고 밖에있는 바인드는 매모리가 해재 되더라

//ex) 
//let a:[u8;3] = [1, 2, 3]; 함수인자로 넘겨지는 순간 drop 된다 
//println!("a 인자가 넘겨 지기전에는 출력된다{:?}", a); 함수인자로 넘겨지기 전에는 프린트 찍힌다 아니~! println!() 이거 함수 아니냐? 응 매크로 참고로 값을 복사만 하고 기존 바인드는 드랍 안된다~!
//function(a); a라는 바인드가 함수의 인자로 넘어가면 밖의 a 라는 바인드는 drop 되면 매모리가 해재된다 오너쉽 개념 단 .clone() 사용시 배열을 복사한다
//println!("{:?}", a); borrow(이동) 되었다면서 오류난다
//데이터 타입이 아닌 단순 value인 경우 (u8 u32 u64 i8 i32 i64 f32 f68) 암묵적으로 copy()가 사용된다 즉 클론처럼 명시 안해줘도 값이 복사가 이루어서 드랍이 발생되지 않는다!
//ex)
//let a1 = 123; 불변 바이든 이기에 참조를 활용한 value 변경이 불가하다 상수랑은 좀 다르다 하던대 상수는 어느 곳에든 생성이가능하다 즉 main() 밖에도 생성이 가능하다 단 상수도 영역을 떠날수가 없다
//println!("{}", a1);
//fnuction(a1) 이경우 위에 타입이 단순 value 타입이기에 자동으로 .copy() 가 암묵적으로 적용된다
//println!("{}", a1);

//프린트는 좀 특수하개 만들어진 함수 실질적으로 복사 개념으로 값이 복사가 되어진다 타입별로 copy() clone() 적용 되던지 아니면 clone()로 모두 동일하개 하던지
//복재와 복사 계념은 같을것 같지만 차이가 있다 copy() 복사는 위에서 설면 한것 처럼 단순한 값을 복사 할때 쓰이고
//clone() 은 배열 또는 구조체(struct)를 복사 할때 쓰인다 
//러스트는 참조 좀 다루고 넘김좀 이해하면 C와 별반 다를꺼 없기는 한대 이 2가지가 핵심... 아 개빡새더라....다른사람은 모르겠고 나만 그렇다고 ㅎㅎ 머리가 나빠서 그런가보다!
//2가지 이해하는대 한잠 오래 걸렷지 특히 구조채 할때 와 짜증오지개 나더라... 일단 컴파일 하면 오류남 그리고 오류도 힌트만 줘 아놔..가뜩이나 머리도 안돌아 가는대 돌아번지는.....
//이거땜에 나온개 참조 카운터인대 쉽개 C++, 자바의 장점을 가저온거임..움직임 확인 귀찬으면 참조가 몇번 되었는지 참조 갯수를 저장하고 참조되어진개 다 사용될때까지 바인드가 유지 된다 라고 생각 하면 된다
//넘김에 익숙 해지면 참조 차운터가 더 어렵더라...... 내가 이상한건가? 2가지를 동시에 사용해서 그런가...
//내가 적은 글 이해 한사람 몇 이나 있을라나 그야물론 RUST를 주력으로 쓰는사람이면 별것도 아니라 생각 하겠지만...
//하다하다 시바 컴파일이랑 대화하고 안자있더라 ㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋ 이건되니? 안되? 그럼 이건? ㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋ