<script lang="ts">
	import { Footer, Header } from '$lib/components/layout';
	import '../app.css';
	let { children } = $props();
</script>

<Header />
{@render children()}
<Footer />
