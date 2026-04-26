import { useState, useEffect, useRef, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { Button } from '@/components/ui/button';
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';

/// 进度事件 payload
interface ProgressPayload {
  segment_index: number;
  total_segments: number;
  text: string;
  start_time: number;
  end_time: number;
}

/// 段落 payload
interface SegmentPayload {
  text: string;
  start_time: number;
  end_time: number;
  timestamp: string;
}

/// 转写完成事件 payload
interface TranscriptionResultPayload {
  full_text: string;
  segments: SegmentPayload[];
}

/// 格式化毫秒为 HH:MM:SS
function formatTime(ms: number): string {
  const totalSeconds = Math.floor(ms / 1000);
  const hours = Math.floor(totalSeconds / 3600);
  const minutes = Math.floor((totalSeconds % 3600) / 60);
  const seconds = totalSeconds % 60;
  return `${String(hours).padStart(2, '0')}:${String(minutes).padStart(2, '0')}:${String(seconds).padStart(2, '0')}`;
}

export interface AsrProgressProps {
  audioPath: string;
  audioName: string;
  onError: (message: string) => void;
  onRestart: () => void;
}

export function AsrProgress({
  audioPath,
  audioName,
  onError,
  onRestart,
}: AsrProgressProps) {
  const [progress, setProgress] = useState<number>(0); // 0~100
  const [segments, setSegments] = useState<ProgressPayload[]>([]);
  const [status, setStatus] = useState<string>('正在准备转写...');
  const [isCompleted, setIsCompleted] = useState(false);
  const [result, setResult] = useState<TranscriptionResultPayload | null>(null);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);
  const [elapsed, setElapsed] = useState(0);
  const timerRef = useRef<ReturnType<typeof setInterval> | null>(null);
  const scrollRef = useRef<HTMLDivElement>(null);

  // 累计文本
  const accumulatedText = segments
    .map((seg) => `[${formatTime(seg.start_time)}] ${seg.text}`)
    .join('\n');

  // 自动滚动到底部
  useEffect(() => {
    if (scrollRef.current) {
      scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
    }
  }, [segments]);

  // 启动计时器
  const startTimer = useCallback(() => {
    timerRef.current = setInterval(() => {
      setElapsed((prev) => prev + 1);
    }, 1000);
  }, []);

  // 停止计时器
  const stopTimer = useCallback(() => {
    if (timerRef.current) {
      clearInterval(timerRef.current);
      timerRef.current = null;
    }
  }, []);

  // 注册事件监听并启动转写
  useEffect(() => {
    const cleanup: UnlistenFn[] = [];

    const setup = async () => {
      // 监听进度事件
      const unlistenProgress = await listen<ProgressPayload>('asr:progress', (event) => {
        const payload = event.payload;
        setProgress(
          payload.total_segments > 0
            ? Math.round(((payload.segment_index + 1) / payload.total_segments) * 100)
            : 0
        );
        setSegments((prev) => [...prev, payload]);
        setStatus(`正在转写... ${payload.segment_index + 1}/${payload.total_segments}`);
      });

      // 监听完成事件
      const unlistenComplete = await listen<TranscriptionResultPayload>(
        'asr:transcription-completed',
        (event) => {
          stopTimer();
          setIsCompleted(true);
          setResult(event.payload);
          setProgress(100);
          setStatus('转写完成');
        }
      );

      // 监听错误事件
      const unlistenError = await listen<{ message: string }>('asr:error', (event) => {
        stopTimer();
        setErrorMessage(event.payload.message);
        setStatus('转写出错');
        onError(event.payload.message);
      });

      cleanup.push(unlistenProgress, unlistenComplete, unlistenError);

      // 启动计时器并开始转写
      startTimer();

      try {
        await invoke('start_transcription', { audioPath });
        setStatus('正在转写...');
      } catch (err) {
        stopTimer();
        // Tauri invoke 失败时抛出的是字符串，需兼容处理
        const msg = typeof err === 'string'
          ? err
          : err instanceof Error
            ? err.message
            : '启动转写失败';
        setErrorMessage(msg);
        onError(msg);
      }
    };

    setup();

    return () => {
      stopTimer();
      cleanup.forEach((fn) => fn());
    };
  }, [audioPath, startTimer, stopTimer, onError]);

  // 格式化用时
  const elapsedText = formatTime(elapsed * 1000);

  // 复制到剪贴板
  const handleCopy = async () => {
    if (result) {
      try {
        await navigator.clipboard.writeText(result.full_text);
      } catch {
        // 降级处理
        const textarea = document.createElement('textarea');
        textarea.value = result.full_text;
        document.body.appendChild(textarea);
        textarea.select();
        document.execCommand('copy');
        document.body.removeChild(textarea);
      }
    }
  };

  return (
    <Card className="w-[500px]">
      <CardHeader>
        <CardTitle>
          {isCompleted ? '转写完成' : errorMessage ? '转写出错' : '语音转写'}
        </CardTitle>
        <CardDescription className="truncate">
          {audioName}
        </CardDescription>
      </CardHeader>

      <CardContent className="flex flex-col gap-4">
        {/* 进度条 */}
        {!isCompleted && !errorMessage && (
          <div>
            <div className="flex justify-between text-sm text-gray-500 mb-1">
              <span>{status}</span>
              <span>{progress}%</span>
            </div>
            <div className="w-full h-2 bg-gray-200 rounded-full overflow-hidden">
              <div
                className="h-full bg-blue-500 rounded-full transition-all duration-300"
                style={{ width: `${progress}%` }}
              />
            </div>
            <div className="text-xs text-gray-400 mt-1 text-right">
              已用时：{elapsedText}
            </div>
          </div>
        )}

        {/* 错误提示 */}
        {errorMessage && (
          <div className="p-3 bg-red-50 border border-red-200 rounded-lg">
            <p className="text-red-600 text-sm font-medium">转写出错</p>
            <p className="text-red-500 text-xs mt-1">{errorMessage}</p>
          </div>
        )}

        {/* 实时转写文本 */}
        {!isCompleted && !errorMessage && (
          <div
            ref={scrollRef}
            className="h-48 overflow-y-auto p-3 bg-gray-50 rounded-lg border text-sm
                       font-mono leading-relaxed whitespace-pre-wrap"
          >
            {accumulatedText || (
              <span className="text-gray-400">等待转写结果...</span>
            )}
          </div>
        )}

        {/* 转写完成 - 显示完整结果 */}
        {isCompleted && result && (
          <div className="flex flex-col gap-3">
            {/* 统计信息 */}
            <div className="flex gap-4 text-sm text-gray-500">
              <span>段落数：{result.segments.length}</span>
              <span>总用时：{elapsedText}</span>
            </div>

            {/* 完整文本 */}
            <div
              ref={scrollRef}
              className="max-h-64 overflow-y-auto p-3 bg-gray-50 rounded-lg border text-sm
                         font-mono leading-relaxed whitespace-pre-wrap"
            >
              {result.full_text}
            </div>

            {/* 操作按钮 */}
            <div className="flex gap-2">
              <Button
                variant="default"
                className="flex-1 h-10 text-sm"
                onClick={handleCopy}
              >
                📋 复制全文
              </Button>
              <Button
                variant="outline"
                className="flex-1 h-10 text-sm"
                onClick={onRestart}
              >
                🔄 重新开始
              </Button>
            </div>
          </div>
        )}

        {/* 转写中的取消/返回按钮 */}
        {!isCompleted && !errorMessage && (
          <Button
            variant="ghost"
            className="w-full h-10 text-sm"
            onClick={onRestart}
          >
            取消转写
          </Button>
        )}

        {errorMessage && (
          <Button
            variant="outline"
            className="w-full h-10 text-sm"
            onClick={onRestart}
          >
            ← 返回
          </Button>
        )}
      </CardContent>
    </Card>
  );
}
