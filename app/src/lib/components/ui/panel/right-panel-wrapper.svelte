<script lang="ts">
	import type { Snippet } from 'svelte';

	type Props = {
		onClose: () => void;
		isOpen: boolean;
		children: Snippet;
	};

	let { isOpen, onClose, children }: Props = $props();

	function clickOutside(node: HTMLElement) {
		function handleClick(event: MouseEvent) {
			if (node && !node.contains(event.target as Node) && !event.defaultPrevented) {
				onClose();
			}
		}

		document.addEventListener('click', handleClick, true);

		return {
			destroy() {
				document.removeEventListener('click', handleClick, true);
			}
		};
	}
</script>

<div
	use:clickOutside
	class="fixed right-0 top-0 z-[10] h-dvh w-[40rem] overflow-hidden overflow-y-auto border-l border-border bg-background shadow-lg transition-transform duration-300 ease-out"
	class:translate-x-full={!isOpen}
	class:translate-x-0={isOpen}
>
	{@render children()}
</div>
