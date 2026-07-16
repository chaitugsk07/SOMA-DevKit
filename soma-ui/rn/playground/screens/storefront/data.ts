import type { Product } from '@/lib/components';

// Mock catalogue — Unsplash source images keep the demo looking real.
export const PRODUCTS: Product[] = [
  { id: '1', name: 'Aero Runner', price: 129.0, was: 159.0, rating: 4.5, ratingCount: 312, badge: 'SALE',
    image: 'https://images.unsplash.com/photo-1542291026-7eec264c27ff?w=600&q=80' },
  { id: '2', name: 'Field Jacket', price: 248.0, rating: 4.8, ratingCount: 88,
    image: 'https://images.unsplash.com/photo-1551028719-00167b16eac5?w=600&q=80' },
  { id: '3', name: 'Mono Watch', price: 320.0, rating: 4.9, ratingCount: 540,
    image: 'https://images.unsplash.com/photo-1523275335684-37898b6baf30?w=600&q=80' },
  { id: '4', name: 'Canvas Tote', price: 64.0, was: 80.0, rating: 4.2, ratingCount: 156, badge: 'SALE',
    image: 'https://images.unsplash.com/photo-1544816155-12df9643f363?w=600&q=80' },
  { id: '5', name: 'Slate Headphones', price: 199.0, rating: 4.7, ratingCount: 1024,
    image: 'https://images.unsplash.com/photo-1505740420928-5e560c06d30e?w=600&q=80' },
  { id: '6', name: 'Trail Pack', price: 145.0, rating: 4.6, ratingCount: 203,
    image: 'https://images.unsplash.com/photo-1553062407-98eeb64c6a62?w=600&q=80' },
];

export const ORDER_STEPS = [
  { label: 'Order placed', time: '9:24 AM' },
  { label: 'Confirmed', time: '9:26 AM' },
  { label: 'Shipped', time: '11:40 AM' },
  { label: 'Out for delivery', time: 'Today, 2:15 PM' },
  { label: 'Delivered' },
];
