<script lang="ts">
	import { icalUrl, noticeTiming } from './preferenceStore';
</script>

<div class="flex-grow">
	<div>
		<label class="input input-bordered input-accent flex items-center gap-2">
			iCal URL
			<input class="grow" id="icalUrl" placeholder="http://xxx ..." bind:value={$icalUrl} />
		</label>
	</div>

	<div class="gap-2 mt-4">
		通知タイミング
		<input
			type="range"
			min="0"
			max="60"
			bind:value={$noticeTiming}
			class="range range-sm mt-2"
			step="10"
		/>
		<div class="flex w-full justify-between px-2 text-xs">
			<span>1m</span>
			<span>3m</span>
			<span>5m</span>
			<span>10m</span>
			<span>15m</span>
			<span>30m</span>
			<span>1h</span>
		</div>
	</div>
</div>
