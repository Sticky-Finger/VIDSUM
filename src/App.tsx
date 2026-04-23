import { useState } from 'react';
import { InputModeSelect } from './components/InputModeSelect';

function App() {
  const [selectedMode, setSelectedMode] = useState<'media' | 'subtitle' | null>(null);

  const handleSelectMode = (mode: 'media' | 'subtitle') => {
    setSelectedMode(mode);
    console.log('选择的模式:', mode);
    // TODO: 下一步处理
  };

  return (
    <div className="min-h-screen flex items-center justify-center p-4">
      <InputModeSelect onSelectMode={handleSelectMode} />
    </div>
  );
}

export default App;
