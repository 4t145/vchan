import { UserId } from "./UserId";
import { Post } from "@/models/post";
import { styled, theme, css } from "../../stitches.config";

export function VrcPost(props: {
    post: Post
}) {
    const {post} = props
    const {
        id,
        email,
        user_id,
        author,
        title,
        content,
    } = post
  return (
    <>
    <div className="px-3 py-2 bg-surface shadow-main">
      <hgroup className="py-2 shadow-main">
        <p className="text-black text-base space-x-1">
          <a href="#">&gt;&gt;&gt;{String(id).padStart(10, '0')}</a>
          <span className="text-white bg-dark px-1">by</span>
          <UserId user_id={user_id}/>
        </p>
        <h1 className="text-black text-xl">{title}</h1>
        <p className="text-black text-base">{author}{email ? `<${email}>` : null}</p>
      </hgroup>
      <p className="py-2 shadow-main text-black">{
        content
      }</p>
    </div>
    </>
  );
}
