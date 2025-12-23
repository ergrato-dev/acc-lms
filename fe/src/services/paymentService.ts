import type { Order, PaymentIntent, PaymentMethod } from '@/types';
import api from './api';

const PAYMENT_ENDPOINTS = {
  CREATE_ORDER: '/orders',
  GET_ORDER: (id: string) => `/orders/${id}`,
  MY_ORDERS: '/orders/me',
  PAYMENT_METHODS: '/payments/methods',
  CREATE_PAYMENT_INTENT: '/payments/intent',
  CONFIRM_PAYMENT: '/payments/confirm',
  APPLY_COUPON: '/payments/coupon',
} as const;

export interface CreateOrderRequest {
  courseId: string;
  couponCode?: string;
}

export interface PaymentIntentRequest {
  orderId: string;
  paymentMethodId?: string;
}

export interface CouponValidation {
  valid: boolean;
  discountPercent?: number;
  discountAmount?: number;
  message?: string;
}

/**
 * Create a new order
 */
export async function createOrder(data: CreateOrderRequest): Promise<Order> {
  const response = await api.post<{ data: Order }>(
    PAYMENT_ENDPOINTS.CREATE_ORDER,
    data
  );
  return response.data.data;
}

/**
 * Get order by ID
 */
export async function getOrderById(id: string): Promise<Order> {
  const response = await api.get<{ data: Order }>(
    PAYMENT_ENDPOINTS.GET_ORDER(id)
  );
  return response.data.data;
}

/**
 * Get user's orders
 */
export async function getMyOrders(): Promise<Order[]> {
  const response = await api.get<{ data: Order[] }>(
    PAYMENT_ENDPOINTS.MY_ORDERS
  );
  return response.data.data;
}

/**
 * Get saved payment methods
 */
export async function getPaymentMethods(): Promise<PaymentMethod[]> {
  const response = await api.get<{ data: PaymentMethod[] }>(
    PAYMENT_ENDPOINTS.PAYMENT_METHODS
  );
  return response.data.data;
}

/**
 * Create payment intent
 */
export async function createPaymentIntent(
  data: PaymentIntentRequest
): Promise<PaymentIntent> {
  const response = await api.post<{ data: PaymentIntent }>(
    PAYMENT_ENDPOINTS.CREATE_PAYMENT_INTENT,
    data
  );
  return response.data.data;
}

/**
 * Confirm payment
 */
export async function confirmPayment(
  paymentIntentId: string,
  paymentMethodId: string
): Promise<Order> {
  const response = await api.post<{ data: Order }>(
    PAYMENT_ENDPOINTS.CONFIRM_PAYMENT,
    { paymentIntentId, paymentMethodId }
  );
  return response.data.data;
}

/**
 * Validate coupon code
 */
export async function validateCoupon(
  couponCode: string,
  courseId: string
): Promise<CouponValidation> {
  const response = await api.post<{ data: CouponValidation }>(
    PAYMENT_ENDPOINTS.APPLY_COUPON,
    { couponCode, courseId }
  );
  return response.data.data;
}

export const paymentService = {
  createOrder,
  getOrderById,
  getMyOrders,
  getPaymentMethods,
  createPaymentIntent,
  confirmPayment,
  validateCoupon,
};

export default paymentService;
