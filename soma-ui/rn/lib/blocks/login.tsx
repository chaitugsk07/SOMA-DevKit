import { useState } from 'react';
import { View } from 'react-native';
import {
  Text,
  Input,
  Button,
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
  CardFooter,
} from '@/lib/components';
import { FadeIn } from '@/lib/components/motion';

export type LoginProps = {
  title?: string;
  subtitle?: string;
  onSubmit?: (email: string, password: string) => void;
};

/** Centered login card — email + password + submit. */
export function Login({
  title = 'Sign in',
  subtitle = 'Enter your credentials to continue.',
  onSubmit,
}: LoginProps) {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  return (
    <View className="flex-1 items-center justify-center bg-background p-6">
      <FadeIn className="w-full max-w-sm">
        <Card>
          <CardHeader>
            <CardTitle>{title}</CardTitle>
            <CardDescription>{subtitle}</CardDescription>
          </CardHeader>
          <CardContent className="gap-3">
            <View className="gap-1.5">
              <Text variant="small" className="text-muted-foreground">Email</Text>
              <Input placeholder="you@example.com" autoCapitalize="none" keyboardType="email-address" value={email} onChangeText={setEmail} />
            </View>
            <View className="gap-1.5">
              <Text variant="small" className="text-muted-foreground">Password</Text>
              <Input placeholder="••••••••" secureTextEntry value={password} onChangeText={setPassword} />
            </View>
          </CardContent>
          <CardFooter>
            <Button className="w-full" label="Sign in" onPress={() => onSubmit?.(email, password)} />
          </CardFooter>
        </Card>
      </FadeIn>
    </View>
  );
}
