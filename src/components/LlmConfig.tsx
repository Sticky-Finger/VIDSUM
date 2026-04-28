import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';

/// LLM 配置数据
interface LlmConfigData {
  base_url: string;
  api_key: string;
  model: string;
}

/// 组件 Props
interface LlmConfigProps {
  /// 配置完成后的回调
  onConfigured: (config: LlmConfigData) => void;
  /// 返回上一步的回调
  onBack: () => void;
}

export function LlmConfig({ onConfigured, onBack }: LlmConfigProps) {
  const [baseUrl, setBaseUrl] = useState('https://api.openai.com/v1');
  const [apiKey, setApiKey] = useState('');
  const [model, setModel] = useState('gpt-4o-mini');
  const [loading, setLoading] = useState(false);
  const [testing, setTesting] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [testResult, setTestResult] = useState<string | null>(null);

  /// 加载已保存的配置
  useEffect(() => {
    loadSavedConfig();
  }, []);

  const loadSavedConfig = async () => {
    try {
      const config = await invoke<LlmConfigData>('load_llm_config');
      setBaseUrl(config.base_url);
      setApiKey(config.api_key);
      setModel(config.model);
    } catch (e) {
      console.error('加载 LLM 配置失败:', e);
    }
  };

  /// 保存配置
  const handleSave = async () => {
    setError(null);
    setLoading(true);

    try {
      const config: LlmConfigData = { base_url: baseUrl, api_key: apiKey, model };
      await invoke('save_llm_config', { config });
      onConfigured(config);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  };

  /// 测试连接
  const handleTest = async () => {
    setError(null);
    setTestResult(null);
    setTesting(true);

    try {
      const config: LlmConfigData = { base_url: baseUrl, api_key: apiKey, model };
      const result = await invoke<string>('test_llm_connection', { config });
      setTestResult(result);
    } catch (e) {
      setError(String(e));
    } finally {
      setTesting(false);
    }
  };

  return (
    <Card className="w-[480px]">
      <CardHeader>
        <CardTitle>大模型 API 配置</CardTitle>
        <CardDescription>
          配置 OpenAI 兼容的 API 接口，用于生成字幕总结
        </CardDescription>
      </CardHeader>

      <CardContent className="flex flex-col gap-4">
        {/* Base URL */}
        <div className="flex flex-col gap-2">
          <Label htmlFor="base-url">Base URL</Label>
          <Input
            id="base-url"
            placeholder="https://api.openai.com/v1"
            value={baseUrl}
            onChange={(e) => setBaseUrl(e.target.value)}
          />
          <p className="text-xs text-gray-400">
            OpenAI 兼容 API 的基础地址，如 https://api.openai.com/v1
          </p>
        </div>

        {/* API Key */}
        <div className="flex flex-col gap-2">
          <Label htmlFor="api-key">API Key</Label>
          <Input
            id="api-key"
            type="password"
            placeholder="sk-..."
            value={apiKey}
            onChange={(e) => setApiKey(e.target.value)}
          />
          <p className="text-xs text-gray-400">
            API 密钥，配置后将保存在本地
          </p>
        </div>

        {/* Model ID */}
        <div className="flex flex-col gap-2">
          <Label htmlFor="model-id">Model ID</Label>
          <Input
            id="model-id"
            placeholder="gpt-4o-mini"
            value={model}
            onChange={(e) => setModel(e.target.value)}
          />
          <p className="text-xs text-gray-400">
            模型名称，如 gpt-4o-mini、deepseek-chat 等
          </p>
        </div>

        {/* 错误提示 */}
        {error && (
          <div className="p-3 bg-red-50 border border-red-200 rounded-lg">
            <p className="text-red-600 text-sm">{error}</p>
          </div>
        )}

        {/* 测试结果 */}
        {testResult && (
          <div className="p-3 bg-green-50 border border-green-200 rounded-lg">
            <p className="text-green-600 text-sm font-medium">连接成功</p>
            <p className="text-green-500 text-xs mt-1">{testResult}</p>
          </div>
        )}

        {/* 按钮区域 */}
        <div className="flex flex-col gap-3">
          <Button
            variant="default"
            className="w-full h-12 text-base"
            onClick={handleSave}
            disabled={loading}
          >
            {loading ? '保存中...' : '保存配置并继续'}
          </Button>
          <Button
            variant="outline"
            className="w-full h-12 text-base"
            onClick={handleTest}
            disabled={testing}
          >
            {testing ? '测试中...' : '测试连接'}
          </Button>
          <Button
            variant="ghost"
            className="w-full h-10 text-sm"
            onClick={onBack}
          >
            ← 返回
          </Button>
        </div>
      </CardContent>
    </Card>
  );
}
