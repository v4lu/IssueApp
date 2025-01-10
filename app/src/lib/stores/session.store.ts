import { writable } from 'svelte/store';
import type { User, UserPreference } from '$lib/types/user.type';

type UserStore = {
	subscribe: (this: void, run: (value: User) => void, invalidate?: () => void) => () => void;
	setUser: (user: User) => void;
	updateUser: (partialUser: Partial<User>) => void;
};

type UserPreferenceStore = {
	subscribe: (
		this: void,
		run: (value: UserPreference) => void,
		invalidate?: () => void
	) => () => void;
	setUserPreference: (userPreference: UserPreference) => void;
	updateUserPreference: (partialUserPreference: Partial<UserPreference>) => void;
};

function createSessionStore(): UserStore {
	const { subscribe, set, update } = writable<User>();

	return {
		subscribe,
		setUser: (user: User) => set(user),
		updateUser: (partialUser: Partial<User>): void => {
			update((user) => {
				if (user) {
					return { ...user, ...partialUser };
				}
				return user;
			});
		}
	};
}

function createUserPreferenceStore(): UserPreferenceStore {
	const { subscribe, set, update } = writable<UserPreference>();

	return {
		subscribe,
		setUserPreference: (userPreference: UserPreference) => set(userPreference),
		updateUserPreference: (partialUserPreference: Partial<UserPreference>): void => {
			update((userPreference) => {
				if (userPreference) {
					return { ...userPreference, ...partialUserPreference };
				}
				return userPreference;
			});
		}
	};
}

export const session = createSessionStore();
export const userPreference = createUserPreferenceStore();
