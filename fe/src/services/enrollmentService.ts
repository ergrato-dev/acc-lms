import type { Certificate, Enrollment, Progress } from '@/types';
import api from './api';

const ENROLLMENT_ENDPOINTS = {
  MY_ENROLLMENTS: '/enrollments/me',
  ENROLL: '/enrollments',
  DETAIL: (id: string) => `/enrollments/${id}`,
  PROGRESS: (enrollmentId: string) => `/enrollments/${enrollmentId}/progress`,
  COMPLETE_LESSON: (enrollmentId: string, lessonId: string) =>
    `/enrollments/${enrollmentId}/lessons/${lessonId}/complete`,
  CERTIFICATE: (enrollmentId: string) =>
    `/enrollments/${enrollmentId}/certificate`,
} as const;

/**
 * Get user's enrollments
 */
export async function getMyEnrollments(): Promise<Enrollment[]> {
  const response = await api.get<{ data: Enrollment[] }>(
    ENROLLMENT_ENDPOINTS.MY_ENROLLMENTS
  );
  return response.data.data;
}

/**
 * Enroll in a course
 */
export async function enrollInCourse(courseId: string): Promise<Enrollment> {
  const response = await api.post<{ data: Enrollment }>(
    ENROLLMENT_ENDPOINTS.ENROLL,
    { courseId }
  );
  return response.data.data;
}

/**
 * Get enrollment details
 */
export async function getEnrollmentById(id: string): Promise<Enrollment> {
  const response = await api.get<{ data: Enrollment }>(
    ENROLLMENT_ENDPOINTS.DETAIL(id)
  );
  return response.data.data;
}

/**
 * Get enrollment progress
 */
export async function getEnrollmentProgress(
  enrollmentId: string
): Promise<Progress> {
  const response = await api.get<{ data: Progress }>(
    ENROLLMENT_ENDPOINTS.PROGRESS(enrollmentId)
  );
  return response.data.data;
}

/**
 * Mark lesson as completed
 */
export async function completeLesson(
  enrollmentId: string,
  lessonId: string,
  watchTime?: number
): Promise<Progress> {
  const response = await api.post<{ data: Progress }>(
    ENROLLMENT_ENDPOINTS.COMPLETE_LESSON(enrollmentId, lessonId),
    { watchTime }
  );
  return response.data.data;
}

/**
 * Get certificate for completed course
 */
export async function getCertificate(
  enrollmentId: string
): Promise<Certificate> {
  const response = await api.get<{ data: Certificate }>(
    ENROLLMENT_ENDPOINTS.CERTIFICATE(enrollmentId)
  );
  return response.data.data;
}

export const enrollmentService = {
  getMyEnrollments,
  enrollInCourse,
  getEnrollmentById,
  getEnrollmentProgress,
  completeLesson,
  getCertificate,
};

export default enrollmentService;
