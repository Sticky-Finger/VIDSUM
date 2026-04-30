import { useState, useCallback } from 'react';
import { Button } from '@/components/ui/button';
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  CardDescription,
} from '@/components/ui/card';
import {
  type SubtitleEntry,
  generateSrt,
  generateVtt,
  downloadFile,
} from '@/lib/subtitle-export';
import { cn } from '@/lib/utils';

export interface SubtitlePreviewProps {
  entries: SubtitleEntry[];
  audioName: string;
  canRetranscribe: boolean;
  onConfirm: (entries: SubtitleEntry[]) => void;
  onBack: () => void;
}

/// 格式化毫秒为 MM:SS.ms
function formatTimeShort(ms: number): string {
  const totalSeconds = Math.floor(ms / 1000);
  const minutes = Math.floor(totalSeconds / 60);
  const seconds = totalSeconds % 60;
  const millis = ms % 1000;
  return `${String(minutes).padStart(2, '0')}:${String(seconds).padStart(2, '0')}.${String(millis).padStart(3, '0').slice(0, 2)}`;
}

export function SubtitlePreview({
  entries,
  audioName,
  canRetranscribe,
  onConfirm,
  onBack,
}: SubtitlePreviewProps) {
  const [editableEntries, setEditableEntries] = useState<SubtitleEntry[]>(entries);
  const [selectedIndex, setSelectedIndex] = useState<number>(entries[0]?.index ?? 0);
  const [editingIndex, setEditingIndex] = useState<number | null>(null);
  const [editText, setEditText] = useState<string>('');

  // 当前选中的条目
  const selectedEntry = editableEntries.find((e) => e.index === selectedIndex);

  // 选择条目
  const handleSelect = useCallback((index: number) => {
    if (editingIndex !== null) {
      // 如果正在编辑，先保存（直接内联避免 stale closure）
      setEditableEntries((prev) =>
        prev.map((entry) =>
          entry.index === editingIndex
            ? { ...entry, text: editText }
            : entry
        )
      );
      setEditingIndex(null);
      setEditText('');
    }
    setSelectedIndex(index);
  }, [editingIndex, editText]);

  // 进入编辑模式
  const handleStartEdit = useCallback(() => {
    if (!selectedEntry) return;
    setEditingIndex(selectedIndex);
    setEditText(selectedEntry.text);
  }, [selectedEntry, selectedIndex]);

  // 保存编辑
  const handleSaveEdit = useCallback(() => {
    setEditableEntries((prev) =>
      prev.map((entry) =>
        entry.index === editingIndex
          ? { ...entry, text: editText }
          : entry
      )
    );
    setEditingIndex(null);
    setEditText('');
  }, [editingIndex, editText]);

  // 取消编辑
  const handleCancelEdit = useCallback(() => {
    setEditingIndex(null);
    setEditText('');
  }, []);

  // 复制全文
  const handleCopyAll = useCallback(async () => {
    const fullText = editableEntries.map((e) => e.text).join('\n');
    try {
      await navigator.clipboard.writeText(fullText);
    } catch {
      const textarea = document.createElement('textarea');
      textarea.value = fullText;
      document.body.appendChild(textarea);
      textarea.select();
      document.execCommand('copy');
      document.body.removeChild(textarea);
    }
  }, [editableEntries]);

  // 导出 SRT
  const handleExportSrt = useCallback(() => {
    const baseName = audioName.replace(/\.[^/.]+$/, '');
    const content = generateSrt(editableEntries);
    downloadFile(content, `${baseName}.srt`, 'text/srt');
  }, [editableEntries, audioName]);

  // 导出 VTT
  const handleExportVtt = useCallback(() => {
    const baseName = audioName.replace(/\.[^/.]+$/, '');
    const content = generateVtt(editableEntries);
    downloadFile(content, `${baseName}.vtt`, 'text/vtt');
  }, [editableEntries, audioName]);

  // 确认字幕
  const handleConfirm = useCallback(() => {
    onConfirm(editableEntries);
  }, [editableEntries, onConfirm]);

  return (
    <Card className="w-[700px] max-w-full">
      <CardHeader>
        <CardTitle>字幕预览与编辑</CardTitle>
        <CardDescription className="truncate">
          {audioName}
        </CardDescription>
      </CardHeader>

      <CardContent className="flex flex-col gap-4">
        {/* 工具栏 */}
        <div className="flex items-center gap-2 flex-wrap">
          <Button
            variant="outline"
            size="sm"
            onClick={handleExportSrt}
          >
            导出 SRT
          </Button>
          <Button
            variant="outline"
            size="sm"
            onClick={handleExportVtt}
          >
            导出 VTT
          </Button>
          <Button
            variant="outline"
            size="sm"
            onClick={handleCopyAll}
          >
            复制全文
          </Button>
          <div className="flex-1" />
          <span className="text-xs text-gray-400">
            {editableEntries.length} 段
          </span>
        </div>

        {/* 左右分栏 */}
        <div className="flex gap-4 h-[400px]">
          {/* 左侧：时间轴列表 */}
          <div className="w-[240px] shrink-0 overflow-y-auto border rounded-lg">
            {editableEntries.map((entry) => (
              <button
                key={entry.index}
                className={cn(
                  'w-full text-left px-3 py-2 text-sm border-b last:border-b-0',
                  'hover:bg-gray-50 transition-colors',
                  selectedIndex === entry.index && 'bg-blue-50 border-l-2 border-l-blue-500'
                )}
                onClick={() => handleSelect(entry.index)}
              >
                <span className="text-xs text-gray-400 block">
                  {formatTimeShort(entry.startTime)}
                </span>
                <span className="text-gray-700 line-clamp-2 mt-0.5">
                  {entry.text}
                </span>
              </button>
            ))}
          </div>

          {/* 右侧：段落详情 */}
          <div className="flex-1 flex flex-col border rounded-lg p-4">
            {selectedEntry && (
              <>
                {/* 时间信息 */}
                <div className="text-xs text-gray-400 mb-3">
                  {formatTimeShort(selectedEntry.startTime)} → {formatTimeShort(selectedEntry.endTime)}
                  <span className="ml-2">
                    第 {selectedEntry.index} 段
                  </span>
                </div>

                {/* 文本内容 */}
                {editingIndex === selectedIndex ? (
                  <div className="flex-1 flex flex-col gap-2">
                    <textarea
                      className="flex-1 w-full border rounded-lg p-2 text-sm resize-none
                                 focus:outline-none focus:ring-2 focus:ring-blue-500"
                      value={editText}
                      onChange={(e) => setEditText(e.target.value)}
                      autoFocus
                    />
                    <div className="flex gap-2">
                      <Button
                        size="sm"
                        variant="default"
                        onClick={handleSaveEdit}
                      >
                        保存
                      </Button>
                      <Button
                        size="sm"
                        variant="ghost"
                        onClick={handleCancelEdit}
                      >
                        取消
                      </Button>
                    </div>
                  </div>
                ) : (
                  <div
                    className="flex-1 text-sm text-gray-700 leading-relaxed cursor-pointer
                               hover:bg-gray-50 rounded p-2 -mx-2 transition-colors"
                    onClick={handleStartEdit}
                    title="点击编辑"
                  >
                    {selectedEntry.text}
                  </div>
                )}
              </>
            )}
          </div>
        </div>

        {/* 底部操作 */}
        <div className="flex gap-2">
          <Button
            variant="default"
            className="flex-1 h-10 text-sm"
            onClick={handleConfirm}
          >
            确认字幕并进入总结
          </Button>
          <Button
            variant="outline"
            className="h-10 text-sm"
            onClick={onBack}
          >
            {canRetranscribe ? '← 重新转写' : '← 返回'}
          </Button>
        </div>
      </CardContent>
    </Card>
  );
}
