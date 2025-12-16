-- Migration: 002_assignments_and_grades.sql
-- Description: Quizzes, assignments and grading system
-- Author: System
-- Date: 2025-08-08
-- Updated: 2025-12-15 (Schema separation)
--
-- PREREQUISITE: Run 000_schema_setup.sql and 001_initial_schema.sql first
--
-- This migration creates tables in the assessments schema:
-- - assessments.quizzes : Quiz definitions
-- - assessments.quiz_questions : Questions for quizzes
-- - assessments.quiz_submissions : Student quiz attempts
-- - assessments.quiz_responses : Individual question responses

-- ========================================
-- ASSESSMENTS SCHEMA: Quizzes & Grades
-- ========================================

-- Quizzes and assessments
CREATE TABLE assessments.quizzes (
    quiz_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    course_id UUID NOT NULL, -- References courses.courses(course_id)
    lesson_id UUID, -- References courses.lessons(lesson_id)
    title TEXT NOT NULL,
    description TEXT,
    instructions TEXT,
    total_points INTEGER NOT NULL DEFAULT 100,
    passing_score_percentage DECIMAL(5,2) NOT NULL DEFAULT 70.00,
    time_limit_minutes INTEGER,
    max_attempts INTEGER DEFAULT 1,
    shuffle_questions BOOLEAN NOT NULL DEFAULT FALSE,
    show_correct_answers BOOLEAN NOT NULL DEFAULT TRUE,
    is_published BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_assessments_quizzes_course_id ON assessments.quizzes(course_id);
CREATE INDEX idx_assessments_quizzes_lesson_id ON assessments.quizzes(lesson_id);

-- Quiz questions
CREATE TABLE assessments.quiz_questions (
    question_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    quiz_id UUID NOT NULL REFERENCES assessments.quizzes(quiz_id) ON DELETE CASCADE,
    question_text TEXT NOT NULL,
    question_type TEXT NOT NULL CHECK (question_type IN ('single_choice', 'multiple_choice', 'true_false', 'short_answer', 'essay', 'code')),
    points INTEGER NOT NULL DEFAULT 5,
    sort_order INTEGER NOT NULL,
    explanation TEXT,
    options JSONB DEFAULT '[]'::jsonb,
    correct_answers JSONB DEFAULT '[]'::jsonb,
    code_language TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_assessments_questions_quiz_id ON assessments.quiz_questions(quiz_id);

-- Student quiz submissions
CREATE TABLE assessments.quiz_submissions (
    submission_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    quiz_id UUID NOT NULL REFERENCES assessments.quizzes(quiz_id),
    user_id UUID NOT NULL, -- References auth.users(user_id)
    enrollment_id UUID NOT NULL, -- References enrollments.enrollments(enrollment_id)
    attempt_number INTEGER NOT NULL DEFAULT 1,
    status TEXT NOT NULL CHECK (status IN ('in_progress', 'submitted', 'graded')),
    score DECIMAL(5,2) DEFAULT 0.00,
    max_score DECIMAL(5,2) NOT NULL,
    passed BOOLEAN,
    time_spent_seconds INTEGER DEFAULT 0,
    started_at TIMESTAMP NOT NULL DEFAULT NOW(),
    submitted_at TIMESTAMP,
    graded_at TIMESTAMP,
    instructor_feedback TEXT
);

CREATE INDEX idx_assessments_submissions_quiz_id ON assessments.quiz_submissions(quiz_id);
CREATE INDEX idx_assessments_submissions_user_id ON assessments.quiz_submissions(user_id);
CREATE INDEX idx_assessments_submissions_enrollment_id ON assessments.quiz_submissions(enrollment_id);

-- Individual question responses
CREATE TABLE assessments.quiz_responses (
    response_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    submission_id UUID NOT NULL REFERENCES assessments.quiz_submissions(submission_id) ON DELETE CASCADE,
    question_id UUID NOT NULL REFERENCES assessments.quiz_questions(question_id),
    answer_data JSONB NOT NULL,
    is_correct BOOLEAN,
    points_earned DECIMAL(5,2) NOT NULL DEFAULT 0.00,
    instructor_feedback TEXT,
    auto_graded BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_assessments_responses_submission_id ON assessments.quiz_responses(submission_id);

-- Triggers
CREATE TRIGGER assessments_quizzes_updated_at
    BEFORE UPDATE ON assessments.quizzes
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
