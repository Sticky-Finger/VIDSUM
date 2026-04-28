import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Button } from '@/components/ui/button';
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';

export interface FileSelectorProps {
  mode: 'media' | 'subtitle';
  onBack: () => void;
  onFileSelected: (file: SelectedFile) => void;
}

export interface SelectedFile {
  path: string;
  name: string;
  type: 'media' | 'subtitle';
  size?: number;
}

// 音视频格式
const MEDIA_EXTENSIONS = ['.mp4', '.mkv', '.mov', '.avi', '.webm', '.flv', '.mp3', '.wav', '.m4a', '.aac', '.ogg', '.flac'];

// 字幕格式
const SUBTITLE_EXTENSIONS = ['.srt', '.vtt', '.ass', '.ssa'];

export function FileSelector({ mode, onBack, onFileSelected }: FileSelectorProps) {
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const isMediaMode = mode === 'media';
  const title = isMediaMode ? '选择音视频文件' : '选择字幕文件';
  const description = isMediaMode
    ? '支持 MP4, MKV, MOV, AVI, WebM, FLV, MP3, WAV, M4A 等格式'
    : '支持 SRT, VTT, ASS, SSA 格式';

  const handleSelectFile = async () => {
    setIsLoading(true);
    setError(null);

    try {
      // 调用 Rust 命令打开文件选择对话框
      const filePath: string = await invoke('select_file', {
        fileType: isMediaMode ? 'media' : 'subtitle',
      });

      if (filePath) {
        // 提取文件名
        const name = filePath.split(/[/\\]/).pop() || '未知文件';
        // 获取文件扩展名
        const ext = name.slice(name.lastIndexOf('.')).toLowerCase();

        // 根据扩展名判断文件类型
        let fileType: 'media' | 'subtitle' = isMediaMode ? 'media' : 'subtitle';
        if (MEDIA_EXTENSIONS.includes(ext)) {
          fileType = 'media';
        } else if (SUBTITLE_EXTENSIONS.includes(ext)) {
          fileType = 'subtitle';
        }

        const selectedFile: SelectedFile = {
          path: filePath,
          name,
          type: fileType,
        };

        onFileSelected(selectedFile);
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : '选择文件失败');
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <Card className="w-[400px]">
      <CardHeader>
        <CardTitle>{title}</CardTitle>
        <CardDescription>{description}</CardDescription>
      </CardHeader>
      <CardContent className="flex flex-col gap-3">
        <Button
          variant="default"
          className="w-full h-12 text-base"
          onClick={handleSelectFile}
          disabled={isLoading}
        >
          {isLoading ? '选择中...' : `📁 打开文件选择器`}
        </Button>

        {error && (
          <div className="text-red-500 text-sm text-center">{error}</div>
        )}

        <Button
          variant="outline"
          className="w-full h-12 text-base"
          onClick={onBack}
          disabled={isLoading}
        >
          ← 返回
        </Button>
      </CardContent>
    </Card>
  );
}
