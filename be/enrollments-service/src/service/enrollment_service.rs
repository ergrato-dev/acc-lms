//! # Enrollment Service
//!
//! Business logic for course enrollments and progress tracking.
//!
//! ## Responsibilities
//!
//! - Enrollment creation and management
//! - Progress tracking and calculation
//! - Completion detection and certificate issuance
//! - Authorization enforcement

use std::sync::Arc;
use uuid::Uuid;

use crate::domain::{
    Enrollment, EnrollmentStatus, EnrollmentWithProgress,
    LessonProgress, LessonProgressStatus, NewEnrollment, NewLessonProgress,
    UpdateEnrollment, UpdateLessonProgress,
};
use crate::repository::{
    EnrollmentRepository, CourseEnrollmentStats, UserLearningStats,
};

/// Enrollment service errors.
#[derive(Debug, thiserror::Error)]
pub enum EnrollmentError {
    #[error("Enrollment not found")]
    NotFound,

    #[error("User is already enrolled in this course")]
    AlreadyEnrolled,

    #[error("Course not found or unavailable")]
    CourseNotFound,

    #[error("Not authorized to access this enrollment")]
    Unauthorized,

    #[error("Enrollment has expired")]
    Expired,

    #[error("Cannot perform action on completed enrollment")]
    AlreadyCompleted,

    #[error("Lesson not found in course")]
    LessonNotFound,

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Validation error: {0}")]
    Validation(String),
}

/// Result type for enrollment operations.
pub type EnrollmentResult<T> = Result<T, EnrollmentError>;

/// Service for enrollment business logic.
#[derive(Debug, Clone)]
pub struct EnrollmentService {
    repository: Arc<EnrollmentRepository>,
}

impl EnrollmentService {
    /// Creates a new enrollment service.
    pub fn new(repository: Arc<EnrollmentRepository>) -> Self {
        Self { repository }
    }

    // =========================================================================
    // ENROLLMENT OPERATIONS
    // =========================================================================

    /// Lists user's enrollments with pagination.
    pub async fn list_user_enrollments(
        &self,
        user_id: Uuid,
        status: Option<EnrollmentStatus>,
        page: i64,
        page_size: i64,
    ) -> EnrollmentResult<(Vec<Enrollment>, i64)> {
        let offset = (page - 1) * page_size;

        let enrollments = self.repository
            .list_by_user(user_id, status, page_size, offset)
            .await?;

        let total = self.repository
            .count_by_user(user_id, status)
            .await?;

        Ok((enrollments, total))
    }

    /// Lists course enrollments (admin/instructor view).
    pub async fn list_course_enrollments(
        &self,
        course_id: Uuid,
        page: i64,
        page_size: i64,
    ) -> EnrollmentResult<Vec<Enrollment>> {
        let offset = (page - 1) * page_size;

        self.repository
            .list_by_course(course_id, page_size, offset)
            .await
            .map_err(Into::into)
    }

    /// Gets enrollment by ID with authorization check.
    pub async fn get_enrollment(
        &self,
        enrollment_id: Uuid,
        requesting_user_id: Uuid,
        is_admin: bool,
    ) -> EnrollmentResult<Enrollment> {
        let enrollment = self.repository
            .find_by_id(enrollment_id)
            .await?
            .ok_or(EnrollmentError::NotFound)?;

        // Authorization: User can only see their own enrollments unless admin
        if enrollment.user_id != requesting_user_id && !is_admin {
            return Err(EnrollmentError::Unauthorized);
        }

        Ok(enrollment)
    }

    /// Gets enrollment with all lesson progress.
    pub async fn get_enrollment_with_progress(
        &self,
        enrollment_id: Uuid,
        requesting_user_id: Uuid,
        is_admin: bool,
    ) -> EnrollmentResult<EnrollmentWithProgress> {
        let enrollment = self.get_enrollment(enrollment_id, requesting_user_id, is_admin).await?;
        let progress = self.repository.list_progress(enrollment_id).await?;

        Ok(EnrollmentWithProgress {
            enrollment,
            lesson_progress: progress,
        })
    }

    /// Checks if user is enrolled in a course.
    pub async fn check_enrollment(
        &self,
        user_id: Uuid,
        course_id: Uuid,
    ) -> EnrollmentResult<Option<Enrollment>> {
        self.repository
            .find_by_user_and_course(user_id, course_id)
            .await
            .map_err(Into::into)
    }

    /// Enrolls a user in a course.
    pub async fn enroll_user(
        &self,
        user_id: Uuid,
        course_id: Uuid,
        source: Option<String>,
        expires_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> EnrollmentResult<Enrollment> {
        // Check if already enrolled
        if self.repository.is_enrolled(user_id, course_id).await? {
            return Err(EnrollmentError::AlreadyEnrolled);
        }

        // TODO: Validate course exists via courses-service API or event
        // For now, we trust the course_id is valid

        let data = NewEnrollment {
            user_id,
            course_id,
            enrollment_source: source,
            expires_at,
        };

        let enrollment = self.repository.create(data).await?;

        // TODO: Emit EnrollmentEvent::Created

        Ok(enrollment)
    }

    /// Updates enrollment status.
    pub async fn update_enrollment_status(
        &self,
        enrollment_id: Uuid,
        status: EnrollmentStatus,
        requesting_user_id: Uuid,
        is_admin: bool,
    ) -> EnrollmentResult<Enrollment> {
        let enrollment = self.get_enrollment(enrollment_id, requesting_user_id, is_admin).await?;

        // Only admins can change status (users can only complete via progress)
        if !is_admin {
            return Err(EnrollmentError::Unauthorized);
        }

        let update = UpdateEnrollment {
            status: Some(status),
            ..Default::default()
        };

        let updated = self.repository.update(enrollment.enrollment_id, update).await?;

        // TODO: Emit EnrollmentEvent::StatusChanged

        Ok(updated)
    }

    // =========================================================================
    // PROGRESS TRACKING
    // =========================================================================

    /// Starts a lesson (creates progress record if not exists).
    pub async fn start_lesson(
        &self,
        enrollment_id: Uuid,
        lesson_id: Uuid,
        user_id: Uuid,
    ) -> EnrollmentResult<LessonProgress> {
        // Verify enrollment belongs to user
        let enrollment = self.repository
            .find_by_id(enrollment_id)
            .await?
            .ok_or(EnrollmentError::NotFound)?;

        if enrollment.user_id != user_id {
            return Err(EnrollmentError::Unauthorized);
        }

        // Check enrollment is active
        if enrollment.status == EnrollmentStatus::Completed {
            // Allow viewing completed content but don't track new progress
        }

        if enrollment.status == EnrollmentStatus::Refunded
            || enrollment.status == EnrollmentStatus::Expired {
            return Err(EnrollmentError::Expired);
        }

        // Mark enrollment as started if first lesson
        self.repository.mark_started(enrollment_id).await?;

        // Create or get progress
        let data = NewLessonProgress {
            enrollment_id,
            lesson_id,
            user_id,
        };

        let progress = self.repository.upsert_progress(data).await?;

        // Update last accessed
        self.repository.update_last_accessed(enrollment_id).await?;

        Ok(progress)
    }

    /// Updates lesson progress (time spent, position).
    pub async fn update_lesson_progress(
        &self,
        enrollment_id: Uuid,
        lesson_id: Uuid,
        user_id: Uuid,
        time_spent_delta: Option<i32>,
        position_seconds: Option<i32>,
        completion_percentage: Option<f64>,
    ) -> EnrollmentResult<LessonProgress> {
        // Verify enrollment and get progress
        let enrollment = self.repository
            .find_by_id(enrollment_id)
            .await?
            .ok_or(EnrollmentError::NotFound)?;

        if enrollment.user_id != user_id {
            return Err(EnrollmentError::Unauthorized);
        }

        let progress = self.repository
            .find_progress(enrollment_id, lesson_id)
            .await?
            .ok_or(EnrollmentError::LessonNotFound)?;

        // Calculate new values
        let new_time = time_spent_delta
            .map(|delta| progress.time_spent_seconds + delta);

        // Update status based on completion percentage
        let new_status = completion_percentage.map(|pct| {
            if pct >= 100.0 {
                LessonProgressStatus::Completed
            } else if pct > 0.0 {
                LessonProgressStatus::InProgress
            } else {
                LessonProgressStatus::NotStarted
            }
        });

        let update = UpdateLessonProgress {
            status: new_status,
            completion_percentage,
            time_spent_seconds: new_time,
            last_position_seconds: position_seconds,
            completed_at: if completion_percentage.map(|p| p >= 100.0).unwrap_or(false) {
                Some(Some(chrono::Utc::now()))
            } else {
                None
            },
        };

        let updated_progress = self.repository
            .update_progress(progress.progress_id, update)
            .await?;

        // Update last accessed
        self.repository.update_last_accessed(enrollment_id).await?;

        // TODO: Emit ProgressEvent::Updated

        Ok(updated_progress)
    }

    /// Marks a lesson as completed.
    pub async fn complete_lesson(
        &self,
        enrollment_id: Uuid,
        lesson_id: Uuid,
        user_id: Uuid,
        total_lessons_in_course: i64,
    ) -> EnrollmentResult<(LessonProgress, bool)> {
        // Verify enrollment
        let enrollment = self.repository
            .find_by_id(enrollment_id)
            .await?
            .ok_or(EnrollmentError::NotFound)?;

        if enrollment.user_id != user_id {
            return Err(EnrollmentError::Unauthorized);
        }

        // Get or create progress
        let progress = match self.repository.find_progress(enrollment_id, lesson_id).await? {
            Some(p) => p,
            None => {
                self.start_lesson(enrollment_id, lesson_id, user_id).await?
            }
        };

        // Mark completed
        let updated = self.repository
            .mark_lesson_completed(progress.progress_id)
            .await?;

        // TODO: Emit ProgressEvent::LessonCompleted

        // Check if course is now complete
        let course_completed = self
            .check_and_complete_course(enrollment_id, total_lessons_in_course)
            .await?;

        Ok((updated, course_completed))
    }

    /// Saves playback position for a lesson.
    pub async fn save_position(
        &self,
        enrollment_id: Uuid,
        lesson_id: Uuid,
        user_id: Uuid,
        position_seconds: i32,
    ) -> EnrollmentResult<()> {
        // Quick authorization check
        let enrollment = self.repository
            .find_by_id(enrollment_id)
            .await?
            .ok_or(EnrollmentError::NotFound)?;

        if enrollment.user_id != user_id {
            return Err(EnrollmentError::Unauthorized);
        }

        // Get progress (should exist if player is running)
        if let Some(progress) = self.repository.find_progress(enrollment_id, lesson_id).await? {
            let update = UpdateLessonProgress {
                last_position_seconds: Some(position_seconds),
                ..Default::default()
            };

            self.repository.update_progress(progress.progress_id, update).await?;
        }

        Ok(())
    }

    /// Gets lesson progress for an enrollment.
    pub async fn get_lesson_progress(
        &self,
        enrollment_id: Uuid,
        user_id: Uuid,
    ) -> EnrollmentResult<Vec<LessonProgress>> {
        // Verify enrollment
        let enrollment = self.repository
            .find_by_id(enrollment_id)
            .await?
            .ok_or(EnrollmentError::NotFound)?;

        if enrollment.user_id != user_id {
            return Err(EnrollmentError::Unauthorized);
        }

        self.repository
            .list_progress(enrollment_id)
            .await
            .map_err(Into::into)
    }

    // =========================================================================
    // COMPLETION & CERTIFICATES
    // =========================================================================

    /// Checks if course is complete and marks it if so.
    async fn check_and_complete_course(
        &self,
        enrollment_id: Uuid,
        total_lessons: i64,
    ) -> EnrollmentResult<bool> {
        let completed_count = self.repository
            .count_completed_lessons(enrollment_id)
            .await?;

        if completed_count >= total_lessons && total_lessons > 0 {
            // Mark enrollment as completed
            self.repository.mark_completed(enrollment_id).await?;

            // TODO: Emit EnrollmentEvent::Completed
            // TODO: Trigger certificate generation

            Ok(true)
        } else {
            // Update progress percentage
            let progress_pct = self.repository
                .calculate_progress(enrollment_id, total_lessons)
                .await?;

            let update = UpdateEnrollment {
                progress_percentage: Some(progress_pct as f64),
                ..Default::default()
            };

            self.repository.update(enrollment_id, update).await?;

            Ok(false)
        }
    }

    /// Issues a certificate for completed enrollment.
    pub async fn issue_certificate(
        &self,
        enrollment_id: Uuid,
        is_admin: bool,
    ) -> EnrollmentResult<Enrollment> {
        if !is_admin {
            return Err(EnrollmentError::Unauthorized);
        }

        let enrollment = self.repository
            .find_by_id(enrollment_id)
            .await?
            .ok_or(EnrollmentError::NotFound)?;

        if enrollment.status != EnrollmentStatus::Completed {
            return Err(EnrollmentError::Validation(
                "Cannot issue certificate for incomplete enrollment".into()
            ));
        }

        let update = UpdateEnrollment {
            certificate_issued_at: Some(Some(chrono::Utc::now())),
            ..Default::default()
        };

        let updated = self.repository.update(enrollment_id, update).await?;

        // TODO: Emit EnrollmentEvent::CertificateIssued

        Ok(updated)
    }

    // =========================================================================
    // STATISTICS
    // =========================================================================

    /// Gets enrollment statistics for a course.
    pub async fn get_course_stats(
        &self,
        course_id: Uuid,
    ) -> EnrollmentResult<CourseEnrollmentStats> {
        self.repository
            .get_course_stats(course_id)
            .await
            .map_err(Into::into)
    }

    /// Gets learning statistics for a user.
    pub async fn get_user_stats(
        &self,
        user_id: Uuid,
    ) -> EnrollmentResult<UserLearningStats> {
        self.repository
            .get_user_stats(user_id)
            .await
            .map_err(Into::into)
    }
}
