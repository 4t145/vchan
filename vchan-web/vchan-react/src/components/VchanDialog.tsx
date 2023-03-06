import * as Dialog from "@radix-ui/react-dialog";
import { Cross1Icon } from "@radix-ui/react-icons";
import clsx from "clsx";
import { filter } from "lodash";
import { ReactNode, useState } from "react";
import { css, styled, theme } from "../../stitches.config";


const Trigger = css({
  background: 'Transparent',
  border:"None",
  height: "3em",
  width: "3em",
  borderRadius: '9999px',
  "&:hover": {
    color: theme.colors.vc_main,
    background: '#0001',
  },
  "&:active": {
    color: theme.colors.vc_main,
    background: '#fff8',
  }
});


const OverLay = css({
  position: "fixed",
  zIndex: "20",
  backgroundColor: "#0008",
  inset: 0,
});

const Content = css({
  position: "fixed",
  zIndex: "50",
  padding: 4,
  width: "fit-content",
  height: "fit-content",
  inset: 0,
  display: "flex",
  flexDirection: "column",
  backgroundColor: "$vc_surface",
  boxShadow: `4px 4px ${theme.colors.vc_main}`,
  top: "50%",
  left: "50%",
  translate: "-50% -50%",
});

const Close = css({
  position: "absolute",
  top: 3.5,
  right: 3.5,
  display: "inline-flex",
  justifyItems: "center",
  background: "transparent",
  borderStyle: "none",
  cursor: "pointer",
  padding: 1,
  "&:hover": {
    color: theme.colors.vc_main,
  },
});

const Title = css({
  margin: 0,
  background: "transparent",
  borderStyle: "none",
  fontSize: '1em',
  padding: 1,
});

const ContentBody = css({
  display: "flex",
  justifyItems: "center",
  flexDirection: "column"
});

const TitleBar = css({
  display: "inline-flex",
  justifyItems: "space-between",
});

export function VchanDialog(
  props: React.PropsWithChildren<{
    title: string;
    trigger: React.ReactNode;
  }>
) {
  const [open, setOpen] = useState(false);
  return (
    <Dialog.Root open={open} onOpenChange={setOpen}>
      <Dialog.Trigger className={Trigger()} onClick={() => setOpen(true)}>
        {props.trigger}
      </Dialog.Trigger>
      <Dialog.Portal>
        <Dialog.Overlay className={OverLay()} />
        <Dialog.Content className={Content()}>
          <div className={TitleBar()}>
            <Dialog.Title className={Title()}>
              {props.title}
            </Dialog.Title>
            <Dialog.Close className={Close()}>
              <Cross1Icon />
            </Dialog.Close>
          </div>
          <div className={ContentBody()}>{props.children}</div>
        </Dialog.Content>
      </Dialog.Portal>
    </Dialog.Root>
  );
}
