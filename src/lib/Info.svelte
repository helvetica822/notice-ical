<script lang="ts">
	import { createEventDispatcher, onDestroy } from 'svelte';

	export let message = '';

	// アラートが非表示になるまでのミリ秒。
	const duration = 3000;

	let visible = false;
	let timeout: number;

	const dispatch = createEventDispatcher();

	$: {
		if (message) {
			visible = true;

			timeout = window.setTimeout(() => {
				visible = false;

				// alertEndを親に通知します。
				dispatch('alertEnd');
			}, duration);
		}
	}

	// コンポーネントが破棄されるタイミングでタイマーをクリアします。
	onDestroy(() => {
		clearTimeout(timeout);
	});
</script>

{#if visible}
	<div class="alert alert-info shadow-lg">
		<div>
			<span>{message}</span>
		</div>
	</div>
{/if}
