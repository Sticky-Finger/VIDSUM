import { useState, useEffect, useRef } from 'react';
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

/// 模型信息
interface ModelInfo {
  id: string;
  label: string;
  size: string;
  description: string;
}

/// 可用模型列表
const MODELS: ModelInfo[] = [
  { id: 'tiny', label: 'Tiny', size: '~77 MB', description: '快速，适合实时处理' },
  { id: 'base', label: 'Base', size: '~147 MB', description: '平衡速度与准确度' },
  { id: 'small', label: 'Small', size: '~488 MB', description: '较高准确度' },
  { id: 'medium', label: 'Medium', size: '~1.5 GB', description: '高准确度，性能较好' },
  { id: 'large', label: 'Large', size: '~3.1 GB', description: '最高准确度，速度最慢' },
];

/// 语言选项
const LANGUAGES = [
  { code: 'auto', name: '自动检测' },
  { code: 'zh', name: '中文' },
  { code: 'en', name: 'English' },
  { code: 'ja', name: '日本語' },
];

export interface ModelSelectProps {
  onInitialized: () => void;
  onBack: () => void;
  onError: (message: string) => void;
}

export function ModelSelect({ onInitialized, onBack, onError }: ModelSelectProps) {
  const [selectedModel, setSelectedModel] = useState<string | null>(null);
  const [language, setLanguage] = useState('zh');
  const [isInitializing, setIsInitializing] = useState(false);
  const [status, setStatus] = useState<string | null>(null);
  const unlistenersRef = useRef<UnlistenFn[]>([]);

  // 注册事件监听
  useEffect(() => {
    const cleanup: UnlistenFn[] = [];

    const setupListeners = async () => {
      const unlistenInit = await listen<{ message: string }>('asr:engine-initialized', (event) => {
        setIsInitializing(false);
        setStatus(event.payload.message);
        // 初始化成功后自动进入下一步
        setTimeout(() => onInitialized(), 600);
      });

      const unlistenError = await listen<{ message: string }>('asr:error', (event) => {
        setIsInitializing(false);
        setStatus(null);
        onError(event.payload.message);
      });

      cleanup.push(unlistenInit, unlistenError);
      unlistenersRef.current = cleanup;
    };

    setupListeners();

    return () => {
      cleanup.forEach((fn) => fn());
    };
  }, [onInitialized, onError]);

  /// 选择模型并初始化引擎
  const handleSelectModel = async (modelId: string) => {
    if (isInitializing) return;

    setSelectedModel(modelId);
    setIsInitializing(true);
    setStatus(`正在加载 ${modelId.toUpperCase()} 模型，请稍候...`);

    try {
      await invoke('init_whisper_engine', {
        modelName: modelId,
        language: language === 'auto' ? null : language,
      });
    } catch (err) {
      setIsInitializing(false);
      setStatus(null);
      // Tauri invoke 失败时抛出的是字符串，需兼容处理
      const errMsg = typeof err === 'string'
        ? err
        : err instanceof Error
          ? err.message
          : '初始化失败，请重试';
      onError(errMsg);
    }
  };

  return (
    <Card className="w-[400px]">
      <CardHeader>
        <CardTitle>选择转写模型</CardTitle>
        <CardDescription>
          选择 Whisper 模型进行语音转写。模型越大准确度越高，但速度越慢。
          首次使用时会自动下载模型文件。
        </CardDescription>
      </CardHeader>

      <CardContent className="flex flex-col gap-4">
        {/* 语言选择 */}
        <div>
          <label className="text-sm text-gray-500 mb-1.5 block">
            转写语言
          </label>
          <select
            className="w-full border rounded-lg px-3 py-2 text-sm bg-white
                       focus:outline-none focus:ring-2 focus:ring-blue-500
                       disabled:opacity-50"
            value={language}
            onChange={(e) => setLanguage(e.target.value)}
            disabled={isInitializing}
          >
            {LANGUAGES.map((lang) => (
              <option key={lang.code} value={lang.code}>
                {lang.name}
              </option>
            ))}
          </select>
        </div>

        {/* 模型列表 */}
        <div className="flex flex-col gap-2">
          {MODELS.map((model) => (
            <Button
              key={model.id}
              variant={selectedModel === model.id ? 'default' : 'outline'}
              className={`w-full h-auto py-3 flex flex-col items-start gap-0.5
                ${selectedModel === model.id ? '' : 'hover:border-blue-300'}`}
              onClick={() => handleSelectModel(model.id)}
              disabled={isInitializing}
            >
              <span className="text-base font-medium">{model.label}</span>
              <span className="text-xs opacity-70">
                {model.size} · {model.description}
              </span>
            </Button>
          ))}
        </div>

        {/* 初始化状态 */}
        {status && (
          <div className="flex items-center justify-center gap-2 text-sm text-gray-500">
            <span className="inline-block w-3 h-3 border-2 border-gray-400
                           border-t-transparent rounded-full animate-spin" />
            {status}
          </div>
        )}

        <Button
          variant="ghost"
          className="w-full h-12 text-base"
          onClick={onBack}
          disabled={isInitializing}
        >
          ← 返回
        </Button>
      </CardContent>
    </Card>
  );
}
