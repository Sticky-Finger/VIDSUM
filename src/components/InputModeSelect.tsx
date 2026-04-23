import { Button } from '@/components/ui/button';
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';

export interface InputModeSelectProps {
  onSelectMode: (mode: 'media' | 'subtitle') => void;
}

export function InputModeSelect({ onSelectMode }: InputModeSelectProps) {
  return (
    <Card className="w-[400px]">
      <CardHeader>
        <CardTitle>选择输入方式</CardTitle>
        <CardDescription>
          请选择您想要使用的输入方式
        </CardDescription>
      </CardHeader>
      <CardContent className="flex flex-col gap-3">
        <Button
          variant="default"
          className="w-full h-12 text-base"
          onClick={() => onSelectMode('media')}
        >
          🎬 音视频文件转写
        </Button>
        <Button
          variant="outline"
          className="w-full h-12 text-base"
          onClick={() => onSelectMode('subtitle')}
        >
          📄 已有字幕文件
        </Button>
      </CardContent>
    </Card>
  );
}
