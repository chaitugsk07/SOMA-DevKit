import { useState } from 'react';
import { View } from 'react-native';
import { ComponentPage, Demo } from '../component-page';
import { Alert, Skeleton, Spinner, Progress, Button } from '@/lib/components';

export function AlertScreen() {
  return (
    <ComponentPage title="Alert" description="Inline status message.">
      <Demo label="Variants">
        <View className="w-full gap-3">
          <Alert title="Heads up" description="This is a default alert." />
          <Alert variant="destructive" title="Error" description="Something went wrong." />
          <Alert variant="success" title="Success" description="Saved successfully." />
          <Alert variant="warning" title="Warning" description="Review this before continuing." />
          <Alert variant="info" title="Information" description="A new version is available." />
        </View>
      </Demo>
    </ComponentPage>
  );
}

export function SkeletonScreen() {
  return (
    <ComponentPage title="Skeleton" description="Loading placeholder.">
      <Demo label="Shapes">
        <View className="w-full gap-3">
          <Skeleton className="h-10 w-10 rounded-full" />
          <Skeleton className="h-4 w-3/4" />
          <Skeleton className="h-4 w-1/2" />
        </View>
      </Demo>
    </ComponentPage>
  );
}

export function SpinnerScreen() {
  return (
    <ComponentPage title="Spinner" description="Activity indicator.">
      <Demo label="Sizes">
        <Spinner size="small" />
        <Spinner size="large" />
      </Demo>
    </ComponentPage>
  );
}

export function ProgressScreen() {
  const [value, setValue] = useState(40);
  return (
    <ComponentPage title="Progress" description="Determinate progress bar.">
      <Demo label="Interactive">
        <View className="w-full gap-3">
          <Progress value={value} />
          <View className="flex-row gap-2">
            <Button size="sm" variant="outline" label="-10" onPress={() => setValue((v) => v - 10)} />
            <Button size="sm" variant="outline" label="+10" onPress={() => setValue((v) => v + 10)} />
          </View>
        </View>
      </Demo>
    </ComponentPage>
  );
}
