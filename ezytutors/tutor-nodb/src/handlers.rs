use super::state::AppState;
use actix_web::{web, HttpResponse};
use super::models::Course;
use chrono::Utc;

pub async  fn health_check_handler(app_state: web::Data<AppState>) -> 
    HttpResponse {
        let health_check_response = &app_state.health_check_response;
        let mut visit_count = app_state.visit_count.lock().unwrap();
        let response = format!("{} {} times", health_check_response, visit_count);
        *visit_count += 1;
        println!("잘되냐? fn health_check_handler() ");
        HttpResponse::Ok().json(&response)
    }

pub async fn new_course(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>
) -> HttpResponse {
    let coures_count_for_user = app_state
    .courses
    .lock()
    .unwrap()
    .clone()
    .into_iter()
    .filter(|course| course.tutor_id == new_course.tutor_id)
    .count();
    let new_course = Course {
        tutor_id: new_course.tutor_id,
        course_id: Some((coures_count_for_user + 1).try_into().unwrap()),
        course_name: new_course.course_name.clone(),
        posted_time: Some(Utc::now().naive_utc()),
    };
    app_state.courses.lock().unwrap().push(new_course);
    println!("잘되냐? fn new_course() ");
    HttpResponse::Ok().json("Added course")
}
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn post_course_test() {
        let coures = web::Json(Course {
            tutor_id: 1,
            course_name:"hello this is test course".into(),
            course_id: None,
            posted_time: None,
        });

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });

        let resp = new_course(coures, app_state).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}