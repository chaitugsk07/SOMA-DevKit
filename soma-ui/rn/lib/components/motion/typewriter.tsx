import { useEffect, useState } from 'react';
import { Text, type TextProps } from '@/lib/components/data-display/text';

export type TypewriterProps = Omit<TextProps, 'children'> & {
  text: string;
  /** ms per character. */
  speed?: number;
};

/** Types text out one character at a time. */
export function Typewriter({ text, speed = 40, ...textProps }: TypewriterProps) {
  const [count, setCount] = useState(0);

  useEffect(() => {
    setCount(0);
    const id = setInterval(() => {
      setCount((c) => {
        if (c >= text.length) {
          clearInterval(id);
          return c;
        }
        return c + 1;
      });
    }, speed);
    return () => clearInterval(id);
  }, [text, speed]);

  return <Text {...textProps}>{text.slice(0, count)}</Text>;
}
