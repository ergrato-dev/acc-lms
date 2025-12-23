import type { UpdateProfileRequest, User, UserProfile } from '@/types';
import api from './api';

const USER_ENDPOINTS = {
  PROFILE: '/users/me',
  UPDATE_PROFILE: '/users/me',
  CHANGE_PASSWORD: '/users/me/password',
  UPLOAD_AVATAR: '/users/me/avatar',
  DELETE_ACCOUNT: '/users/me',
  PREFERENCES: '/users/me/preferences',
  NOTIFICATIONS: '/users/me/notifications',
} as const;

export interface UserPreferences {
  language: string;
  theme: 'light' | 'dark' | 'system';
  emailNotifications: boolean;
  pushNotifications: boolean;
  marketingEmails: boolean;
}

export interface NotificationSettings {
  courseUpdates: boolean;
  newLessons: boolean;
  promotions: boolean;
  reminders: boolean;
  achievements: boolean;
}

/**
 * Get user profile
 */
export async function getProfile(): Promise<UserProfile> {
  const response = await api.get<{ data: UserProfile }>(USER_ENDPOINTS.PROFILE);
  return response.data.data;
}

/**
 * Update user profile
 */
export async function updateProfile(data: UpdateProfileRequest): Promise<User> {
  const response = await api.patch<{ data: User }>(
    USER_ENDPOINTS.UPDATE_PROFILE,
    data
  );
  return response.data.data;
}

/**
 * Change user password
 */
export async function changePassword(
  currentPassword: string,
  newPassword: string
): Promise<void> {
  await api.post(USER_ENDPOINTS.CHANGE_PASSWORD, {
    currentPassword,
    newPassword,
  });
}

/**
 * Upload user avatar
 */
export async function uploadAvatar(file: File): Promise<string> {
  const formData = new FormData();
  formData.append('avatar', file);

  const response = await api.post<{ data: { url: string } }>(
    USER_ENDPOINTS.UPLOAD_AVATAR,
    formData,
    {
      headers: {
        'Content-Type': 'multipart/form-data',
      },
    }
  );
  return response.data.data.url;
}

/**
 * Delete user account
 */
export async function deleteAccount(password: string): Promise<void> {
  await api.delete(USER_ENDPOINTS.DELETE_ACCOUNT, {
    data: { password },
  });
}

/**
 * Get user preferences
 */
export async function getPreferences(): Promise<UserPreferences> {
  const response = await api.get<{ data: UserPreferences }>(
    USER_ENDPOINTS.PREFERENCES
  );
  return response.data.data;
}

/**
 * Update user preferences
 */
export async function updatePreferences(
  preferences: Partial<UserPreferences>
): Promise<UserPreferences> {
  const response = await api.patch<{ data: UserPreferences }>(
    USER_ENDPOINTS.PREFERENCES,
    preferences
  );
  return response.data.data;
}

/**
 * Get notification settings
 */
export async function getNotificationSettings(): Promise<NotificationSettings> {
  const response = await api.get<{ data: NotificationSettings }>(
    USER_ENDPOINTS.NOTIFICATIONS
  );
  return response.data.data;
}

/**
 * Update notification settings
 */
export async function updateNotificationSettings(
  settings: Partial<NotificationSettings>
): Promise<NotificationSettings> {
  const response = await api.patch<{ data: NotificationSettings }>(
    USER_ENDPOINTS.NOTIFICATIONS,
    settings
  );
  return response.data.data;
}

export const userService = {
  getProfile,
  updateProfile,
  changePassword,
  uploadAvatar,
  deleteAccount,
  getPreferences,
  updatePreferences,
  getNotificationSettings,
  updateNotificationSettings,
};

export default userService;
