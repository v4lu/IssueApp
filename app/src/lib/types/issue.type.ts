import type { CommentResponse } from './comment.type';

export type IssuePriority = 'Urgent' | 'High' | 'Medium' | 'Low';
export type IssueStatus =
	| 'Backlog'
	| 'Todo'
	| 'InProgress'
	| 'InReview'
	| 'Done'
	| 'Canceled'
	| 'Blocked';

export type Issue = {
	id: string;
	org_id: string;
	creator_id: string;
	number: number;
	title: string;
	description: string;
	priority: IssuePriority;
	status: IssueStatus;
	parent_id: string | null;
	due_date: Date | null;
	created_at: string;
	updated_at: string;
};

export type IssueRequest = {
	title: string;
	description: string;
	priority: IssuePriority;
	status: IssueStatus;
	parent_id?: string;
	due_date: Date | null;
};

export type IssueUpdate = {
	title?: string;
	description?: string;
	priority?: IssuePriority;
	status?: IssueStatus;
	parent_id?: string;
	due_date?: Date | null;
	remove_due_date?: boolean;
};

export type IssueResponse = {
	issue: Issue;
	issue_id: string;
	sub_issues: Issue[] | null;
	comments: CommentResponse[] | null;
};
