// Curated, editorial-grade imagery for the luxe storefront demos.
export type LuxeProduct = {
  id: string;
  name: string;
  subtitle: string;
  price: number;
  image: string;
  tag?: string;
};

export const LUXE_PRODUCTS: LuxeProduct[] = [
  { id: '1', name: 'Orbit', subtitle: 'Wireless Headphones', price: 349, tag: 'NEW',
    image: 'https://images.unsplash.com/photo-1505740420928-5e560c06d30e?w=900&q=90' },
  { id: '2', name: 'Meridian', subtitle: 'Automatic Watch', price: 1290,
    image: 'https://images.unsplash.com/photo-1524592094714-0f0654e20314?w=900&q=90' },
  { id: '3', name: 'Atelier', subtitle: 'Leather Weekender', price: 620,
    image: 'https://images.unsplash.com/photo-1553062407-98eeb64c6a62?w=900&q=90' },
  { id: '4', name: 'Nimbus', subtitle: 'Performance Runner', price: 210, tag: 'SALE',
    image: 'https://images.unsplash.com/photo-1542291026-7eec264c27ff?w=900&q=90' },
  { id: '5', name: 'Ceramic', subtitle: 'Pour-Over Set', price: 96,
    image: 'https://images.unsplash.com/photo-1495474472287-4d71bcdd2085?w=900&q=90' },
];
