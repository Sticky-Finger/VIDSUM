import { useState } from 'react';
import { InputModeSelect } from './components/InputModeSelect';
import { FileSelector, SelectedFile } from './components/FileSelector';
import { ModelSelect } from './components/ModelSelect';
import { AsrProgress } from './components/AsrProgress';
import { Button } from '@/components/ui/button';
import { type SubtitleEntry } from './lib/subtitle-export';

/// 应用状态模式
type AppMode = 'select' | 'file' | 'confirm' | 'model_select' | 'transcribing' | 'preview';

function App() {
  const [currentMode, setCurrentMode] = useState<AppMode>('select');
  const [selectedInputMode, setSelectedInputMode] = useState<'media' | 'subtitle' | null>(null);
  const [selectedFile, setSelectedFile] = useState<SelectedFile | null>(null);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);
  const [errorMode, setErrorMode] = useState<AppMode | null>(null);
  const [confirmedSubtitle, setConfirmedSubtitle] = useState<SubtitleEntry[] | null>(null);

  /// 选择输入模式
  const handleSelectInputMode = (mode: 'media' | 'subtitle') => {
    setSelectedInputMode(mode);
    setCurrentMode('file');
  };

  /// 文件选择完成
  const handleFileSelected = (file: SelectedFile) => {
    setSelectedFile(file);
    setCurrentMode('confirm');
  };

  /// 返回上一级
  const handleBack = () => {
    setErrorMessage(null);
    setErrorMode(null);

    if (currentMode === 'file') {
      setCurrentMode('select');
      setSelectedInputMode(null);
    } else if (currentMode === 'confirm') {
      setCurrentMode('file');
      setSelectedFile(null);
    } else if (currentMode === 'model_select') {
      setCurrentMode('confirm');
    }
  };

  /// 确认文件，进入模型选择
  const handleConfirm = () => {
    if (!selectedFile) return;

    if (selectedFile.type === 'media') {
      setCurrentMode('model_select');
    } else {
      // 字幕文件暂未实现处理逻辑
      setErrorMessage('字幕文件处理功能开发中，请先选择音视频文件');
      setErrorMode('confirm');
    }
  };

  /// 模型初始化完成，自动开始转写
  const handleModelInitialized = () => {
    setCurrentMode('transcribing');
  };

  /// 处理错误
  const handleError = (message: string, fromMode: AppMode) => {
    setErrorMessage(message);
    setErrorMode(fromMode);
  };

  /// 模型选择阶段出错
  const handleModelError = (message: string) => {
    handleError(message, 'model_select');
  };

  /// 转写阶段出错
  const handleTranscriptionError = (message: string) => {
    handleError(message, 'transcribing');
  };

  /// 预览确认字幕，进入总结
  const handlePreviewConfirm = (entries: SubtitleEntry[]) => {
    setConfirmedSubtitle(entries);
    setCurrentMode('preview');
    alert('字幕已确认！后续将进入大模型总结功能。');
  };

  /// 重新开始
  const handleRestart = () => {
    setCurrentMode('select');
    setSelectedInputMode(null);
    setSelectedFile(null);
    setConfirmedSubtitle(null);
    setErrorMessage(null);
    setErrorMode(null);
  };

  /// 清除错误
  const handleDismissError = () => {
    setErrorMessage(null);
    if (errorMode) {
      setCurrentMode(errorMode);
      setErrorMode(null);
    }
  };

  return (
    <div className="min-h-screen flex flex-col items-center justify-center p-4">
      {/* 全局错误提示 */}
      {errorMessage && currentMode !== 'transcribing' && (
        <div className="mb-4 w-[400px] p-3 bg-red-50 border border-red-200 rounded-lg
                        flex items-start justify-between gap-2">
          <div className="flex-1 min-w-0">
            <p className="text-red-600 text-sm font-medium">出错了</p>
            <p className="text-red-500 text-xs mt-1 break-words">{errorMessage}</p>
          </div>
          <button
            className="text-red-400 hover:text-red-600 text-lg leading-none shrink-0"
            onClick={handleDismissError}
          >
            ×
          </button>
        </div>
      )}

      {/* 模式选择 */}
      {currentMode === 'select' && (
        <InputModeSelect onSelectMode={handleSelectInputMode} />
      )}

      {/* 文件选择 */}
      {currentMode === 'file' && selectedInputMode && (
        <FileSelector
          mode={selectedInputMode}
          onBack={handleBack}
          onFileSelected={handleFileSelected}
        />
      )}

      {/* 确认文件 */}
      {currentMode === 'confirm' && selectedFile && (
        <div className="w-[400px]">
          <div className="mb-4 p-4 border rounded-lg">
            <h3 className="text-lg font-semibold mb-2">已选择文件</h3>
            <div className="space-y-2 text-sm">
              <div>
                <span className="text-gray-500">名称：</span>
                {selectedFile.name}
              </div>
              <div>
                <span className="text-gray-500">类型：</span>
                {selectedFile.type === 'media' ? '音视频' : '字幕'}
              </div>
              <div className="truncate">
                <span className="text-gray-500">路径：</span>
                {selectedFile.path}
              </div>
            </div>
          </div>
          <div className="flex flex-col gap-3">
            <Button
              variant="default"
              className="w-full h-12 text-base"
              onClick={handleConfirm}
            >
              {selectedFile.type === 'media' ? '🎬 开始转写' : '📄 开始处理'}
            </Button>
            <Button
              variant="outline"
              className="w-full h-12 text-base"
              onClick={handleBack}
            >
              重新选择
            </Button>
            <Button
              variant="ghost"
              className="w-full h-12 text-base"
              onClick={handleRestart}
            >
              ← 返回主页
            </Button>
          </div>
        </div>
      )}

      {/* 模型选择 */}
      {currentMode === 'model_select' && (
        <ModelSelect
          onInitialized={handleModelInitialized}
          onBack={handleBack}
          onError={handleModelError}
        />
      )}

      {/* 转写进度 */}
      {currentMode === 'transcribing' && selectedFile && (
        <AsrProgress
          audioPath={selectedFile.path}
          audioName={selectedFile.name}
          onError={handleTranscriptionError}
          onRestart={handleRestart}
          onPreview={handlePreviewConfirm}
        />
      )}

      {/* 预览确认 - 字幕确认后占位 */}
      {currentMode === 'preview' && confirmedSubtitle && (
        <div className="w-[400px]">
          <div className="mb-4 p-4 border rounded-lg">
            <h3 className="text-lg font-semibold mb-2">字幕已确认</h3>
            <p className="text-sm text-gray-500">
              共 {confirmedSubtitle.length} 段字幕已确认，即将进入大模型总结。
            </p>
          </div>
          <Button
            variant="outline"
            className="w-full h-10 text-sm"
            onClick={handleRestart}
          >
            ← 返回主页
          </Button>
        </div>
      )}
    </div>
  );
}

export default App;
