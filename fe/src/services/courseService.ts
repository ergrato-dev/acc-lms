import type {
  Course,
  CourseCategory,
  CourseListParams,
  PaginatedResponse,
} from '@/types';
import api from './api';

const COURSES_ENDPOINTS = {
  LIST: '/courses',
  DETAIL: (id: string) => `/courses/${id}`,
  CATEGORIES: '/courses/categories',
  FEATURED: '/courses/featured',
  POPULAR: '/courses/popular',
  SEARCH: '/courses/search',
  BY_INSTRUCTOR: (instructorId: string) =>
    `/courses/instructor/${instructorId}`,
} as const;

/**
 * Get paginated list of courses
 */
export async function getCourses(
  params?: CourseListParams
): Promise<PaginatedResponse<Course>> {
  const response = await api.get<PaginatedResponse<Course>>(
    COURSES_ENDPOINTS.LIST,
    { params }
  );
  return response.data;
}

/**
 * Get course by ID
 */
export async function getCourseById(id: string): Promise<Course> {
  const response = await api.get<{ data: Course }>(
    COURSES_ENDPOINTS.DETAIL(id)
  );
  return response.data.data;
}

/**
 * Get course by slug
 */
export async function getCourseBySlug(slug: string): Promise<Course> {
  const response = await api.get<{ data: Course }>(
    `${COURSES_ENDPOINTS.LIST}/slug/${slug}`
  );
  return response.data.data;
}

/**
 * Get all course categories
 */
export async function getCategories(): Promise<CourseCategory[]> {
  const response = await api.get<{ data: CourseCategory[] }>(
    COURSES_ENDPOINTS.CATEGORIES
  );
  return response.data.data;
}

/**
 * Get featured courses
 */
export async function getFeaturedCourses(): Promise<Course[]> {
  const response = await api.get<{ data: Course[] }>(
    COURSES_ENDPOINTS.FEATURED
  );
  return response.data.data;
}

/**
 * Get popular courses
 */
export async function getPopularCourses(limit = 10): Promise<Course[]> {
  const response = await api.get<{ data: Course[] }>(
    COURSES_ENDPOINTS.POPULAR,
    { params: { limit } }
  );
  return response.data.data;
}

/**
 * Search courses
 */
export async function searchCourses(
  query: string,
  params?: Omit<CourseListParams, 'search'>
): Promise<PaginatedResponse<Course>> {
  const response = await api.get<PaginatedResponse<Course>>(
    COURSES_ENDPOINTS.SEARCH,
    { params: { q: query, ...params } }
  );
  return response.data;
}

/**
 * Get courses by instructor
 */
export async function getCoursesByInstructor(
  instructorId: string,
  params?: CourseListParams
): Promise<PaginatedResponse<Course>> {
  const response = await api.get<PaginatedResponse<Course>>(
    COURSES_ENDPOINTS.BY_INSTRUCTOR(instructorId),
    { params }
  );
  return response.data;
}

export const courseService = {
  getCourses,
  getCourseById,
  getCourseBySlug,
  getCategories,
  getFeaturedCourses,
  getPopularCourses,
  searchCourses,
  getCoursesByInstructor,
};

export default courseService;
