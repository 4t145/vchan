import invert, { BlackWhite } from "invert-color";

const regex = /[A-F|a-f|0-9]{9}/;
export function UserId(props: { user_id: string } & React.ScriptHTMLAttributes<HTMLSpanElement>) {
  const { user_id: id } = props;
  const result = parseId(id);

  return (

    <code {...props}>
      {typeof result == "string" ? (
        <span
          style={{
            background: "black",
            color: 'chartreuse'
          }}
        >
          {result}
        </span>
      ) : (
        result.map((seg, index) => (
          <span
            key={seg + index.toString()}
            style={{
              color: invert("#" + seg, true),
              background: "#" + seg,
            }}
          >
            {seg}
          </span>
        ))
      )}
    </code>
  );
}

function parseId(id: string): string | [string, string, string] {
  if (id.length == 9) {
    const isColorful = regex.test(id);
    if (isColorful) {
      const c0 = id.slice(0, 3);
      const c1 = id.slice(3, 6);
      const c2 = id.slice(6, 9);
      return [c0, c1, c2];
    }
  }
  return id;
}
