<script lang="ts">
	import { Contact, Faq, Features, Hero, Pricing, Process } from '$lib/components/sections/home';
</script>

<div class="absolute inset-0 -z-10">
	<div
		class="absolute inset-0 bg-gradient-to-br from-primary/20 via-background to-transparent opacity-50"
	></div>

	<div class="absolute inset-0 flex flex-col">
		{#each Array(14) as _, i}
			<div
				class="flex-1 animate-pulse border-b border-border/[0.15] backdrop-blur-[4px]"
				style="animation-delay: {i * 0.1}s; opacity: {1 - i * 0.07}"
			></div>
		{/each}
	</div>
	<div class="absolute inset-0 flex flex-row">
		{#each Array(14) as _, i}
			<div
				class="flex-1 animate-pulse border-r border-border/[0.2] backdrop-blur-[1px]"
				style="animation-delay: {i * 0.1}s; opacity: {1 - i * 0.07}"
			></div>
		{/each}
	</div>
</div>
<Hero />
<Features />
<Process />
<Pricing />
<Faq />
<Contact />
