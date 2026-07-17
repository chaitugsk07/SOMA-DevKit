import { useState } from 'react';
import { View } from 'react-native';
import { ComponentPage, Demo } from '../component-page';
import { Dialog, Sheet, Tooltip, Button, Text, useToast } from '@/lib/components';

export function DialogScreen() {
  const [open, setOpen] = useState(false);
  return (
    <ComponentPage title="Dialog" description="Modal overlay. Dismiss via Cancel, backdrop tap, or system back.">
      <Demo label="Trigger">
        <Button label="Open dialog" onPress={() => setOpen(true)} />
      </Demo>
      <Dialog
        visible={open}
        onClose={() => setOpen(false)}
        title="Delete project?"
        description="This action cannot be undone."
      >
        <View className="flex-row justify-end gap-2">
          <Button variant="outline" label="Cancel" onPress={() => setOpen(false)} />
          <Button variant="destructive" label="Delete" onPress={() => setOpen(false)} />
        </View>
      </Dialog>
    </ComponentPage>
  );
}

export function ToastScreen() {
  const { toast } = useToast();
  return (
    <ComponentPage title="Toast" description="Transient notifications that animate up from the bottom.">
      <Demo label="Variants">
        <Button label="Default" onPress={() => toast({ title: 'Saved', description: 'Your changes were saved.' })} />
        <Button
          variant="secondary"
          label="Success"
          onPress={() => toast({ title: 'Deployed', description: 'Build is live.', variant: 'success' })}
        />
        <Button
          variant="destructive"
          label="Error"
          onPress={() => toast({ title: 'Failed', description: 'Something went wrong.', variant: 'destructive' })}
        />
        <Button
          variant="outline"
          label="Warning"
          onPress={() => toast({ title: 'Action needed', description: 'Review the pending changes.', variant: 'warning' })}
        />
        <Button
          variant="ghost"
          label="Info"
          onPress={() => toast({ title: 'Update available', description: 'A new version is ready.', variant: 'info' })}
        />
      </Demo>
    </ComponentPage>
  );
}

export function TooltipScreen() {
  return (
    <ComponentPage title="Tooltip" description="Tap-to-toggle bubble above the trigger (touch has no hover).">
      <Demo label="Default">
        <Tooltip label="Copied to clipboard">
          <View className="rounded-md border border-border bg-card px-4 py-2">
            <Text variant="p">Tap me</Text>
          </View>
        </Tooltip>
      </Demo>
    </ComponentPage>
  );
}

export function SheetScreen() {
  const [open, setOpen] = useState(false);
  return (
    <ComponentPage title="Sheet" description="Bottom sheet — drag down or tap the backdrop to dismiss.">
      <Demo label="Trigger">
        <Button label="Open sheet" onPress={() => setOpen(true)} />
      </Demo>
      <Sheet visible={open} onClose={() => setOpen(false)} title="Share">
        <Text variant="muted" className="text-sm">
          Drag this sheet down to dismiss it, or tap outside.
        </Text>
        <View className="mt-4">
          <Button label="Done" onPress={() => setOpen(false)} />
        </View>
      </Sheet>
    </ComponentPage>
  );
}
