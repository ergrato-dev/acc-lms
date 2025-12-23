// Local Storage keys
export const STORAGE_KEYS = {
  ACCESS_TOKEN: 'acc_access_token',
  REFRESH_TOKEN: 'acc_refresh_token',
  USER: 'acc_user',
  THEME: 'acc_theme',
  LOCALE: 'i18nextLng',
  CART: 'acc_cart',
  WISHLIST: 'acc_wishlist',
  VIDEO_PROGRESS: 'acc_video_progress',
} as const;

/**
 * Get an item from localStorage with JSON parsing
 */
export function getStorageItem<T>(key: string): T | null {
  try {
    const item = localStorage.getItem(key);
    return item ? JSON.parse(item) : null;
  } catch {
    return null;
  }
}

/**
 * Set an item in localStorage with JSON stringification
 */
export function setStorageItem<T>(key: string, value: T): void {
  try {
    localStorage.setItem(key, JSON.stringify(value));
  } catch (error) {
    console.error('Error saving to localStorage:', error);
  }
}

/**
 * Remove an item from localStorage
 */
export function removeStorageItem(key: string): void {
  try {
    localStorage.removeItem(key);
  } catch (error) {
    console.error('Error removing from localStorage:', error);
  }
}

/**
 * Clear all ACC LMS related items from localStorage
 */
export function clearAppStorage(): void {
  Object.values(STORAGE_KEYS).forEach((key) => {
    removeStorageItem(key);
  });
}

/**
 * Get an item from sessionStorage with JSON parsing
 */
export function getSessionItem<T>(key: string): T | null {
  try {
    const item = sessionStorage.getItem(key);
    return item ? JSON.parse(item) : null;
  } catch {
    return null;
  }
}

/**
 * Set an item in sessionStorage with JSON stringification
 */
export function setSessionItem<T>(key: string, value: T): void {
  try {
    sessionStorage.setItem(key, JSON.stringify(value));
  } catch (error) {
    console.error('Error saving to sessionStorage:', error);
  }
}

/**
 * Remove an item from sessionStorage
 */
export function removeSessionItem(key: string): void {
  try {
    sessionStorage.removeItem(key);
  } catch (error) {
    console.error('Error removing from sessionStorage:', error);
  }
}

/**
 * Storage utility object for convenience
 */
export const storage = {
  get: getStorageItem,
  set: setStorageItem,
  remove: removeStorageItem,
  clear: clearAppStorage,
  session: {
    get: getSessionItem,
    set: setSessionItem,
    remove: removeSessionItem,
  },
};
