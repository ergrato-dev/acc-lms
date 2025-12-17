//! # Grade Service
//!
//! Business logic for grade aggregation and summaries.

use std::sync::Arc;
use redis::aio::ConnectionManager;
use uuid::Uuid;

use crate::domain::{
    CourseGrade, GradeEntry, GradeError, GradeFilter, Pagination,
    StudentGradeSummary,
};
use crate::repository::GradeRepository;

/// Service for grade operations.
pub struct GradeService {
    repo: Arc<GradeRepository>,
    redis: ConnectionManager,
}

impl GradeService {
    /// Create a new grade service.
    pub fn new(repo: Arc<GradeRepository>, redis: ConnectionManager) -> Self {
        Self { repo, redis }
    }

    /// Get complete grade summary for a student.
    pub async fn get_student_summary(
        &self,
        user_id: Uuid,
        filter: GradeFilter,
    ) -> Result<StudentGradeSummary, GradeError> {
        // Get all courses the user is enrolled in
        let courses = self.repo.get_user_course_info(user_id).await?;

        // Get grades for each course
        let mut course_grades = Vec::new();
        for course in courses {
            // Check if filter includes this course
            if let Some(filter_course_id) = filter.course_id {
                if filter_course_id != course.course_id {
                    continue;
                }
            }

            let entries = self.repo.get_course_grades_for_user(user_id, course.course_id).await?;

            if !entries.is_empty() {
                let course_grade = CourseGrade::from_entries(
                    course.course_id,
                    course.course_title,
                    user_id,
                    course.enrollment_id,
                    entries,
                );
                course_grades.push(course_grade);
            }
        }

        Ok(StudentGradeSummary::from_course_grades(user_id, course_grades))
    }

    /// Get grades for a specific course.
    pub async fn get_course_grades(
        &self,
        user_id: Uuid,
        course_id: Uuid,
    ) -> Result<CourseGrade, GradeError> {
        // Get course info
        let courses = self.repo.get_user_course_info(user_id).await?;
        let course = courses.into_iter()
            .find(|c| c.course_id == course_id)
            .ok_or_else(|| GradeError::NotFound(
                "Course enrollment not found".to_string()
            ))?;

        // Get all grades for the course
        let entries = self.repo.get_course_grades_for_user(user_id, course_id).await?;

        Ok(CourseGrade::from_entries(
            course.course_id,
            course.course_title,
            user_id,
            course.enrollment_id,
            entries,
        ))
    }

    /// Get a specific grade entry.
    pub async fn get_grade_entry(
        &self,
        submission_id: Uuid,
        user_id: Option<Uuid>,
    ) -> Result<GradeEntry, GradeError> {
        self.repo.get_grade_entry(submission_id, user_id).await
    }

    /// Get paginated list of grades for a user.
    pub async fn get_user_grades_paginated(
        &self,
        user_id: Uuid,
        filter: &GradeFilter,
        pagination: &Pagination,
    ) -> Result<Vec<GradeEntry>, GradeError> {
        self.repo.get_user_grades(user_id, filter, pagination).await
    }
}
