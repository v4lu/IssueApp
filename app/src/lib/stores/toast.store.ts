import { writable } from 'svelte/store';

type ToastType = 'success' | 'error' | 'info';

type Toast = {
	id: number;
	message: string;
	type: ToastType;
	timeoutId: number;
};

type ToastStore = {
	subscribe: (this: void, run: (value: Toast[]) => void, invalidate?: () => void) => () => void;
	success: (message: string, duration?: number) => void;
	error: (message: string, duration?: number) => void;
	info: (message: string, duration?: number) => void;
	remove: (id: number) => void;
};

function createToastStore(): ToastStore {
	const { subscribe, update } = writable<Toast[]>([]);
	let id = 0;

	function addToast(message: string, type: ToastType, duration: number = 5000): void {
		const toastId = id++;
		const timeoutId = window.setTimeout(() => {
			removeToast(toastId);
		}, duration);
		const toast: Toast = { id: toastId, message, type, timeoutId };
		update((toasts) => [...toasts, toast]);
	}

	function removeToast(id: number): void {
		update((toasts) => {
			const toastToRemove = toasts.find((t) => t.id === id);
			if (toastToRemove) {
				clearTimeout(toastToRemove.timeoutId);
			}
			return toasts.filter((t) => t.id !== id);
		});
	}

	function createToastFunction(type: ToastType) {
		return (message: string, duration?: number): void => addToast(message, type, duration);
	}

	return {
		subscribe,
		success: createToastFunction('success'),
		error: createToastFunction('error'),
		info: createToastFunction('info'),
		remove: removeToast
	};
}

export const toast = createToastStore();
