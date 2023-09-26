#![allow(unused_mut)]
#![allow(unused_must_use)]
fn main() {

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

    let (qwe, asd, zxc) = (-10i32, 10u8, 1000u64);
    let (lll1, lll2, lll3) = (&qwe, &asd, &zxc);
    println!("{} {} {}", qwe, asd, zxc);
    println!("{:p}\n{:p}\n{:p}", lll1, lll2, lll3);

    let [qwe1, qwe2, qwe3] = [15i32;3];
    let [lll0, lll9, lll8] = [&qwe1, &qwe2, &qwe3];
    println!("{} {} {}", qwe1, qwe2, qwe3);
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
//쉐도윙 오너쉽 라이프타임 이라는 개념을 이해 하면 변수가 이동한다 라고 생각 하게 될꺼임..
//와 나도 이개 뭔가 했다가 1번 사용된 바인드(변수)은 컴파일이 알아서 매모리 해재 시켜버림...좋긴 한대 참조 역참조 쓰다보면 골때리더라...참조도 다양하고 함수는 말할것도 없고..
//불변참조 불변역참조 가변참조 가변역참조 & / * / &mut / *mut 와 C의 포인터 하고는 상상 할 수 없을 정도로 다르더라... ㅎㅎㅎㅎ 근대 익숙 해지면 상당히 안전성 있개 프로그래밍이 가능하더라
//참고로 역참조는 불변 쓰더라도 값이 변하는 경우가 있다 역참조 사용시 주의 해야 한다 아놔....
//참조 없이 함수에 인자 값으로 그냥 넘겨보면 인자로 넘겨진 바인드(변수)가 인자 속에 복사? 되고 밖에있는 바인드는 매모리가 해재 되더라

//ex) 
//let a = 123; 함수인자로 넘겨지는 순간 drop 된다 
//println!("a 인자가 넘겨 지기전에는 출력된다{}", a); 함수인자로 넘겨지기 전에는 프린트 찍힌다 아니~! println!() 이거 함수 아니냐? 응 매크로 참고로 값을 복사만 하고 기존 바인드는 드랍 안된다~!
//function(a); a라는 바인드가 함수의 인자로 넘어가면 밖의 a 라는 바인드는 drop 되면 매모리가 해재된다 오너쉽 개념
//println!("{}", a); borrow(이동) 되었다면서 오류난다

//프린트는 좀 특수하개 만들어진 함수 실질적으로 복사 개념으로 값이 복사가 되어진다 이미 위에서 해재되었는대 복사가 안되지.. ㅎㅎ
//복사도 여러가지가 있다.. 딥한 복사 소프트한 복사 하.. 이건 data 다룰때 알개 된것인대 설명하면 복잡하다.....
//러스트는 참조 좀 다루고 넘김좀 이해하면 C와 별반 다를꺼 없기는 한대 이 2가지가 핵심... 아 개빡새더라....다른사람은 모르겠고 나만 그렇다고 ㅎㅎ 머리가 나빠서 그런가보다!
//2가지 이해하는대 한잠 오래 걸렷지 특히 구조채 할때 와 짜증오지개 나더라... 일단 컴파일 하면 오류남 그리고 오류도 힌트만 줘 아놔..가뜩이나 머리도 안돌아 가는대 돌아번지는.....
//이거땜에 나온개 참조 카운터인대 쉽개 C++, 자바의 장점을 가저온거임..움직임 확인 귀찬으면 참조가 몇번 되었는지 참조 갯수를 저장하고 참조되어진개 다 사용될때까지 바인드가 유지 된다 라고 생각 하면 된다
//넘김에 익숙 해지면 참조 차운터가 더 어렵더라...... 내가 이상한건가? 2가지를 동시에 사용해서 그런가...
//내가 적은 글 이해 한사람 몇 이나 있을라나 그야물론 RUST를 주력으로 쓰는사람이면 별것도 아니라 생각 하겠지만...
//하다하다 시바 컴파일이랑 대화하고 안자있더라 ㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋ 이건되니? 안되? 그럼 이건? ㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋ