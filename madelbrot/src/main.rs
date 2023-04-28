use num::complex::Complex;

fn calculate_mandelbrot(max_iters:usize, x_min:f64, x_max:f64, y_min:f64, y_max:f64, width:usize, height:usize) -> Vec<Vec<usize>>{//기본크기를 설정한다
    
    let mut rows: Vec<_> = Vec::with_capacity(width);//가로 화면 _는 타입 추론
    for img_y in 0..height {//이미지 세로
        
        let mut row: Vec<usize> = Vec::with_capacity(height);//세로
        for img_x in 0..width {//이미지 가로

            let x_percent = img_x as f64 / width as f64;//이미지 가로 나누기 화면크기 가로
            let y_percent = img_y as f64 / height as f64;//이미지 세로 나누기 화면크기 세로
            let cx = x_min + (x_max - x_min) * x_percent;//가로 값을 먼저 계산 후 x_percent 값을 곱함다 x의 센터 값
            let cy = y_min + (y_max - y_min) * y_percent;//세로 값을 먼저 계산 후 x_percent 값을 곱한다 y의 센터 값
            let escaped_at = mandelbrot_ad_point(cx, cy, max_iters);//가로센터값 세로센터값 최대공간

            row.push(escaped_at);//만들어진 공간을 삽입한다
        }
        rows.push(row);//세로줄에 삽입한다
    }
    rows//가로 세로 데이터를 보낸다
}

fn mandelbrot_ad_point(cx:f64, cy:f64, max_iters:usize) -> usize {//자 4각형의 mandelbrot_ad_point 공간를 만들어 보자
    let mut z = Complex { re:0.0, im:0.0};//초기화 한다
    let c = Complex::new(cx, cy);//센터 값을 지정한다 Complex 내장형 수학 함수

    for i in 0..=max_iters{//범위가 넘어가지 않도록 공간을 생성한다
        if z.norm() > 2.0 {
            return i;//2.0를 넘어가면 다시 시작해라
        }
        z=z*z+c;//공간확보
    }
    max_iters//변경된 값을 보낸다
}
fn render_mandelbrot(escape_valse:Vec<Vec<usize>>){//데이터를 체운다 (x축 y축 [y[x]] )
    for row in escape_valse{//세로줄 y
        let mut line = String::with_capacity(row.len());//가로줄 x row.len()
        for column in row {//가로줄선택
            let val = match column {//가로줄에 매치활용하여 데이터 삽입 
                0..=2 => ' ',//[]번호에 삽입
                3..=5 => '.',//[]번호에 삽입
                6..=10 => ',',//[]번호에 삽입
                11..=30 => '*',//[]번호에 삽입
                31..=100 => '+',//[]번호에 삽입
                101..=200 => 'x',//[]번호에 삽입
                201..=400 => '$',//[]번호에 삽입
                401..=700 => '#',//[]번호에 삽입
                _ => '%',//그 외의 []번호에 삽입
            };

            line.push(val);//x배열을 1배열식 삽입
        }
        println!("{}", line);//x를 차래로 출력
    }
}

fn main() {
    let mandelbrot = calculate_mandelbrot(1000, -2.0, 1.0, -1.0, 1.0, 100, 24);//기본크기 설정

    render_mandelbrot(mandelbrot);//기본크기 설정된 곳 데이터 표현
    
}
