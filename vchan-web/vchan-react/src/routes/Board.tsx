import { ThreadPreview } from "../components/Thread";
import { CookieIcon } from "@radix-ui/react-icons";
import * as Label from "@radix-ui/react-label";
import { VcrButton, VrcPost, PostEditor } from "../components";
import { css } from "../../stitches.config";
import { useParams } from "react-router-dom";
import { getBoards } from "../memory";
import { LoadingWebData, Post, useLoadingWebData } from "../models";
import { api, PaginationView, Thread } from "../webapi";

const PageList = css({
  listStyle: "none inside",
});

const PageListItem = css({
  marginTop: "1rem",
});

export function Boards() {
  const { board_name } = useParams();
  const posts = useLoadingWebData(async () => {
    const boards = await getBoards();
    const board = boards.find((b) => b.name === board_name)!;
    const board_id = board.id;
    const result = await api.get_threads({
      board_id,
      page: 1,
      pagesize: 10,
    });
    if (!result.ok) return result
    const threads = result.data.items;
    const idList = threads.map(th => th.primary_post_id)
    return api.get_posts_batch(idList)
  }, [board_name]);

  return (
    <div className="">
      {board_name}
      <PostEditor
        onSubmit={
          async (post) => {
            const boards = await getBoards();
            const board = boards.find((b) => b.name === board_name)!;
            const board_id = board.id;
            const result = await api.post_thread({
              post,
              board_id
            })
            if (result.ok) {
              console.log(result.data)
              // 添加发表的post
            } else {
              console.log(result.err)
            }
          }
        }
      />

      <section className={PageList()} role="list">
        <ThreadsSegment posts={posts}></ThreadsSegment>
      </section>
    </div>
  );
}

export function ThreadsSegment(props: {
  posts: LoadingWebData<Post[]>;
}) {
  if (props.posts.loading) {
    return <li></li>;
  } else {
    if (props.posts.success && props.posts.result.ok) {
      const posts = props.posts.result.data;
      return (
        <>
          {posts.map((post) => (
            <li key={post.id}>
              <ThreadPreview
                post={post}
              />
            </li>
          ))}
        </>
      );
    } else {
      return <>
        加载失败
      </>
    }
  }
}
