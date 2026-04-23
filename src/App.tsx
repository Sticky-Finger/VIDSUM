import { useState } from 'react';
import { InputModeSelect } from './components/InputModeSelect';
import { FileSelector, SelectedFile } from './components/FileSelector';

type AppMode = 'select' | 'file' | 'confirm' | 'result';

function App() {
  const [currentMode, setCurrentMode] = useState<AppMode>('select');
  const [selectedInputMode, setSelectedInputMode] = useState<'media' | 'subtitle' | null>(null);
  const [selectedFile, setSelectedFile] = useState<SelectedFile | null>(null);

  // 选择输入模式（音视频转写 / 已有字幕）
  const handleSelectInputMode = (mode: 'media' | 'subtitle') => {
    setSelectedInputMode(mode);
    setCurrentMode('file');
  };

  // 文件选择完成
  const handleFileSelected = (file: SelectedFile) => {
    setSelectedFile(file);
    setCurrentMode('confirm');
  };

  // 返回模式选择
  const handleBack = () => {
    if (currentMode === 'file') {
      setCurrentMode('select');
      setSelectedInputMode(null);
    } else if (currentMode === 'confirm') {
      setCurrentMode('file');
      setSelectedFile(null);
    }
  };

  // 确认文件，开始处理
  const handleConfirm = () => {
    // TODO: 跳转到处理页面
    console.log('确认处理文件:', selectedFile);
    setCurrentMode('result');
  };

  // 重新开始
  const handleRestart = () => {
    setCurrentMode('select');
    setSelectedInputMode(null);
    setSelectedFile(null);
  };

  return (
    <div className="min-h-screen flex items-center justify-center p-4">
      {currentMode === 'select' && (
        <InputModeSelect onSelectMode={handleSelectInputMode} />
      )}

      {currentMode === 'file' && selectedInputMode && (
        <FileSelector
          mode={selectedInputMode}
          onBack={handleBack}
          onFileSelected={handleFileSelected}
        />
      )}

      {currentMode === 'confirm' && selectedFile && (
        <div className="w-[400px] p-4">
          <div className="mb-4 p-4 border rounded-lg">
            <h3 className="text-lg font-semibold mb-2">已选择文件</h3>
            <div className="space-y-2 text-sm">
              <div><span className="text-gray-500">名称:</span> {selectedFile.name}</div>
              <div><span className="text-gray-500">类型:</span> {selectedFile.type === 'media' ? '音视频' : '字幕'}</div>
              <div><span className="text-gray-500">路径:</span> {selectedFile.path}</div>
            </div>
          </div>
          <div className="flex flex-col gap-3">
            <button
              className="w-full px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
              onClick={handleConfirm}
            >
              确认并开始处理
            </button>
            <button
              className="w-full px-4 py-2 border rounded hover:bg-gray-100"
              onClick={handleBack}
            >
              重新选择
            </button>
            <button
              className="w-full px-4 py-2 text-gray-600 hover:text-gray-800"
              onClick={handleRestart}
            >
              ← 返回主页
            </button>
          </div>
        </div>
      )}

      {currentMode === 'result' && (
        <div className="w-[400px] p-4 text-center">
          <h3 className="text-lg font-semibold mb-2">准备处理</h3>
          <p className="text-gray-500 mb-4">文件：{selectedFile?.name}</p>
          <p className="text-sm text-gray-400">
            （后续功能开发中...）
          </p>
          <button
            className="mt-4 px-4 py-2 text-blue-500 hover:text-blue-600"
            onClick={handleRestart}
          >
            返回主页
          </button>
        </div>
      )}
    </div>
  );
}

export default App;
