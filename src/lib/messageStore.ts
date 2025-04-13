import { writable } from 'svelte/store';

// アラートに表示するエラーメッセージ。
export const alertErrorMessage = writable('');
export const alertInfoMessage = writable('');
