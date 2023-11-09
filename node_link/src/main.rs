#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(non_camel_case_types)]
use std::sync::{Arc, Mutex};
use std::any::type_name;

#[derive(Debug, Clone)]
struct Table<A, B, C> {
    q:Option<Box<A>>,
    w:Option<Box<A>>,
    e:Option<Box<B>>,
    r:Option<Box<B>>,
    t:Option<Box<C>>,
    y:Option<Box<C>>,
    up: Option<Arc<Mutex<Table<A, B, C>>>>,//이건 잘 잡혀 있긴한대...
    down: Option<Arc<Mutex<Table<A, B, C>>>>, //Arc는 'Atomic Reference Counting’의 약자로, 여러 스레드에서 안전하게 공유할 수 있는 참조 카운팅 포인터 라는대... 안되던대....clone() 써도 안됨
}

impl<A, B, C> PartialEq for Table<A, B, C>
where
A: PartialEq + std::fmt::Debug + Clone,
B: PartialEq + std::fmt::Debug + Clone,
C: PartialEq + std::fmt::Debug + Clone,
{
    fn eq(&self, other: &Self) -> bool {//이전생성과 지금 생성의 주소가 같은지 비교문
        self.q == other.q && self.w == other.w && self.e == other.e && self.r == other.r && self.t == other.t && self.y == other.y
    }
}

trait Table_fn<A, B, C>:std::fmt::Debug{
    fn new(q:A, w:A, e:B, r:B, t:C, y:C) -> Self where Self: Sized;
    fn set_table(&mut self, q:A, w:A, e:B, r:B, t:C, y:C);
    fn get_start_table(&self)->([Option<Box<A>>;2], [Option<Box<B>>;2], [Option<Box<C>>;2]);
    fn get_down_table(&self)->([Option<Box<A>>;2], [Option<Box<B>>;2], [Option<Box<C>>;2]);
    fn get_down_twostep_table(&self)->([Option<Box<A>>;2], [Option<Box<B>>;2], [Option<Box<C>>;2]);
    // fn get_up_table(&self)->([Option<Box<A>>;2], [Option<Box<B>>;2], [Option<Box<C>>;2]);
    //fn get_q(&self) -> Option<Box<A>>;
}

impl<A, B, C> Table_fn<A, B, C> for Table<A, B, C> 
where 
A:std::fmt::Debug + Clone, 
B:std::fmt::Debug + Clone, 
C:std::fmt::Debug + Clone,
Table<A, B, C>: PartialEq,
{
    fn new(q:A, w:A, e:B, r:B, t:C, y:C) -> Self where Self: Sized {
        Self {
            q:Some(Box::new(q)),
            w:Some(Box::new(w)),
            e:Some(Box::new(e)),
            r:Some(Box::new(r)),
            t:Some(Box::new(t)),
            y:Some(Box::new(y)),
            up:None,
            down:None,
        }
    }

    fn set_table(&mut self, q: A, w: A, e: B, r: B, t: C, y: C) {
        let new_table = Arc::new(Mutex::new(Self::new(q, w, e, r, t, y)));
        if let Some(down) = &self.down {
            
            new_table.lock().unwrap().up = Some(down.clone());//현재의 테이블에서 down에 데이터 있을경우 up으로 연결 시켜라 주소 복재 쓰긴했는대... 안되더라..
            
            println!("set_table() 작동은 하냐??? {:#?}", new_table);
        }
        self.down = Some(new_table.clone());//down에 새로운 데이터를 추가 하라 복재 쓰긴했는대... 안되더라.. 전부 None 로 되어 있는대.. 코드가 잘못 되었나 ㅎㅎ어디부터 봐야 하니 망햇내.........
    }

    // fn get_q(&self) -> Option<Box<A>> {
    //     self.q.clone()
    // }

    fn get_start_table(&self)->([Option<Box<A>>;2], [Option<Box<B>>;2], [Option<Box<C>>;2]){
        ([self.q.clone(), self.w.clone()], [self.e.clone(),self.r.clone()], [self.t.clone(), self.y.clone()])
    }

    // fn get_down_table(&self) -> ([Option<Box<A>>;2], [Option<Box<B>>;2], [Option<Box<C>>;2]) {
    //     if let Some(down) = &self.down {
    //         let down_table = down.lock().unwrap();
    //         return ([down_table.q.clone(), down_table.w.clone()], [down_table.e.clone(), down_table.r.clone()], [down_table.t.clone(), down_table.y.clone()]);
    //     }
    //     panic!("Down table is None");
    // }
    fn get_down_twostep_table(&self)->([Option<Box<A>>;2], [Option<Box<B>>;2], [Option<Box<C>>;2]){
        if let Some(down) = &self.down {
            let down_table = down.lock().unwrap();
            if let Some(down_down) = &down_table.down {
                let down_down_table = down_down.lock().unwrap();
                return ([down_down_table.q.clone(), down_down_table.w.clone()], [down_down_table.e.clone(), down_down_table.r.clone()], [down_down_table.t.clone(), down_down_table.y.clone()]);
            }
        }
        panic!("Down table is None");
    }
    fn get_down_table(&self)->([Option<Box<A>>;2], [Option<Box<B>>;2], [Option<Box<C>>;2]){
        if self.down.is_some() {
            let down_table = self.down.as_ref().unwrap().lock().unwrap();
            return ([down_table.q.clone(), down_table.w.clone()], [down_table.e.clone(), down_table.r.clone()], [down_table.t.clone(), down_table.y.clone()]);
        }
        panic!("Down table is None");
    }
    // fn get_down_twostep_table(&self)->([Option<Box<A>>;2], [Option<Box<B>>;2], [Option<Box<C>>;2]){
    //     let down_table;
    //     if self.down.is_some() {
    //         down_table = self.down.as_ref().unwrap().lock().unwrap();
    //         if down_table.down.is_some(){
    //             down_table = down_table.down.as_ref().unwrap().lock().unwrap();
    //             return ([down_table.q.clone(), down_table.w.clone()], [down_table.e.clone(), down_table.r.clone()], [down_table.t.clone(), down_table.y.clone()]);
    //         }
    //     }
    //     panic!("Down table is None");
    // }

}

fn type_of<T>(_: &T) -> &str {
    type_name::<T>()
}

fn main() {
    let mut start: Box<dyn Table_fn<_, _, _>> = Box::new(Table::new(123_u32, 456_u32, -123_i32, -456_i32, String::from("일단은 된다"), String::from("끝까지 한다")));
    start.set_table(789u32, 012u32, 345i32, 678i32, String::from("foo"), String::from("bar"));
    start.set_table(1, 2, 3, 4, String::from("5"), String::from("6"));
    start.set_table(0, 9, 8, 7, "6".to_owned(), "5".to_owned());
    start.set_table(12, 9, 8, 7, "6".to_owned(), "5".to_owned());
    start.set_table(23, 9, 8, 7, "6".to_owned(), "5".to_owned());
    println!("{:#?}", start);
    println!();
    println!("start.get_start_table().2[0].as_ref().unwrap()\n {:?}\n", start.get_start_table().2[0].as_ref().unwrap());
    println!("\nstart.get_start_table()\n {:?}\n", start.get_start_table());
    println!();
    println!("\nstart.get_down_table()\n {:?}\n", start.get_down_table());
    println!();
    println!("\nstart.get_down_twostep_table()\n {:?}\n", start.get_down_twostep_table());
}

/*
뭔가 좀 이상 하다........................

난 다운 설정 다 해놨는대....
 다운 이 설정이 안되있내... 전부 None 이군.........
잘못했나? 근대 빌드가 되내???
뭐 부터 봐야 하냐 망햇군...............!!!!!!!!!!!!!
희안하네.. clone() 으로 전부 했는대..
set_table() 작동은 하냐??? Mutex {
    data: Table {
        q: Some(
            1,
        ),
        w: Some(
            2,
        ),
        e: Some(
            3,
        ),
        r: Some(
            4,
        ),
        t: Some(
            "5",
        ),
        y: Some(
            "6",
        ),
        up: Some(
            Mutex {
                data: Table {
                    q: Some(
                        789,
                    ),
                    w: Some(
                        12,
                    ),
                    e: Some(
                        345,
                    ),
                    r: Some(
                        678,
                    ),
                    t: Some(
                        "foo",
                    ),
                    y: Some(
                        "bar",
                    ),
                    up: None,
                    down: None,
                },
                poisoned: false,
                ..
            },
        ),
        down: None,
    },
    poisoned: false,
    ..
}
set_table() 작동은 하냐??? Mutex {
    data: Table {
        q: Some(
            0,
        ),
        w: Some(
            9,
        ),
        e: Some(
            8,
        ),
        r: Some(
            7,
        ),
        t: Some(
            "6",
        ),
        y: Some(
            "5",
        ),
        up: Some(
            Mutex {
                data: Table {
                    q: Some(
                        1,
                    ),
                    w: Some(
                        2,
                    ),
                    e: Some(
                        3,
                    ),
                    r: Some(
                        4,
                    ),
                    t: Some(
                        "5",
                    ),
                    y: Some(
                        "6",
                    ),
                    up: Some(
                        Mutex {
                            data: Table {
                                q: Some(
                                    789,
                                ),
                                w: Some(
                                    12,
                                ),
                                e: Some(
                                    345,
                                ),
                                r: Some(
                                    678,
                                ),
                                t: Some(
                                    "foo",
                                ),
                                y: Some(
                                    "bar",
                                ),
                                up: None,
                                down: None,
                            },
                            poisoned: false,
                            ..
                        },
                    ),
                    down: None,
                },
                poisoned: false,
                ..
            },
        ),
        down: None,
    },
    poisoned: false,
    ..
}
set_table() 작동은 하냐??? Mutex {
    data: Table {
        q: Some(
            12,
        ),
        w: Some(
            9,
        ),
        e: Some(
            8,
        ),
        r: Some(
            7,
        ),
        t: Some(
            "6",
        ),
        y: Some(
            "5",
        ),
        up: Some(
            Mutex {
                data: Table {
                    q: Some(
                        0,
                    ),
                    w: Some(
                        9,
                    ),
                    e: Some(
                        8,
                    ),
                    r: Some(
                        7,
                    ),
                    t: Some(
                        "6",
                    ),
                    y: Some(
                        "5",
                    ),
                    up: Some(
                        Mutex {
                            data: Table {
                                q: Some(
                                    1,
                                ),
                                w: Some(
                                    2,
                                ),
                                e: Some(
                                    3,
                                ),
                                r: Some(
                                    4,
                                ),
                                t: Some(
                                    "5",
                                ),
                                y: Some(
                                    "6",
                                ),
                                up: Some(
                                    Mutex {
                                        data: Table {
                                            q: Some(
                                                789,
                                            ),
                                            w: Some(
                                                12,
                                            ),
                                            e: Some(
                                                345,
                                            ),
                                            r: Some(
                                                678,
                                            ),
                                            t: Some(
                                                "foo",
                                            ),
                                            y: Some(
                                                "bar",
                                            ),
                                            up: None,
                                            down: None,
                                        },
                                        poisoned: false,
                                        ..
                                    },
                                ),
                                down: None,
                            },
                            poisoned: false,
                            ..
                        },
                    ),
                    down: None,
                },
                poisoned: false,
                ..
            },
        ),
        down: None,
    },
    poisoned: false,
    ..
}
set_table() 작동은 하냐??? Mutex {
    data: Table {
        q: Some(
            23,
        ),
        w: Some(
            9,
        ),
        e: Some(
            8,
        ),
        r: Some(
            7,
        ),
        t: Some(
            "6",
        ),
        y: Some(
            "5",
        ),
        up: Some(
            Mutex {
                data: Table {
                    q: Some(
                        12,
                    ),
                    w: Some(
                        9,
                    ),
                    e: Some(
                        8,
                    ),
                    r: Some(
                        7,
                    ),
                    t: Some(
                        "6",
                    ),
                    y: Some(
                        "5",
                    ),
                    up: Some(
                        Mutex {
                            data: Table {
                                q: Some(
                                    0,
                                ),
                                w: Some(
                                    9,
                                ),
                                e: Some(
                                    8,
                                ),
                                r: Some(
                                    7,
                                ),
                                t: Some(
                                    "6",
                                ),
                                y: Some(
                                    "5",
                                ),
                                up: Some(
                                    Mutex {
                                        data: Table {
                                            q: Some(
                                                1,
                                            ),
                                            w: Some(
                                                2,
                                            ),
                                            e: Some(
                                                3,
                                            ),
                                            r: Some(
                                                4,
                                            ),
                                            t: Some(
                                                "5",
                                            ),
                                            y: Some(
                                                "6",
                                            ),
                                            up: Some(
                                                Mutex {
                                                    data: Table {
                                                        q: Some(
                                                            789,
                                                        ),
                                                        w: Some(
                                                            12,
                                                        ),
                                                        e: Some(
                                                            345,
                                                        ),
                                                        r: Some(
                                                            678,
                                                        ),
                                                        t: Some(
                                                            "foo",
                                                        ),
                                                        y: Some(
                                                            "bar",
                                                        ),
                                                        up: None,
                                                        down: None,
                                                    },
                                                    poisoned: false,
                                                    ..
                                                },
                                            ),
                                            down: None,
                                        },
                                        poisoned: false,
                                        ..
                                    },
                                ),
                                down: None,
                            },
                            poisoned: false,
                            ..
                        },
                    ),
                    down: None,
                },
                poisoned: false,
                ..
            },
        ),
        down: None,
    },
    poisoned: false,
    ..
}
Table {
    q: Some(
        123,
    ),
    w: Some(
        456,
    ),
    e: Some(
        -123,
    ),
    r: Some(
        -456,
    ),
    t: Some(
        "일단은 된다",
    ),
    y: Some(
        "끝까지 한다",
    ),
    up: None,
    down: Some(
        Mutex {
            data: Table {
                q: Some(
                    23,
                ),
                w: Some(
                    9,
                ),
                e: Some(
                    8,
                ),
                r: Some(
                    7,
                ),
                t: Some(
                    "6",
                ),
                y: Some(
                    "5",
                ),
                up: Some(
                    Mutex {
                        data: Table {
                            q: Some(
                                12,
                            ),
                            w: Some(
                                9,
                            ),
                            e: Some(
                                8,
                            ),
                            r: Some(
                                7,
                            ),
                            t: Some(
                                "6",
                            ),
                            y: Some(
                                "5",
                            ),
                            up: Some(
                                Mutex {
                                    data: Table {
                                        q: Some(
                                            0,
                                        ),
                                        w: Some(
                                            9,
                                        ),
                                        e: Some(
                                            8,
                                        ),
                                        r: Some(
                                            7,
                                        ),
                                        t: Some(
                                            "6",
                                        ),
                                        y: Some(
                                            "5",
                                        ),
                                        up: Some(
                                            Mutex {
                                                data: Table {
                                                    q: Some(
                                                        1,
                                                    ),
                                                    w: Some(
                                                        2,
                                                    ),
                                                    e: Some(
                                                        3,
                                                    ),
                                                    r: Some(
                                                        4,
                                                    ),
                                                    t: Some(
                                                        "5",
                                                    ),
                                                    y: Some(
                                                        "6",
                                                    ),
                                                    up: Some(
                                                        Mutex {
                                                            data: Table {
                                                                q: Some(
                                                                    789,
                                                                ),
                                                                w: Some(
                                                                    12,
                                                                ),
                                                                e: Some(
                                                                    345,
                                                                ),
                                                                r: Some(
                                                                    678,
                                                                ),
                                                                t: Some(
                                                                    "foo",
                                                                ),
                                                                y: Some(
                                                                    "bar",
                                                                ),
                                                                up: None, 여기두 없네????
                                                                down: None, 여기에왜 없지?????
                                                            },
                                                            poisoned: false,
                                                            ..
                                                        },
                                                    ),
                                                    down: None,
                                                },
                                                poisoned: false,
                                                ..
                                            },
                                        ),
                                        down: None,
                                    },
                                    poisoned: false,
                                    ..
                                },
                            ),
                            down: None,
                        },
thread 'main' panicked at main.rs:94:9:
Down table is None
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace  아니 왜 없다고 나오냐? 분명 clone() 썻는대...
                        poisoned: false,
                        ..
                    },
                ),
                down: None,
            },
            poisoned: false,
            ..
        },
    ),
}

start.get_start_table().2[0].as_ref().unwrap()
 "일단은 된다"


start.get_start_table()
 ([Some(123), Some(456)], [Some(-123), Some(-456)], [Some("일단은 된다"), Some("끝까지 한다")]) 뭐지?????



start.get_down_table()
 ([Some(23), Some(9)], [Some(8), Some(7)], [Some("6"), Some("5")]) 마지막이 찍히내 중간에 생성된건 다 얼로 날려 먹었냐? ㅎㅎ



[Done] exited with code=101 in 0.995 seconds
*/