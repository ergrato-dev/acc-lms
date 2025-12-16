//! # HTTP Handlers
//!
//! Request handlers for assessment endpoints.

use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use validator::Validate;

use crate::api::dto::*;
use crate::domain::{NewQuiz, NewQuizQuestion, UpdateQuiz, UpdateQuizQuestion};
use crate::service::{AssessmentError, AssessmentService};

/// Application state shared across handlers.
pub struct AppState {
    pub service: AssessmentService,
}

// =============================================================================
// ERROR HANDLING
// =============================================================================

impl From<AssessmentError> for HttpResponse {
    fn from(err: AssessmentError) -> Self {
        let (status, error_type) = match &err {
            AssessmentError::QuizNotFound => (actix_web::http::StatusCode::NOT_FOUND, "quiz_not_found"),
            AssessmentError::QuestionNotFound => (actix_web::http::StatusCode::NOT_FOUND, "question_not_found"),
            AssessmentError::SubmissionNotFound => (actix_web::http::StatusCode::NOT_FOUND, "submission_not_found"),
            AssessmentError::QuizNotPublished => (actix_web::http::StatusCode::BAD_REQUEST, "quiz_not_published"),
            AssessmentError::MaxAttemptsExceeded => (actix_web::http::StatusCode::BAD_REQUEST, "max_attempts_exceeded"),
            AssessmentError::SubmissionAlreadyCompleted => (actix_web::http::StatusCode::BAD_REQUEST, "submission_completed"),
            AssessmentError::QuizHasSubmissions => (actix_web::http::StatusCode::CONFLICT, "quiz_has_submissions"),
            AssessmentError::Unauthorized => (actix_web::http::StatusCode::FORBIDDEN, "unauthorized"),
            AssessmentError::TimeLimitExceeded => (actix_web::http::StatusCode::BAD_REQUEST, "time_limit_exceeded"),
            AssessmentError::Database(_) => (actix_web::http::StatusCode::INTERNAL_SERVER_ERROR, "database_error"),
            AssessmentError::Validation(_) => (actix_web::http::StatusCode::BAD_REQUEST, "validation_error"),
        };

        HttpResponse::build(status).json(ErrorResponse::new(error_type, err.to_string()))
    }
}

fn validation_error(e: validator::ValidationErrors) -> HttpResponse {
    HttpResponse::BadRequest().json(
        ErrorResponse::new("validation_error", "Invalid request data")
            .with_details(serde_json::to_value(e).unwrap_or_default())
    )
}

// =============================================================================
// QUIZ HANDLERS
// =============================================================================

/// List quizzes for a course.
/// GET /courses/{course_id}/quizzes
pub async fn list_course_quizzes(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    query: web::Query<PaginationQuery>,
    _user_id: web::ReqData<Uuid>,
    is_instructor: web::ReqData<bool>,
) -> impl Responder {
    let course_id = path.into_inner();
    let is_instructor = is_instructor.into_inner();

    match state.service.list_course_quizzes(course_id, is_instructor, query.page, query.page_size).await {
        Ok(quizzes) => {
            let response: Vec<QuizResponseDto> = quizzes.into_iter().map(Into::into).collect();
            HttpResponse::Ok().json(PaginatedResponse::new(response, query.page, query.page_size))
        }
        Err(e) => e.into(),
    }
}

/// Get a quiz by ID.
/// GET /quizzes/{quiz_id}
pub async fn get_quiz(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let quiz_id = path.into_inner();

    match state.service.get_quiz(quiz_id).await {
        Ok(quiz) => HttpResponse::Ok().json(QuizResponseDto::from(quiz)),
        Err(e) => e.into(),
    }
}

/// Get a quiz with questions.
/// GET /quizzes/{quiz_id}/full
pub async fn get_quiz_with_questions(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    is_instructor: web::ReqData<bool>,
) -> impl Responder {
    let quiz_id = path.into_inner();
    let is_instructor = is_instructor.into_inner();

    match state.service.get_quiz_with_questions(quiz_id, is_instructor).await {
        Ok(quiz) => HttpResponse::Ok().json(QuizWithQuestionsResponse::from(quiz)),
        Err(e) => e.into(),
    }
}

/// Get quizzes for a lesson.
/// GET /lessons/{lesson_id}/quizzes
pub async fn get_lesson_quizzes(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let lesson_id = path.into_inner();

    match state.service.get_lesson_quizzes(lesson_id).await {
        Ok(quizzes) => {
            let response: Vec<QuizResponseDto> = quizzes.into_iter().map(Into::into).collect();
            HttpResponse::Ok().json(response)
        }
        Err(e) => e.into(),
    }
}

/// Create a new quiz.
/// POST /quizzes
pub async fn create_quiz(
    state: web::Data<AppState>,
    user_id: web::ReqData<Uuid>,
    body: web::Json<CreateQuizRequest>,
) -> impl Responder {
    if let Err(e) = body.validate() {
        return validation_error(e);
    }

    let user_id = user_id.into_inner();
    let data = body.into_inner();

    let new_quiz = NewQuiz {
        course_id: data.course_id,
        lesson_id: data.lesson_id,
        title: data.title,
        description: data.description,
        instructions: data.instructions,
        total_points: data.total_points,
        time_limit_minutes: data.time_limit_minutes,
        max_attempts: data.max_attempts,
        passing_score_percentage: data.passing_score_percentage,
        shuffle_questions: data.shuffle_questions,
        show_correct_answers: data.show_correct_answers,
    };

    match state.service.create_quiz(new_quiz, user_id).await {
        Ok(quiz) => HttpResponse::Created().json(QuizResponseDto::from(quiz)),
        Err(e) => e.into(),
    }
}

/// Update a quiz.
/// PATCH /quizzes/{quiz_id}
pub async fn update_quiz(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateQuizRequest>,
) -> impl Responder {
    if let Err(e) = body.validate() {
        return validation_error(e);
    }

    let quiz_id = path.into_inner();
    let data = body.into_inner();

    let update = UpdateQuiz {
        title: data.title,
        description: data.description.map(Some),
        instructions: data.instructions.map(Some),
        total_points: data.total_points,
        passing_score_percentage: data.passing_score_percentage,
        time_limit_minutes: data.time_limit_minutes.map(Some),
        max_attempts: data.max_attempts.map(Some),
        shuffle_questions: data.shuffle_questions,
        show_correct_answers: data.show_correct_answers,
        is_published: data.is_published,
    };

    match state.service.update_quiz(quiz_id, update).await {
        Ok(quiz) => HttpResponse::Ok().json(QuizResponseDto::from(quiz)),
        Err(e) => e.into(),
    }
}

/// Publish a quiz.
/// POST /quizzes/{quiz_id}/publish
pub async fn publish_quiz(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let quiz_id = path.into_inner();

    match state.service.publish_quiz(quiz_id).await {
        Ok(quiz) => HttpResponse::Ok().json(QuizResponseDto::from(quiz)),
        Err(e) => e.into(),
    }
}

/// Delete a quiz.
/// DELETE /quizzes/{quiz_id}
pub async fn delete_quiz(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let quiz_id = path.into_inner();

    match state.service.delete_quiz(quiz_id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => e.into(),
    }
}

/// Get quiz statistics.
/// GET /quizzes/{quiz_id}/stats
pub async fn get_quiz_stats(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let quiz_id = path.into_inner();

    match state.service.get_quiz_stats(quiz_id).await {
        Ok(stats) => HttpResponse::Ok().json(QuizStatsResponse::from(stats)),
        Err(e) => e.into(),
    }
}

// =============================================================================
// QUESTION HANDLERS
// =============================================================================

/// List questions for a quiz.
/// GET /quizzes/{quiz_id}/questions
pub async fn list_quiz_questions(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let quiz_id = path.into_inner();

    match state.service.get_quiz_questions(quiz_id).await {
        Ok(questions) => {
            let response: Vec<QuestionResponseDto> = questions.into_iter().map(Into::into).collect();
            HttpResponse::Ok().json(response)
        }
        Err(e) => e.into(),
    }
}

/// Create a question.
/// POST /questions
pub async fn create_question(
    state: web::Data<AppState>,
    body: web::Json<CreateQuestionRequest>,
) -> impl Responder {
    if let Err(e) = body.validate() {
        return validation_error(e);
    }

    let data = body.into_inner();

    let new_question = NewQuizQuestion {
        quiz_id: data.quiz_id,
        question_type: data.question_type,
        question_text: data.question_text,
        options: data.options,
        correct_answers: data.correct_answers,
        points: data.points,
        sort_order: data.sort_order,
        explanation: data.explanation,
        code_language: data.code_language,
    };

    match state.service.add_question(new_question).await {
        Ok(question) => HttpResponse::Created().json(QuestionResponseDto::from(question)),
        Err(e) => e.into(),
    }
}

/// Update a question.
/// PATCH /questions/{question_id}
pub async fn update_question(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateQuestionRequest>,
) -> impl Responder {
    if let Err(e) = body.validate() {
        return validation_error(e);
    }

    let question_id = path.into_inner();
    let data = body.into_inner();

    let update = UpdateQuizQuestion {
        question_type: data.question_type,
        question_text: data.question_text,
        options: data.options,
        correct_answers: data.correct_answers,
        points: data.points,
        sort_order: data.sort_order,
        explanation: data.explanation.map(Some),
        code_language: data.code_language.map(Some),
    };

    match state.service.update_question(question_id, update).await {
        Ok(question) => HttpResponse::Ok().json(QuestionResponseDto::from(question)),
        Err(e) => e.into(),
    }
}

/// Delete a question.
/// DELETE /questions/{question_id}
pub async fn delete_question(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let question_id = path.into_inner();

    match state.service.remove_question(question_id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => e.into(),
    }
}

// =============================================================================
// SUBMISSION HANDLERS
// =============================================================================

/// Start a quiz attempt.
/// POST /quizzes/{quiz_id}/start
pub async fn start_quiz(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    user_id: web::ReqData<Uuid>,
    body: web::Json<StartQuizRequest>,
) -> impl Responder {
    let quiz_id = path.into_inner();
    let user_id = user_id.into_inner();
    let data = body.into_inner();

    match state.service.start_quiz(quiz_id, user_id, data.enrollment_id).await {
        Ok(submission) => HttpResponse::Created().json(SubmissionResponseDto::from(submission)),
        Err(e) => e.into(),
    }
}

/// Get a submission.
/// GET /submissions/{submission_id}
pub async fn get_submission(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    user_id: web::ReqData<Uuid>,
    is_instructor: web::ReqData<bool>,
) -> impl Responder {
    let submission_id = path.into_inner();
    let user_id = user_id.into_inner();
    let is_instructor = is_instructor.into_inner();

    match state.service.get_submission(submission_id, user_id, is_instructor).await {
        Ok(submission) => HttpResponse::Ok().json(SubmissionResponseDto::from(submission)),
        Err(e) => e.into(),
    }
}

/// Get a submission with responses.
/// GET /submissions/{submission_id}/full
pub async fn get_submission_with_responses(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    user_id: web::ReqData<Uuid>,
    is_instructor: web::ReqData<bool>,
) -> impl Responder {
    let submission_id = path.into_inner();
    let user_id = user_id.into_inner();
    let is_instructor = is_instructor.into_inner();

    match state.service.get_submission_with_responses(submission_id, user_id, is_instructor).await {
        Ok(submission) => HttpResponse::Ok().json(SubmissionWithResponsesResponse::from(submission)),
        Err(e) => e.into(),
    }
}

/// Get user's submissions for a quiz.
/// GET /quizzes/{quiz_id}/my-submissions
pub async fn get_my_submissions(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    user_id: web::ReqData<Uuid>,
) -> impl Responder {
    let quiz_id = path.into_inner();
    let user_id = user_id.into_inner();

    match state.service.get_user_submissions(user_id, quiz_id).await {
        Ok(submissions) => {
            let response: Vec<SubmissionResponseDto> = submissions.into_iter().map(Into::into).collect();
            HttpResponse::Ok().json(response)
        }
        Err(e) => e.into(),
    }
}

/// Save an answer.
/// POST /submissions/{submission_id}/answers
pub async fn save_answer(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    user_id: web::ReqData<Uuid>,
    body: web::Json<SaveAnswerRequest>,
) -> impl Responder {
    let submission_id = path.into_inner();
    let user_id = user_id.into_inner();
    let data = body.into_inner();

    match state.service.save_answer(submission_id, data.question_id, data.answer_data, user_id).await {
        Ok(response) => HttpResponse::Ok().json(AnswerResponseDto::from(response)),
        Err(e) => e.into(),
    }
}

/// Submit a quiz.
/// POST /submissions/{submission_id}/submit
pub async fn submit_quiz(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    user_id: web::ReqData<Uuid>,
    body: web::Json<SubmitQuizRequest>,
) -> impl Responder {
    let submission_id = path.into_inner();
    let user_id = user_id.into_inner();
    let data = body.into_inner();

    match state.service.submit_quiz(submission_id, user_id, data.time_spent_seconds).await {
        Ok(submission) => HttpResponse::Ok().json(SubmissionResponseDto::from(submission)),
        Err(e) => e.into(),
    }
}

/// Grade a submission (instructor).
/// POST /submissions/{submission_id}/grade
pub async fn grade_submission(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<GradeSubmissionRequest>,
) -> impl Responder {
    if let Err(e) = body.validate() {
        return validation_error(e);
    }

    let submission_id = path.into_inner();
    let data = body.into_inner();

    // TODO: Get passing percentage from quiz
    match state.service.grade_submission(submission_id, data.score, data.feedback, 70.0).await {
        Ok(submission) => HttpResponse::Ok().json(SubmissionResponseDto::from(submission)),
        Err(e) => e.into(),
    }
}

/// Grade a response (instructor).
/// POST /responses/{response_id}/grade
pub async fn grade_response(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<GradeResponseRequest>,
) -> impl Responder {
    if let Err(e) = body.validate() {
        return validation_error(e);
    }

    let response_id = path.into_inner();
    let data = body.into_inner();

    match state.service.grade_single_response(response_id, data.points_earned, data.feedback).await {
        Ok(response) => HttpResponse::Ok().json(AnswerResponseDto::from(response)),
        Err(e) => e.into(),
    }
}

// =============================================================================
// HEALTH CHECK
// =============================================================================

/// Health check endpoint.
/// GET /health
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "assessments-service"
    }))
}
