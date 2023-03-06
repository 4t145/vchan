export type Post = {
  id: number;
  title?: string;
  email?: string;
  author?: string;
  create_time: Date;
  thread_id: number;
  thread_path: number[];
  user_id: string;
  content: string;
};

export type PostSend = {
  thread_id: number,
  thread_path: number[],
  flag: number,
  title?: string;
  email?: string;
  author?: string;
  content: string;
};
