import { View } from 'react-native';
import { ComponentPage, Demo } from '../component-page';
import {
  Text,
  Badge,
  Avatar,
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
  CardFooter,
  Button,
} from '@/lib/components';

export function TextScreen() {
  return (
    <ComponentPage title="Text" description="Typed text. Headings use Rajdhani, body uses Outfit.">
      <Demo label="Headings (Rajdhani)">
        <View className="gap-1">
          <Text variant="h1">Heading 1</Text>
          <Text variant="h2">Heading 2</Text>
          <Text variant="h3">Heading 3</Text>
        </View>
      </Demo>
      <Demo label="Body (Outfit)">
        <View className="gap-1">
          <Text variant="p">Paragraph text.</Text>
          <Text variant="muted">Muted text.</Text>
          <Text variant="small">Small text.</Text>
        </View>
      </Demo>
    </ComponentPage>
  );
}

export function BadgeScreen() {
  return (
    <ComponentPage title="Badge" description="Compact status label.">
      <Demo label="Variants">
        <Badge label="Default" />
        <Badge variant="secondary" label="Secondary" />
        <Badge variant="destructive" label="Destructive" />
        <Badge variant="success" label="Success" />
        <Badge variant="outline" label="Outline" />
      </Demo>
    </ComponentPage>
  );
}

export function AvatarScreen() {
  return (
    <ComponentPage title="Avatar" description="User image with initials fallback.">
      <Demo label="Sizes (fallback)">
        <Avatar size="sm" fallback="SA" />
        <Avatar size="default" fallback="SO" />
        <Avatar size="lg" fallback="MA" />
      </Demo>
      <Demo label="Image">
        <Avatar uri="https://i.pravatar.cc/100" />
      </Demo>
    </ComponentPage>
  );
}

export function CardScreen() {
  return (
    <ComponentPage title="Card" description="Surface container with header/content/footer.">
      <Card>
        <CardHeader>
          <CardTitle>Deploy</CardTitle>
          <CardDescription>Ship the current build to production.</CardDescription>
        </CardHeader>
        <CardContent>
          <Text variant="p">Card body content goes here.</Text>
        </CardContent>
        <CardFooter>
          <Button label="Deploy" size="sm" />
        </CardFooter>
      </Card>
    </ComponentPage>
  );
}
