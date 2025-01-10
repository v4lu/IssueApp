import { writable } from 'svelte/store';

type ContextMenuState = {
	activeId: string | null;
	x: number;
	y: number;
};

export const contextMenuStore = writable<ContextMenuState>({
	activeId: null,
	x: 0,
	y: 0
});
