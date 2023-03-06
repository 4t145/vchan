import { UserId } from "./UserId";
import { Post } from "../models";
import { styled, theme, css } from "../../stitches.config";

export function Thread(props: { post: Post }) {
  const { post } = props;
  const { id, email, user_id, author, title, content } = post;
  return (
    <>
    </>
  );
}



const Card = css({
  background: '$vc_surface',
  color: 'black',
  borderRadius: 4,
  border: "1px solid transparent",
  padding: "1rem"
})

const Head = css({
  background: '$vc_dark',
  color: 'black',
  borderRadius: 4,
  border: "1px solid transparent",
  padding: 0,
  display: 'flex',
  justifyContent: 'flex-start',
  flexDirection:'row',
})

const PostLink = css({
  background: '$vc_surface',
  color: 'black',
  borderRadius: 4,
  border: "1px solid transparent",
  paddingInline: '0.5rem',
  flexGrow: 0
})

export function ThreadPreview(props: { post: Post }) {
  const { post } = props;
  const { id, email, user_id, author, title, content } = post;
  return (
    <>
      <div className={Head()}>
        <span className={PostLink()}>
          <a href="#" >
            <code>
              &gt;&gt;&gt;{String(id).padStart(10, "0")}
            </code>
          </a>
          &nbsp;
          <UserId user_id={user_id}/>
        </span>
      </div>
      <div className={Card()}>
        <article>
          <hgroup>
            <h1 className="">{title}</h1>
            <p className="">
              {author}
              {email ? `<${email}>` : null}
            </p>
          </hgroup>
          <p className="">{content}</p>
        </article>
      </div>
    </>
  );
}
