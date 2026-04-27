/// 字幕条目接口
export interface SubtitleEntry {
  index: number;
  text: string;
  startTime: number;   // 毫秒
  endTime: number;     // 毫秒
  timestamp: string;   // 格式化后的时间戳，如 "00:01:23"
}

/// 将毫秒转换为 SRT 时间格式 (HH:MM:SS,mmm)
function toSrtTime(ms: number): string {
  const totalSeconds = Math.floor(ms / 1000);
  const hours = Math.floor(totalSeconds / 3600);
  const minutes = Math.floor((totalSeconds % 3600) / 60);
  const seconds = totalSeconds % 60;
  const millis = ms % 1000;
  return `${String(hours).padStart(2, '0')}:${String(minutes).padStart(2, '0')}:${String(seconds).padStart(2, '0')},${String(millis).padStart(3, '0')}`;
}

/// 将毫秒转换为 VTT 时间格式 (HH:MM:SS.mmm)
function toVttTime(ms: number): string {
  const totalSeconds = Math.floor(ms / 1000);
  const hours = Math.floor(totalSeconds / 3600);
  const minutes = Math.floor((totalSeconds % 3600) / 60);
  const seconds = totalSeconds % 60;
  const millis = ms % 1000;
  return `${String(hours).padStart(2, '0')}:${String(minutes).padStart(2, '0')}:${String(seconds).padStart(2, '0')}.${String(millis).padStart(3, '0')}`;
}

/// 生成 SRT 格式字幕
export function generateSrt(entries: SubtitleEntry[]): string {
  return entries
    .map((entry) => {
      return `${entry.index}\n${toSrtTime(entry.startTime)} --> ${toSrtTime(entry.endTime)}\n${entry.text}\n`;
    })
    .join('\n');
}

/// 生成 VTT 格式字幕
export function generateVtt(entries: SubtitleEntry[]): string {
  const header = 'WEBVTT\n\n';
  const body = entries
    .map((entry) => {
      return `${toVttTime(entry.startTime)} --> ${toVttTime(entry.endTime)}\n${entry.text}\n`;
    })
    .join('\n');
  return header + body;
}

/// 下载文件
export function downloadFile(content: string, filename: string, mimeType: string): void {
  const blob = new Blob([content], { type: mimeType });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = filename;
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
  URL.revokeObjectURL(url);
}
