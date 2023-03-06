import { Post, PostSend } from "../models";
import axios, { Axios, AxiosRequestConfig, Method } from "axios";

export type VchanResponse<T> =
  | {
      ok: true;
      data: T;
    }
  | {
      ok: false;
      err: VchanError;
    };

type ExtractEnum<T extends Record<string, any>> = {
  [Variant in keyof T]: {
    type: Variant;
    data: T[Variant];
  };
}[keyof T];

export type PermissionDenied = {
  group: Nullable<number>;
};

export type DatabaseUnreachble = {
  err: string;
};

export type TransactionError = string;

export type QueryError = string;

export type VchanErrorEnum = {
  PermissionDenied: PermissionDenied;
  DatabaseUnreachble: DatabaseUnreachble;
  TransactionError: TransactionError;
  QueryError: QueryError;
};

export type VchanError = ExtractEnum<VchanErrorEnum>;

export type VchanApiMethod<TReq, TResp> = TReq extends void
  ? () => Promise<VchanResponse<TResp>>
  : (req: TReq) => Promise<VchanResponse<TResp>>;

export type Pagination = {
  page: number;
  pagesize: number;
};

export type PaginationQuery<V> = Pagination & V;

export type PaginationView<T> = {
  items: T[];
  pagination: Pagination;
};

export type Board = {
  id: number;
  name: string;
};

export type Thread = {
  id: number;
  primary_post_id: number;
  update_time: string;
  board_id: number;
};

export type ThreadPageFilter = {
  list: Array<Board>;
  filter_mode: "WhiteList" | "BlackList";
};

export type ThreadPageView = {
  board_id: number;
};

export type PostPageView =
  | {
      type: "Thread";
      data: number;
    }
  | {
      type: "Poster";
      data: string;
    }
  | {
      type: "PosterOfThread";
      data: {
        poster_id: string;
        thread_id: number;
      };
    }
  | {
      type: "All";
    };

export type RegisterResponse = {
  cookie_token: string;
};

export type ApiDefine = {
  register: {
    req: void;
    resp: {
      cookie_token: string;
    };
  };
  get_threads: {
    req: PaginationQuery<ThreadPageView>;
    resp: PaginationView<Thread>;
  };
  create_board: {
    req: string;
    resp: { board_id: number };
  };
  get_board: {
    req: void;
    resp: Board[];
  };
  get_posts_batch: {
    req: Array<number>;
    resp: Array<Post>;
  };
  post_thread: {
    req: {
      post: PostSend;
      board_id: number;
    };
    resp: {
      primary_post_id: number;
      thread_id: number;
    };
  };
};

export type ApiMethods = {
  [M in keyof ApiDefine]: VchanApiMethod<
    ApiDefine[M]["req"],
    ApiDefine[M]["resp"]
  >;
};

class Api implements ApiMethods {
  static baseUrl: string = "http://localhost:80";
  static readonly _: Api = new Api();
  constructor() {
    axios.defaults.withCredentials = true
  }
  get baseUrl(): string {
    return Api.baseUrl
  }
  private path(path: string[]): string {
    return [Api.baseUrl, ...path].join("/");
  }
  register: () => Promise<VchanResponse<{ cookie_token: string }>> =
    async () => {
      return (await axios.post(Api._.path(["register"]))).data;
    };
  get_threads: (
    req: PaginationQuery<ThreadPageView>
  ) => Promise<VchanResponse<PaginationView<Thread>>> = async (req) => {
    return (
      await axios.get(Api._.path(["thread"]), {
        params: req,
      })
    ).data;
  };
  create_board: (
    board_name: string
  ) => Promise<VchanResponse<{ board_id: number }>> = async (board_name) => {
    return (await axios.post(Api._.path(["board", "create", board_name]))).data;
  };
  get_board: () => Promise<VchanResponse<Board[]>> = async () => {
    return (await axios.get(Api._.path(["board"]))).data;
  };
  get_posts_batch: (req: number[]) => Promise<VchanResponse<Post[]>> = async (
    req
  ) => {
    return (await axios.post(Api._.path(["post", "batch"]), req)).data;
  };
  post_thread: (req: {
    post: PostSend;
    board_id: number;
  }) => Promise<VchanResponse<{ primary_post_id: number; thread_id: number }>> =
    async (req) => {
      return (await axios.post(Api._.path(["thread"]), req)).data;
    };
}

export const api = Api._;
