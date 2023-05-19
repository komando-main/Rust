use std::io::Write;//stdout()사용 하기위해 정의
use std::{thread, time};//thread::sleep(time::Duration::from_millis(500)); 사용하기 위해 정의

fn main() {
    let mut i = 0;
    loop {
        print!("\rCount: {}", i); // \n 캐리지 리턴(Carriage Return)의 약자로, 타자기의 캐리지가 시작점으로 되돌아가는 것을 참조
        std::io::stdout().flush().unwrap();
        /*
        std::io::stdout().flush().unwrap()
        이 코드는 현재 프로세스의 전역 표준 출력 스트림에 대한 핸들을 가져와서 출력 시퀀스를 플러시한다. 즉, 버퍼에 저장된 모든 데이터가 출력 스트림으로 즉시 전송
       
        std::io::stdout()은 현재 프로세스의 전역 표준 출력 스트림에 대한 핸들을 반환
       
        flush() 메소드는 출력 시퀀스를 플러시(비우다 또는 보내다)
       
        unwrap() 메소드는 Result 타입의 값을 처리하고 성공한 경우 Ok 값을 반환하고 실패한 경우 패닉.
        */
        thread::sleep(time::Duration::from_millis(500));
        /*
        thread::sleep(time::Duration::from_millis(500));
        이 코드는 현재 스레드를 지정된 시간 동안 잠재운다
        
        time::Duration::from_millis(500)은 500 밀리초의 지속 시간을 생성
        
        thread::sleep() 함수는 이 지속 시간을 매개 변수로 사용하여 현재 스레드를 잠재운다
        
        스케줄링 세부 사항이나 플랫폼 종속적인 기능으로 인해 스레드가 지정된 시간보다 더 오래 잠들 수 있다

        이 코드는 예를 들어 다른 작업이 완료될 때까지 스레드를 일시 중지하는 등의 경우에 유용할 수 있다.
        */
        i += 1;
        if i == 10 {
            break;
        }
    }
}