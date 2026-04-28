import { useMemo } from 'react';
import { marked } from 'marked';
import { Button } from '@/components/ui/button';
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';

/// 组件 Props
interface SummaryResultProps {
  /// LLM 生成的 Markdown 总结文本
  summary: string;
  /// 返回 LLM 配置页的回调
  onBack: () => void;
  /// 导出 .md 文件到本地的回调
  onExport: () => void;
}

export function SummaryResult({ summary, onBack, onExport }: SummaryResultProps) {
  /// 将 Markdown 渲染为 HTML
  const htmlContent = useMemo(() => {
    try {
      return marked.parse(summary, { breaks: true });
    } catch (e) {
      console.error('Markdown 渲染失败:', e);
      return `<p>Markdown 渲染失败：${String(e)}</p>`;
    }
  }, [summary]);

  return (
    <Card className="w-[720px] max-h-[80vh] flex flex-col">
      <CardHeader>
        <CardTitle>视频总结</CardTitle>
        <CardDescription>
          基于字幕内容生成的带时间戳 Markdown 总结
        </CardDescription>
      </CardHeader>

      <CardContent className="flex flex-col gap-4 flex-1 min-h-0">
        {/* Markdown 展示区域 */}
        <div className="flex-1 overflow-y-auto border rounded-lg p-4 bg-white">
          <div
            className="prose prose-sm max-w-none"
            dangerouslySetInnerHTML={{ __html: htmlContent }}
          />
        </div>

        {/* 按钮区域 */}
        <div className="flex gap-3">
          <Button
            variant="default"
            className="flex-1 h-12 text-base"
            onClick={onExport}
          >
            导出 .md 文件
          </Button>
          <Button
            variant="outline"
            className="flex-1 h-12 text-base"
            onClick={onBack}
          >
            返回配置
          </Button>
        </div>
      </CardContent>
    </Card>
  );
}
