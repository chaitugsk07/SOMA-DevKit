import type { Delivery } from '@/lib/components';

export const AVAILABLE_JOBS: Delivery[] = [
  { id: 'd1', pickup: 'Slate Roasters, 4th & Main', dropoff: '128 Birch Lane', distanceKm: 3.2, payout: 8.5 },
  { id: 'd2', pickup: 'Field Market, Union Sq', dropoff: '77 Cedar Ct, Apt 4B', distanceKm: 5.8, payout: 12.0 },
  { id: 'd3', pickup: 'Mono Deli, Harbor St', dropoff: '901 Elm Ave', distanceKm: 2.1, payout: 6.75 },
  { id: 'd4', pickup: 'Trail Cafe, West End', dropoff: '15 Willow Rd', distanceKm: 7.4, payout: 15.25 },
];

export const ACTIVE_DELIVERY: Delivery = {
  id: 'active',
  pickup: 'Slate Roasters, 4th & Main',
  dropoff: '128 Birch Lane',
  distanceKm: 3.2,
  payout: 8.5,
  status: { label: 'En route to pickup', tone: 'active', live: true },
};

export const DELIVERY_STEPS = [
  { label: 'Accepted', time: '2:02 PM' },
  { label: 'Arrived at pickup', time: '2:09 PM' },
  { label: 'Picked up', time: '2:12 PM' },
  { label: 'Delivered' },
];
