import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
const appWindow = getCurrentWebviewWindow()

/**
 * ウィンドウを非表示にします。
 */
export function hide() {
    appWindow.hide();
}
