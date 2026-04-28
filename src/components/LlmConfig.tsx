import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Textarea } from '@/components/ui/textarea';
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
  /// 配置完成后的回调（含自定义 Prompt）
  onConfigured: (config: LlmConfigData, systemPrompt: string, userPromptTemplate: string) => void;
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

  // Prompt 编辑相关状态
  const [systemPrompt, setSystemPrompt] = useState('');
  const [userPromptTemplate, setUserPromptTemplate] = useState('');
  const [showPromptEditor, setShowPromptEditor] = useState(false);
  const [defaultSystemPrompt, setDefaultSystemPrompt] = useState('');
  const [defaultUserPromptTemplate, setDefaultUserPromptTemplate] = useState('');

  /// 加载已保存的配置和默认 Prompt
  useEffect(() => {
    loadSavedConfig();
    loadDefaultPrompt();
  }, []);

  /// 加载已保存的配置
  const loadSavedConfig = async () => {
    try {
      const config = await invoke<LlmConfigData & { system_prompt?: string; user_prompt_template?: string }>('load_llm_config');
      setBaseUrl(config.base_url);
      setApiKey(config.api_key);
      setModel(config.model);
      if (config.system_prompt) setSystemPrompt(config.system_prompt);
      if (config.user_prompt_template) setUserPromptTemplate(config.user_prompt_template);
    } catch (e) {
      console.error('加载 LLM 配置失败:', e);
    }
  };

  /// 加载默认 Prompt
  const loadDefaultPrompt = async () => {
    try {
      const result = await invoke<{ system_prompt: string; user_prompt_template: string }>('get_default_prompt');
      setDefaultSystemPrompt(result.system_prompt);
      setDefaultUserPromptTemplate(result.user_prompt_template);
      // 仅当用户尚未设置过自定义 Prompt 时才使用默认值
      if (!systemPrompt) setSystemPrompt(result.system_prompt);
      if (!userPromptTemplate) setUserPromptTemplate(result.user_prompt_template);
    } catch (e) {
      console.error('获取默认 Prompt 失败:', e);
    }
  };

  /// 恢复默认 Prompt
  const handleResetPrompt = () => {
    setSystemPrompt(defaultSystemPrompt);
    setUserPromptTemplate(defaultUserPromptTemplate);
  };

  /// 保存配置
  const handleSave = async () => {
    setError(null);
    setLoading(true);

    try {
      const config: LlmConfigData = { base_url: baseUrl, api_key: apiKey, model };
      await invoke('save_llm_config', {
        config: {
          ...config,
          system_prompt: systemPrompt || null,
          user_prompt_template: userPromptTemplate || null,
        },
      });
      onConfigured(config, systemPrompt, userPromptTemplate);
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
    <Card className="w-[640px]">
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

        {/* 编辑 Prompt 折叠区域 */}
        <div className="border rounded-lg">
          <button
            type="button"
            className="w-full flex items-center justify-between px-4 py-3 text-sm font-medium hover:bg-gray-50 rounded-lg transition-colors"
            onClick={() => setShowPromptEditor(!showPromptEditor)}
          >
            <span>编辑 Prompt</span>
            <span className="text-gray-400">{showPromptEditor ? '▲' : '▼'}</span>
          </button>

          {showPromptEditor && (
            <div className="px-4 pb-4 flex flex-col gap-4 border-t pt-4">
              {/* 系统提示 */}
              <div className="flex flex-col gap-2">
                <Label htmlFor="system-prompt">系统提示（System Prompt）</Label>
                <Textarea
                  id="system-prompt"
                  className="min-h-[120px] text-xs font-mono"
                  value={systemPrompt}
                  onChange={(e) => setSystemPrompt(e.target.value)}
                />
                <p className="text-xs text-gray-400">
                  指示 LLM 如何生成总结的顶层指令
                </p>
              </div>

              {/* 用户提示模板 */}
              <div className="flex flex-col gap-2">
                <Label htmlFor="user-prompt-template">用户提示模板（User Prompt Template）</Label>
                <Textarea
                  id="user-prompt-template"
                  className="min-h-[80px] text-xs font-mono"
                  value={userPromptTemplate}
                  onChange={(e) => setUserPromptTemplate(e.target.value)}
                />
                <p className="text-xs text-gray-400">
                  使用 <code className="bg-gray-100 px-1 rounded">{'{subtitles}'}</code> 占位符插入字幕全文
                </p>
              </div>

              {/* 恢复默认按钮 */}
              <Button
                variant="outline"
                size="sm"
                className="self-start"
                onClick={handleResetPrompt}
              >
                恢复默认
              </Button>
            </div>
          )}
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
