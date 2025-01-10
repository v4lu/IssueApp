<script lang="ts">
	import '@fontsource-variable/inter';
	import { ModeWatcher } from 'mode-watcher';
	import { fade } from 'svelte/transition';
	import '../app.css';
	import { toast } from '../lib/stores/toast.store';
	import { Toast } from '$lib/components/ui/toast';

	let { children } = $props();
</script>

<div class="fixed right-4 top-4 z-[10000] flex flex-col gap-2">
	{#each $toast as t (t.id)}
		<div transition:fade={{ duration: 300 }}>
			<Toast message={t.message} type={t.type} close={() => toast.remove(t.id)} />
		</div>
	{/each}
</div>
<ModeWatcher />
{@render children()}
