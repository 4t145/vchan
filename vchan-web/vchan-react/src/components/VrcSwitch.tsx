import * as Label from "@radix-ui/react-label";
import * as Switch from "@radix-ui/react-switch";
import { css, theme } from "../../stitches.config";

const Thumb = css({
  display: "Block",
  width: "21px",
  height: "100%",
  backgroundColor: "$vc_main",

  // boxShadow: `4px 4px ${theme.colors.vc_main}`,
  borderRadius: "4px",
  transition: "transform 100ms",
  transform: "translateX(2px)",
  willChange: "transform",

  "&[data-state=checked]": {
    transform: "translateX(19px)",
    backgroundColor: "$vc_surface",
  },
});

const Root = css({
  border: 0,
//   padding: 0,
  width: "42px",
  height: "1.5em",
  padding: '1px',
  backgroundColor: "$vc_surface",
  borderRadius: "4px",
  position: "relative",
  "&:focus": {},
  "&[data-state=checked]": {
    backgroundColor: "$vc_main",
  },
});

export function VrcSwitch(props: {
  label: string;
  checked: boolean;
  onCheckedChange: (_: boolean) => void;
}) {
  return (
    <>
      <div
        className={css({
          display: "inline-flex",
          fontSize: '0.9em'
        })()}
      >
        <Label.Root
          className={css({
            marginRight: "4px",
          })()}
        >
          {props.label}
        </Label.Root>
        <Switch.Root className={Root()} checked={props.checked} onCheckedChange={props.onCheckedChange}>
          <Switch.Thumb className={Thumb()} />
        </Switch.Root>
      </div>
    </>
  );
}
