import { Post, PostSend } from "../../models";
import MDEditor from "@uiw/react-md-editor";
import React from "react";
import rehypeSanitize from "rehype-sanitize";
import { css, theme } from "../../../stitches.config";
import { VcrButton } from "../VcrButton";
import { VrcSwitch } from "../VrcSwitch";
const MarkdownView = css({
  backgroundColor: "$vc_surface",
});

const Form = css({
  display: "flex",
  flexDirection: "column",
  justifyContent: "stretch",
});

const FormRow = css({
  display: "flex",
  flexDirection: "row",
});

const FormCol = css({
  display: "flex",
  flexDirection: "column",
});

const Input = css({
  flexGrow: 1,
  resize: "vertical",
  width: "100%",
  borderRadius: "4px",
  minHeight: "1.5em",
  border: "1px solid transparent",
  backgroundColor: "$vc_surface",
  borderColor: theme.colors.vc_main,
  borderStyle: "none",
  outline: "none",
  margin: "4px",
});

function checkedContent(content: string | undefined) {
  if (content && content.length > 0 && content.length < 4096) {
    return true;
  }
  return false;
}

enum Flags {
  Main = 1 << 0,
  RichText = 1 << 1
}
function getFlag(markdown: boolean): number {
  let flag = Flags.Main
  if (markdown) {
    flag ||= Flags.RichText
  }
  return flag
}
export function PostEditor(props: {
  onSubmit: (p: PostSend) => Promise<void>;
}) {
  const [advancedMode, setAdvancedMode] = React.useState<boolean>(false);
  const [title, setTitle] = React.useState<string>("");
  const [author, setAuthor] = React.useState<string>("");
  const [email, setEmail] = React.useState<string>("");
  const [value, setValue] = React.useState<string | undefined>(undefined);
  const button = (
    <VcrButton
      fill
      type="button"
      onClick={async () => {
        if (checkedContent(value)) {
          await props.onSubmit({
            flag: getFlag(advancedMode),
            thread_id: 0,
            thread_path: [],
            content: value!,
            title,
            email,
            author,
          });

        }
      }}
    >
      提交
    </VcrButton>
  );
  return (
    <>
      <div>
        <VrcSwitch
          label="复杂模式"
          checked={advancedMode}
          onCheckedChange={setAdvancedMode}
        ></VrcSwitch>
        <form className={Form()}>
          {advancedMode ? (
            <>
              <MDEditor
                value={value}
                onChange={setValue}
                className={Input()}
                previewOptions={{ rehypePlugins: [[rehypeSanitize]] }}
                preview="edit"
                extraCommands={[]}
              />
              <div
                className={css({
                  display: "flex",
                })()}
              >
                <div
                  className={FormCol({
                    css: {
                      flexGrow: 1,
                    },
                  })}
                >
                  <div className={FormRow()}>
                    <input
                      placeholder="标题"
                      type="text"
                      aria-label="title"
                      value={title}
                      onChange={(e) => setTitle(e.target.value)}
                      className={Input()}
                    ></input>
                  </div>
                  <div className={FormRow()}>
                    <input
                      placeholder="作者"
                      type="text"
                      aria-label="author"
                      value={author}
                      onChange={(e) => setAuthor(e.target.value)}
                      className={Input()}
                    ></input>
                    <input
                      placeholder="电邮"
                      type="email"
                      aria-label="email"
                      value={email}
                      onChange={(e) => setEmail(e.target.value)}
                      className={Input()}
                    ></input>
                  </div>
                </div>
                <div
                  className={FormCol({
                    css: {
                      height: "auto",
                      flexGrow: 0,
                      justifyContent: "center",
                    },
                  })}
                >
                  {button}
                </div>
              </div>
            </>
          ) : (
            <div
              className={css({
                display: "flex",
              })()}
            >
              <div
                className={FormCol({
                  css: {
                    height: "auto",
                    flexGrow: 1,
                    justifyContent: "center",
                  },
                })}
              ></div>
              <textarea
                aria-label="content"
                placeholder="在此输入..."
                className={Input()}
                value={value}
                onChange={(e) => {
                  setValue(e.target.value);
                }}
              ></textarea>
              <div
                className={FormCol({
                  css: {
                    height: "auto",
                    flexGrow: 0,
                    flexShrink: 0,
                    justifyContent: "center",
                  },
                })}
              >
                {button}
              </div>
            </div>
          )}
        </form>
        {}
        <MDEditor.Markdown
          source={value}
          className={MarkdownView()}
          rehypePlugins={[[rehypeSanitize]]}
        />
      </div>
    </>
  );
}
