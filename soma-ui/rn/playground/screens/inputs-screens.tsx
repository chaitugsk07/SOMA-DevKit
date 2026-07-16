import { useState } from 'react';
import { ComponentPage, Demo } from '../component-page';
import { Button, Input, Switch, Checkbox, Textarea } from '@/lib/components';

export function ButtonScreen() {
  return (
    <ComponentPage title="Button" description="Pressable action. 6 variants × 4 sizes.">
      <Demo label="Variants">
        <Button label="Default" />
        <Button variant="secondary" label="Secondary" />
        <Button variant="destructive" label="Destructive" />
        <Button variant="outline" label="Outline" />
        <Button variant="ghost" label="Ghost" />
        <Button variant="link" label="Link" />
      </Demo>
      <Demo label="Sizes">
        <Button size="sm" label="Small" />
        <Button size="default" label="Default" />
        <Button size="lg" label="Large" />
      </Demo>
      <Demo label="Disabled">
        <Button label="Disabled" disabled />
      </Demo>
    </ComponentPage>
  );
}

export function InputScreen() {
  const [value, setValue] = useState('');
  return (
    <ComponentPage title="Input" description="Single-line text field with focus ring.">
      <Demo label="Default">
        <Input
          className="w-full"
          placeholder="you@example.com"
          value={value}
          onChangeText={setValue}
        />
      </Demo>
      <Demo label="Disabled">
        <Input className="w-full" placeholder="Disabled" editable={false} />
      </Demo>
    </ComponentPage>
  );
}

export function SwitchScreen() {
  const [on, setOn] = useState(true);
  return (
    <ComponentPage title="Switch" description="Boolean toggle.">
      <Demo label="Interactive">
        <Switch value={on} onValueChange={setOn} />
      </Demo>
      <Demo label="Disabled">
        <Switch value onValueChange={() => {}} disabled />
        <Switch value={false} onValueChange={() => {}} disabled />
      </Demo>
    </ComponentPage>
  );
}

export function CheckboxScreen() {
  const [checked, setChecked] = useState(true);
  return (
    <ComponentPage title="Checkbox" description="Boolean selection.">
      <Demo label="Interactive">
        <Checkbox checked={checked} onCheckedChange={setChecked} />
      </Demo>
      <Demo label="Disabled">
        <Checkbox checked onCheckedChange={() => {}} disabled />
        <Checkbox checked={false} onCheckedChange={() => {}} disabled />
      </Demo>
    </ComponentPage>
  );
}

export function TextareaScreen() {
  const [value, setValue] = useState('');
  return (
    <ComponentPage title="Textarea" description="Multi-line text field.">
      <Demo label="Default">
        <Textarea
          className="w-full"
          placeholder="Write something…"
          value={value}
          onChangeText={setValue}
        />
      </Demo>
    </ComponentPage>
  );
}
