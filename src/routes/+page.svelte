<script lang="ts">
	import { X, StarHalf } from 'lucide-svelte';
	import { invoke } from "@tauri-apps/api/core";
	import { onMount } from 'svelte';
	import PreferenceItems from '../lib/PreferenceItems.svelte';
	import Alert from '../lib/Alert.svelte';
	import Info from '../lib/Info.svelte';
	import { hide } from '../lib/preferenceWindow';
	import { icalUrl, noticeTiming } from '../lib/preferenceStore';
	import { alertErrorMessage, alertInfoMessage } from '../lib/messageStore';
	import { showNotification } from '$lib/notification';
	import { listen } from '@tauri-apps/api/event';

	/**
	 * 初期化パラメータ型。
	 * @property ical_url iCal URL
	 * @property notice_timing 通知タイミング
	 */
	type InitialParams = {
		ical_url: string;
		notice_timing: number;
	};

	type ToastNotice = {
		title: string;
		body: string;
	};

	let params: InitialParams;
	let isDisabled = false;

	// コンポーネントがマウントされるタイミングで初期化します。
	onMount(async () => {
		try {
			params = await invoke('get_initial_params');

			icalUrl.set(params.ical_url);
			noticeTiming.set(params.notice_timing.toString());
		} catch (error) {
			console.error('Error fetching initial parameters:', error);
		}
	});

	const handleAlertEnd = () => {
		isDisabled = false;
	};

	/**
	 * URL文字列が不正な値でないか検証します。
	 * @param url URL文字列
	 * @returns 検証結果(正常な場合はtrue。それ以外の場合はfalse), エラーメッセージのオブジェクト
	 */
	const isValidUrl = (url: string): { isValid: boolean; errorMessage: string } => {
		let errorMessage = '';

		if (url) {
			if (!URL.canParse(url)) errorMessage = 'iCal URLが不正な値です。';
		} else {
			errorMessage = 'iCal URLが入力されていません。';
		}

		return { isValid: errorMessage === '', errorMessage: errorMessage };
	};

	/**
	 * 設定を検証します。
	 */
	const testConfig = async () => {
		await saveOrValidConfig2Backend(false);
	};

	/**
	 * 設定を保存します。
	 */
	const saveConfig = async () => {
		await saveOrValidConfig2Backend(true);
	};

	/**
	 * バックエンド側からのイベントを検知します。
	 */
	listen('event-listen', (event) => {
		const notice = event.payload as ToastNotice;
		showNotification(notice.title, notice.body);
	});

	/**
	 * バックエンド側で設定を保存または検証します。
	 * @param save バックエンド側で設定を保存する場合はtrue。検証のみする場合はfalse。
	 */
	const saveOrValidConfig2Backend = async (save: boolean) => {
		const url = $icalUrl;

		const { isValid, errorMessage } = isValidUrl(url);

		alertErrorMessage.set('');
		alertInfoMessage.set('');
		isDisabled = false;

		if (isValid) {
			try {
				const n = $noticeTiming;
				isDisabled = true;

				// Tauriのコマンドを呼び出す
				const response = await invoke('send_ical_url', {
					url,
					noticeTiming: n.toString(),
					saveConfig: save
				});

				if (save) {
					isDisabled = false;
					hide();
				} else {
					setTimeout(() => {
						isDisabled = true;
						alertInfoMessage.set(response as string);
					}, 0);
				}
			} catch (error) {
				// 若干ずらさないとアラートにエラーメッセージが反映されないため0秒待機のsetTimeout。
				setTimeout(() => {
					isDisabled = true;
					alertErrorMessage.set(error as string);
				}, 0);
			}
		} else {
			// 若干ずらさないとアラートにエラーメッセージが反映されないため0秒待機のsetTimeout。
			setTimeout(() => {
				isDisabled = true;
				alertErrorMessage.set(errorMessage);
			}, 0);
		}
	};
</script>

<div class="h-screen flex flex-col">
	<header class="bg-zinc-800 h-12 select-none bg-fixe">
		<nav
			data-tauri-drag-region
			class="mx-auto flex items-center justify-between"
			aria-label="Global"
		>
			<div class="mx-1 p-3">
				<StarHalf class="square-4" />
			</div>
			<div class="mx-1 p-3">設定</div>
			<ul class="flex gap-x-1">
				<li>
					<button on:click={hide} class="window-control-button">
						<X size="16" />
					</button>
				</li>
			</ul>
		</nav>
	</header>

	<div class="bg-zinc-900 h-[1px]"></div>

	<main class="bg-zinc-800 p-4 flex flex-col flex-grow">
		<PreferenceItems />
		<Alert message={$alertErrorMessage} on:alertEnd={handleAlertEnd} />
		<Info message={$alertInfoMessage} on:alertEnd={handleAlertEnd} />

		<div class="flex justify-between items-center mt-4">
			<div>
				<button
					class="btn btn-primary btn-outline px-16"
					on:click={testConfig}
					disabled={isDisabled}>iCal URL テスト</button
				>
			</div>
			<div class="flex gap-2">
				<button
					class="btn btn-primary btn-outline px-16"
					on:click={saveConfig}
					disabled={isDisabled}>OK</button
				>
				<button class="btn btn-primary btn-outline px-16" on:click={hide}>キャンセル</button>
			</div>
		</div>
	</main>
</div>
