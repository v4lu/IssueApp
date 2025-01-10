import { HTTPError } from 'ky';
import { authAPI } from '$lib/api';
import type { Icons } from '$lib/components/icons';
import type { TErrorResponse } from '$lib/types/error.type';
import type { IssueRequest, IssueResponse, IssueUpdate } from '$lib/types/issue.type';
import type { PriorityIconName } from '$lib/components/modals';
import type { CommentRequest, CommentResponse } from '$lib/types/comment.type';

type IconsType = typeof Icons;
export type StatusIconName = keyof IconsType['status'];
export type GroupedIssues = Record<StatusIconName, IssueResponse[]>;

export const statusOrder: StatusIconName[] = [
	'Backlog',
	'Todo',
	'InProgress',
	'InReview',
	'Done',
	'Canceled',
	'Blocked'
];

export const priorityOrder: PriorityIconName[] = ['Low', 'Medium', 'High', 'Urgent'];

class Issue {
	issues = $state<IssueResponse[]>([]);

	isCreatingIssue = $state(false);
	isUpdatingIssue = $state(false);
	isLoading = $state(false);
	isCreatingComment = $state(false);

	groupedIssues = $derived(() => {
		return this.issues.reduce((acc, i) => {
			const status = i.issue.status as StatusIconName;
			if (!acc[status]) {
				acc[status] = [];
			}
			acc[status].push(i);
			return acc;
		}, {} as GroupedIssues);
	});

	sortedStatusKeys = $derived(
		this.groupedIssues()
			? statusOrder.filter((status) => this.groupedIssues()[status]?.length > 0)
			: []
	);

	statusCount = $derived<Map<StatusIconName, number>>(
		Object.keys(this.groupedIssues()).reduce((acc, status) => {
			acc.set(status as StatusIconName, this.groupedIssues()[status as StatusIconName].length);
			return acc;
		}, new Map())
	);
}

export function useIssue(authToken: string, orgId: string) {
	const resp = new Issue();
	const api = authAPI(authToken);

	async function loadIssues() {
		resp.isLoading = true;
		try {
			const response = await api.get<IssueResponse[]>(`issues/${orgId}`).json();
			resp.issues = response;
		} catch (error) {
			if (error instanceof HTTPError) {
				const res = (await error.response.json()) as TErrorResponse;
				console.error(res);
			}
		}
		resp.isLoading = false;
	}

	async function getIssue(issueId: string) {
		return resp.issues.find((i) => i.issue.id === issueId);
	}

	async function createIssue(issue: IssueRequest) {
		resp.isCreatingIssue = true;
		try {
			const newIssue = await api.post(`issues/${orgId}`, { json: issue }).json<IssueResponse>();
			resp.issues = [...resp.issues, newIssue];
		} catch (error) {
			if (error instanceof HTTPError) {
				const res = (await error.response.json()) as TErrorResponse;
				console.error(res);
			}
		}
		resp.isCreatingIssue = false;
	}

	async function createSubIssue(issue: IssueRequest) {
		resp.isCreatingIssue = true;
		try {
			const newIssue = await api.post(`issues/${orgId}`, { json: issue }).json<IssueResponse>();
			const parentIssue = resp.issues.find((i) => i.issue.id === issue.parent_id);
			if (parentIssue) {
				parentIssue.sub_issues = [...(parentIssue.sub_issues || []), newIssue.issue];
				resp.issues = resp.issues.map((i) => {
					if (i.issue.id === issue.parent_id) {
						return parentIssue;
					}
					return i;
				});
			}
		} catch (error) {
			if (error instanceof HTTPError) {
				const res = (await error.response.json()) as TErrorResponse;
				console.error(res);
			}
		}
		resp.isCreatingIssue = false;
	}

	async function updateIssue(issueId: string, update: IssueUpdate) {
		resp.isUpdatingIssue = true;
		try {
			const updatedIssue = await api
				.patch(`issues/${orgId}/${issueId}`, { json: update })
				.json<IssueResponse>();
			resp.issues = resp.issues.map((i) => {
				if (i.issue.id === issueId) {
					return updatedIssue;
				}
				return i;
			});
		} catch (error) {
			if (error instanceof HTTPError) {
				const res = (await error.response.json()) as TErrorResponse;
				console.error(res);
			}
		}
		resp.isUpdatingIssue = false;
	}

	async function deleteIssue(issueId: string) {
		try {
			await api.delete(`issues/${orgId}/${issueId}`).json();
			resp.issues = resp.issues.filter((i) => i.issue.id !== issueId);
		} catch (error) {
			if (error instanceof HTTPError) {
				const res = (await error.response.json()) as TErrorResponse;
				console.error(res);
			}
		}
	}

	async function createComment(payload: CommentRequest) {
		resp.isCreatingComment = true;
		try {
			const res = await api.post(`comments/${orgId}`, { json: payload }).json<CommentResponse>();
			const issue = resp.issues.find((i) => i.issue.id === payload.comment_owner_id);

			if (issue) {
				issue.comments = [...(issue.comments || []), res];
				resp.issues = resp.issues.map((i) => {
					if (i.issue.id === payload.comment_owner_id) {
						return issue;
					}
					return i;
				});
			}
		} catch (error) {
			if (error instanceof HTTPError) {
				const res = (await error.response.json()) as TErrorResponse;
				console.error(res);
			}
		} finally {
			resp.isCreatingComment = false;
		}
	}

	loadIssues();

	return {
		resp,
		createIssue,
		createSubIssue,
		updateIssue,
		deleteIssue,
		getIssue,
		createComment
	};
}
