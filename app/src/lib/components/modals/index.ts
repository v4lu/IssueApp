import type { Component } from 'svelte';
import { type IconProps, Icons } from '../icons';

export { default as CreateOrg } from './create-org.svelte';
export { default as CreateIssue } from './create-issue.svelte';
export { default as DeleteModal } from './delete-modal.svelte';
export { default as InviteUserOrg } from './invite-user-org.svelte';

type IconsType = typeof Icons;
export type PriorityIconName = keyof IconsType['priority'];
export type StatusIconName = keyof IconsType['status'];

export type PriorityArrType = {
	IconName: PriorityIconName;
	Icon: Component<IconProps>;
};

export type StatusArrType = {
	IconName: StatusIconName;
	Icon: Component<IconProps>;
};

export const priorityArr: PriorityArrType[] = [
	{ IconName: 'Low', Icon: Icons.priority.Low },
	{ IconName: 'Medium', Icon: Icons.priority.Medium },
	{ IconName: 'High', Icon: Icons.priority.High },
	{ IconName: 'Urgent', Icon: Icons.priority.Urgent }
];

export const statusArr: StatusArrType[] = [
	{ IconName: 'Backlog', Icon: Icons.status.Backlog },
	{ IconName: 'Todo', Icon: Icons.status.Todo },
	{ IconName: 'InProgress', Icon: Icons.status.InProgress },
	{ IconName: 'Done', Icon: Icons.status.Done },
	{ IconName: 'Canceled', Icon: Icons.status.Canceled },
	{ IconName: 'Blocked', Icon: Icons.status.Blocked }
];
