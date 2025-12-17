//! # Stats Service
//!
//! Business logic for instructor statistics and analytics.

use std::sync::Arc;
use chrono::Utc;
use redis::aio::ConnectionManager;
use uuid::Uuid;

use crate::domain::{
    CourseStats, GradeDistribution, GradeError, LetterGrade,
    QuizStats,
};
use crate::repository::StatsRepository;

/// Service for statistics operations.
pub struct StatsService {
    repo: Arc<StatsRepository>,
    redis: ConnectionManager,
}

impl StatsService {
    /// Create a new stats service.
    pub fn new(repo: Arc<StatsRepository>, redis: ConnectionManager) -> Self {
        Self { repo, redis }
    }

    /// Verify instructor has access to a course.
    pub async fn verify_instructor_access(
        &self,
        user_id: Uuid,
        course_id: Uuid,
    ) -> Result<bool, GradeError> {
        self.repo.verify_instructor_access(user_id, course_id).await
    }

    /// Verify instructor has access to a quiz.
    pub async fn verify_quiz_instructor_access(
        &self,
        user_id: Uuid,
        quiz_id: Uuid,
    ) -> Result<bool, GradeError> {
        self.repo.verify_quiz_instructor_access(user_id, quiz_id).await
    }

    /// Get complete course statistics.
    pub async fn get_course_stats(&self, course_id: Uuid) -> Result<CourseStats, GradeError> {
        // Get overview stats
        let overview = self.repo.get_course_stats_overview(course_id).await?;

        // Get quiz-level stats
        let quiz_rows = self.repo.get_course_quiz_stats(course_id).await?;
        let mut quiz_stats = Vec::new();

        for quiz_row in quiz_rows {
            let quiz_id = quiz_row.quiz_id;
            let question_rows = self.repo.get_quiz_question_stats(quiz_id).await?;

            quiz_stats.push(QuizStats {
                quiz_id: quiz_row.quiz_id,
                quiz_title: quiz_row.quiz_title,
                total_submissions: quiz_row.total_submissions,
                unique_students: quiz_row.unique_students,
                average_score: quiz_row.average_score,
                highest_score: quiz_row.highest_score,
                lowest_score: quiz_row.lowest_score,
                pass_rate: quiz_row.pass_rate,
                average_time_seconds: quiz_row.average_time_seconds,
                average_attempts: quiz_row.average_attempts,
                question_stats: question_rows.into_iter().map(Into::into).collect(),
            });
        }

        // Get grade distribution
        let grade_rows = self.repo.get_grade_distribution(course_id).await?;
        let mut distribution = GradeDistribution::default();

        for row in grade_rows {
            let letter_grade = LetterGrade::from_percentage(row.grade_percentage);
            distribution.add_grade(letter_grade);
        }

        Ok(CourseStats {
            course_id: overview.course_id,
            course_title: overview.course_title,
            total_students: overview.total_students,
            students_with_submissions: overview.students_with_submissions,
            total_quizzes: overview.total_quizzes,
            average_score_percentage: overview.average_score_percentage,
            pass_rate: overview.pass_rate,
            grade_distribution: distribution,
            quiz_stats,
            calculated_at: Utc::now(),
        })
    }

    /// Get statistics for a single quiz.
    pub async fn get_quiz_stats(&self, quiz_id: Uuid) -> Result<QuizStats, GradeError> {
        let quiz_row = self.repo.get_quiz_stats(quiz_id).await?;
        let question_rows = self.repo.get_quiz_question_stats(quiz_id).await?;

        Ok(QuizStats {
            quiz_id: quiz_row.quiz_id,
            quiz_title: quiz_row.quiz_title,
            total_submissions: quiz_row.total_submissions,
            unique_students: quiz_row.unique_students,
            average_score: quiz_row.average_score,
            highest_score: quiz_row.highest_score,
            lowest_score: quiz_row.lowest_score,
            pass_rate: quiz_row.pass_rate,
            average_time_seconds: quiz_row.average_time_seconds,
            average_attempts: quiz_row.average_attempts,
            question_stats: question_rows.into_iter().map(Into::into).collect(),
        })
    }
}
